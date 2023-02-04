import logging

from essaymind.core.node import MindNode
from essaymind.body.body import Body
from essaymind.fiber.fiber import FiberAngle

log = logging.getLogger("Nose")

class BodyNose(MindNode):
    def __init__(self, body, name='nose'):
        assert isinstance(body, Body)
        
        super().__init__(body, name)
        
        self.body = body
        self.world = body.world

        self._on_odor = FiberAngle(name)
        self.value_odor = None
        
        self.nose_dist = 0.25
        
    def on_odor(self):
        return self._on_odor
    
    def to(self, target):
        self.on_odor().to(target)
        return self
    
    # actions
        
    def odor(self, odor):
        body = self.body
        x = body.x()
        y = body.y()
        
        value = odor.calculate_value(x, y)
        
        angle = odor.calculate_dir(x, y)
        body_angle = (angle - body.dir() + 1) % 1.0
        
        nose_odor = NoseOdor(odor.name, body_angle, value)
        
        #p = 1
        #self._on_odor(nose_odor, body_angle, p)
        self._on_odor(odor.name, body_angle)
        self.value_odor = nose_odor
        
    def tick(self, ticks):
        odor = self.value_odor
        self.value_odor = None
        #if odor:
        #    for target in self.targets:
        #        target.odor(odor)
        
    def __str__(self):
        return f'Nose[{self.value_odor}]'
    
class NoseOdor:
    def __init__(self, name, angle, value):
        self.name = name
        self.angle = angle
        self.value = value
        
    def __str__(self):
        return f"{type(self).__name__}[{self.name},{self.angle:.4f},{self.value:.4f}]"
