import logging

from essaymind import MindNode, FiberAngle, FiberKey

log = logging.getLogger('Select')

class StriatumAngle(MindNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._on_action = FiberAngle()
        
        self._sense_map = dict()
        
        self._ticks = 10
        self.unselect_ticks = 0
        
    # build methods
    
    def ticks(self, ticks):
        self._ticks = ticks
        
        return self
        
    def when_sense(self, name, fiber):
        assert not super().is_build()
        assert isinstance(fiber, FiberAngle)
        assert not self._sense_map.get(name)
        
        item = SenseItem(self, name, fiber)
        self._sense_map[name] = item
        
        return self
    
    def when_valence(self, fiber):
        assert not super().is_build()
        assert isinstance(fiber, FiberKey)
        
        fiber.to(self.valence)
        
        return self
    
    def on_action(self, target):
        assert not super().is_build()
        
        self._on_action.to(target)
    
    # active methods
    
    def valence(self, key, p):
        log.info(f"valence {key} {p}")
    
    def select(self):
        self.select_ticks = self.selector.select_tick_max
    
    def unselect(self):
        self.unselect_ticks = self.selector.unselect_tick_max
        
    def tick(self, ticks):
        return
        
class SenseItem:
    def __init__(self, parent, name, fiber):
        assert isinstance(fiber, FiberAngle)
        
        self._parent = parent
        self.name = name
        
        fiber.to(self.sense)
        
    def sense(self, key, angle):
        log.info(f"sense {self.name}-Item {key} {angle:.2f}")
