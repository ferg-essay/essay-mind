import logging
from body.node import BodyNode
from body.fiber import FiberKeyValue

log = logging.getLogger("Theta")

class Theta(BodyNode):
    def __init__(self, parent, name='theta'):
        super().__init__(parent, name)
        
        self._period = 10
        self._ticks = 0
        
        self._on_data = FiberKeyValue()
        
    def period(self, period):
        assert period >= 0 and period % 2 == 0
        
        self._period = period
        
        return self
    
    def on_data(self):
        assert not self.is_build()
        
        return self._on_data
    
    def to(self, target):
        assert not self.is_build()
        
        self._on_data.to(target)
        
        return self
    
    def build(self):
        super().build()
        
        self._half_ticks = self._period / 2
    
    # build
    
    def up(self):
        assert self.is_build()
        assert self._ticks == 0
        
        self._ticks = 1
        self._on_data.send(self.name, 1, 1)
    
    def down(self):
        assert self.is_build()
        assert self._ticks == 1
        
        self._ticks = 0
        self._on_data.send(self.name, 0, 1)
    
    def tick(self):
        if not self.period:
            return
        
        if self._ticks == 0:
            self._on_data.send(self.name, 1, 1)
        elif self._ticks == self._half_ticks:
            self._on_data.send(self.name, 0, 1)
        
        self._ticks += 1
        
        if self._period <= self._ticks:
            self._ticks = 0