import logging
import math

from essaymind import BodyNode
from essaymind import FiberKeyValue

log = logging.getLogger("Decay")

class Decay(BodyNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._half_life = 1
        self._threshold = 0.5
        
        self._on_value = FiberKeyValue()
        
    def half_life(self, half_life):
        assert not self.is_build()
        assert half_life > 0
        
        self._half_life = half_life
        
        return self
        
    def threshold(self, threshold):
        assert not self.is_build()
        assert 0 <= threshold and threshold <= 1
        
        self._threshold = threshold
        
        return self
    
    def on_value(self):
        assert not self.is_build()
        
        return self._on_value
    
    def to(self, target):
        self.on_value().to(target)
        
        return self
    
    def when_excite(self, fiber):
        fiber.to(self.on_excite)
        
        return self
    
    def when_inhibit(self, fiber):
        fiber.to(self.on_inhibit)
        
        return self
        
    def build(self):
        super().build()
        
        self._value = 0
        self._decay = math.pow(2, - 1 / self._half_life)
        
    # runtime methods
        
    def value(self, value):
        assert 0 <= value and value <= 1
        self._value = value
        
    def excite(self, value):
        assert 0 <= value and value <= 1
        
        self._value = min(1, self._value + value)
        
    def on_excite(self, key, value, p):
        log.info(f"excite {self} ({key},{value},{p})")
        
        self.excite(value)
        
    def inhibit(self, value):
        assert 0 <= value and value <= 1
        
        log.info(f"inhibit {self} {value}")
        self._value = max(0, self._value - value)
        
    def on_inhibit(self, key, value, p):
        log.info(f"inhibit {self} ({key},{value},{p})")
        
        self.inhibit(value)
        
    def tick(self):
        self._value *= self._decay
        
        self._on_value.send(self.name, self._value, 1)

    def __str__(self):
        return f"{self.name}-Decay[{self._value},hl={self._half_life}]"