import logging

from essaymind.core.node import MindNode
from essaymind.fiber.fiber import FiberKeyValue

log = logging.getLogger("Front")

class Front(MindNode):
    '''
    STM - holds a data until it decays.
    '''
    def __init__(self, parent, name='front'):
        super().__init__(parent, name)
        
        self._on_data = FiberKeyValue()
        
        self.next = None
        
        self.key = None
        self.value_decay = 0
        
        self._decay = 1
        self._decay_threshold = 0.5 

    # build
    
    def on_data(self):
        return self._on_data
    
    def to(self, target):
        self.on_data().to(target)
        return self
        
    def decay(self, decay):
        assert 0 <= decay <= 1
        self._decay = decay
        return self
        
    def decay_threshold(self, value):
        assert 0 <= value <= 1
        self._decay_threshold = value
        return self
        
    # active
        
    def __call__(self, key, value, p):
        self.next = (key, value, p)
        
    def tick(self):
        self.value_decay *= self._decay
        
        if self.next:
            key, value, p = self.next
            self.next = None
            
            if not self.key or self.value_decay < self._decay_threshold:
                self.key = key
                self.value_decay = 1
        
        p = self.value_decay
        
        if self._decay_threshold <= p:
            value = 0
            self._on_data(self.key, value, p)