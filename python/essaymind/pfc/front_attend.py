import logging

from essaymind.core.node import MindNode

log = logging.getLogger("Attend")

class Attend(MindNode):
    '''
    Attention model where senses like olfactory quickly habituate and disappear
    as signals unless attended to by the front.
    '''
    def __init__(self, parent, name='attend'):
        super().__init__(parent, name)
        
        self.next = None
        
        self.key = None
        self.value_decay = 0
        
        self._decay = 1
        self._decay_threshold = 0.5 
        
        self.sense_map = dict()
        
        self.senses = []

    # build
    
    def choose(self, name, fiber, target):
        return AttendSense(self, name, fiber, target)
        
    def decay(self, decay):
        assert 0 <= decay <= 1
        self._decay = decay
        return self
        
    def decay_threshold(self, value):
        assert 0 <= value <= 1
        self._decay_threshold = value
        return self
    
    def add_sense(self, name, sense):
        self.sense_map[name] = sense
        
    def add_sense_key(self, sense, key, value, p):
        self.senses.append((sense, key, value, p))
        
    # active
        
    def __call__(self, key, value, p):
        self.next = (key, value, p)
        
    def tick(self):
        self.value_decay *= self._decay
        
        senses = self.senses
        self.senses = []
        
        for sense in senses:
            att_sense, key, value, p = sense
            
            if not self.key or self.value_decay < self._decay_threshold:
                self.key = key
                self.value_decay = 1
                self.target = att_sense
            elif self.key == key:
                self.value_decay = 1

        p = self.value_decay
        
        if self._decay_threshold <= p:
            value = 0
            self.target.target(self.key, value, p)
            
class AttendSense:
    def __init__(self, attend, name, fiber, target):
        self.name = name
        self.attend = attend
        self.target = target
        
        attend.add_sense(name, self)
        
        fiber.to(self)
        
    def __call__(self, key, value, p):
        self.attend.add_sense_key(self, key, value, p)
