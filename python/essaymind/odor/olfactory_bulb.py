from essaymind.core.node import MindNode
from essaymind.fiber.fiber import FiberKeyValue

import hashlib
import logging
import base64

log = logging.getLogger('Ob')

class OlfactoryBulb(MindNode):
    def __init__(self, parent, name='ob'):
        super().__init__(parent, name)
        
        self._on_odor = FiberKeyValue()
        
        self._odors = []
        self._odor_map = dict()
        
        self._last_item = None
        self._last_odor = None
        
        self._sustain = 1
        self._decay = 1  # decay = habituate
        self._decay_threshold = 0.5
        
        self._last_value = 0
        self._last_decay = 0
        self._last_sustain = 0
        
        self._is_symbol = True
        
    # build
    
    def when(self, fiber):
        fiber.to(self)
        
        return self
    
    def on_odor(self):
        return self._on_odor
    
    def to(self, target):
        self.on_odor().to(target)
    
    def choose(self, name):
        return ChoiceOutput(self, name)
    
    def sustain(self, value):
        assert 0 <= value <= 1
        self._sustain = value
    
    def decay(self, value):
        assert 0 <= value <= 1
        self._decay = value
    
    def decay_threshold(self, value):
        assert 0 <= value <= 1
        self._decay_threshold = value
        
    def symbol(self, is_symbol):
        self._is_symbol = is_symbol
        return self
    
    # action

    def odor(self, odor, p):
        self._odors.append(odor)
        
    def __call__(self, odor, p):
        self.odor(odor, p)
        
    def categorize(self, name):
        if not self._last_item:
            return
        
        choice = self[name]
        
        self._last_item.categorize(choice)
        
    def attend(self, key, value, p):
        item = self._last_item
        
        if item and item.key == key:
            self._last_decay = 1
        
    def tick(self):
        odors = self._odors
        self._odors = []
        
        for odor in odors:
            self.process_odor(odor)
            
    def process_odor(self, odor):
        self._odor = odor
        
        item = self._odor_map.get(odor.name)
        
        if not item:
            item = OdorItem(odor.name, self._is_symbol)
            self._odor_map[odor.name] = item
            
        if self._last_odor and odor.name == self._last_odor.name:
            self._last_decay *= self._decay
        else:
            self._last_decay = 1
            
        self._last_sustain = 1
            
        self._last_item = item
        self._last_odor = odor

        #p = self.last_decay * self.last_sustain
        #self._on_odor(item.key, odor.angle, p)
        #item.on_odor(odor.angle, p)
        
        p = 1
        self._on_odor(item.key, odor.angle, p)
        
    def tick_old(self):
        if len(self._odors) > 0:
            self.process_new_odor()
        else:
            self.process_old_odor()
            
        p = self._last_decay * self._last_sustain
        
        if self._last_item and self._decay_threshold <= p:
            self._on_odor(self._last_item.key, self._last_odor.angle, p)
            self._last_item.on_odor(self._last_odor.angle, p)
            
    def process_new_odor(self):
        odors = self._odors
        
        self._odors = []
        self._odor = odor = odors[0]
        
        item = self._odor_map.get(odor.name)
        
        if not item:
            item = OdorItem(odor.name, self._is_symbol)
            self._odor_map[odor.name] = item
            
        if self._last_odor and odor.name == self._last_odor.name:
            self._last_decay *= self._decay
        else:
            self._last_decay = 1
            
        self._last_sustain = 1
            
        self._last_item = item
        self._last_odor = odor

        #p = self.last_decay * self.last_sustain
        #self._on_odor(item.key, odor.angle, p)
        #item.on_odor(odor.angle, p)
            
    def process_old_odor(self):
        self._last_decay *= self._decay
        self._last_sustain *= self._sustain
        
class OdorItem:
    def __init__(self, name, is_symbol):
        self.name = name
        
        if not is_symbol:
            sha256 = hashlib.sha256(name.encode('utf-8'))
        
            self.key = base64.b32encode(sha256.digest())[0:8]
        else:
            self.key = f"ob({name})"
        
        self.choice_map = dict()
        self.choice_list = []
        
    def categorize(self, choice):
        if not self.choice_map.get(choice):
            item = ChoiceItem(self.name, choice)
            self.choice_map[choice] = item
            self.choice_list.append(item)
            
    def on_odor(self, angle, p):
        for item in self.choice_list:
            item._on_odor(item.key, angle, p)
        
class ChoiceItem:
    def __init__(self, name, choice):
        self.name = name
        self.choice = choice
        self._on_odor = choice._on_odor
        
        sha256 = hashlib.sha256(name.encode('utf-8'))
        sha256.update(choice.name.encode('utf-8'))
        
        self.key = base64.b32encode(sha256.digest())[0:8]
        
class ChoiceOutput(MindNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._on_odor = FiberKeyValue()
        
    def on_odor(self):
        return self._on_odor
    
    def to(self, target):
        self._on_odor.to(target)
        
    