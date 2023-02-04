import logging

from essaymind.core.node import MindNode
from essaymind.fiber.fiber import FiberKeyValue

log = logging.getLogger("Front")

class FrontDecide(MindNode):
    '''
    Explore vs Exploit
    
    Exploitation occurs as long as consistent success. With lack of success, 
    explore to another area.
    '''
    def __init__(self, parent, name='front.decide'):
        super().__init__(parent, name)
        
        self._on_explore = FiberKeyValue()
        self._on_exploit = FiberKeyValue()

        self._threshold = 10
        
    def on_explore(self):
        return self._on_explore
            
    def to_explore(self, action):
        self.on_explore().to(action)
        
        return self
        
    def on_exploit(self):
        return self._on_exploit
        
    def to_exploit(self, action):
        self.on_exploit().to(action)
        
        return self
        
    def threshold(self, threshold):
        assert 0 < threshold
        
        self._threshold = threshold
        
        return self
        
    def when_theta(self, theta):
        theta.to(self.theta)
        
        return self
        
    def when_reward(self, reward):
        reward.to(self.reward)
        
        return self
        
    def build(self):
        super().build()
        
        self._time = 0
        
        self._is_up = False
        
    def theta(self, key, value, p):
        assert self.is_build()
        
        self._is_up = value == 1
        
    def reward(self, key, value, p):
        assert self.is_build()
        
        self._time = 0
        
    def tick(self):
        is_up = self._is_up
        if not is_up:
            return
        self._is_up = False
        
        if self._time < self._threshold:
            self._time += 1
            self._on_exploit.send(self.name, 1, 1)
        else:
            self._time = 0
            self._on_explore.send(self.name, 1, 1)
            
        
        


        