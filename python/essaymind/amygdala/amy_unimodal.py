import logging

from essaymind import MindNode, FiberKey

log = logging.getLogger("Amy")

class AmyUnimodal(MindNode):
    '''
    Mapping from a raw sense to a value.
    
    Single sense, no context.
    '''

    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._on_value = FiberKey()
        self._value_map = dict()
        
    def when_sense(self, fiber):
        fiber.to(self.sense)
        
        return self
        
    def on_value(self, target):
        assert not self.is_build()
        
        self._on_value.to(target)
        
        return self
    
    # active methods
    
    def value(self, key):
        self._value_map[key] = 1
        return self
        
    def sense(self, key):
        log.info(f"sense {key}")
        value = self._value_map.get(key)
        
        if value:
            self._on_value(key)