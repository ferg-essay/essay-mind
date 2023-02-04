from body.body import BodyNode
from body.nexus import Nexus 
import numpy as np

import logging

log = logging.getLogger("OT")

class OpticTectum(BodyNode):
    def __init__(self, area, name, onoff):
        self.name = name
        self.area = area
        
        area[name] = self
        
        self.on_data = onoff.on_data
        self.off_data = onoff.off_data
        
        self.len = (int) (np.sqrt(len(self.on_data)))
        
        assert self.len * self.len == len(self.on_data)
        assert len(self.on_data) == len(self.off_data)
        
        self.top = self.len // 2 - 2
        self.bottom = max(self.len // 2 + 2, self.len - 2)
        
        self._on_barrier = self.area.nexus(self.name + '.on_barrier')
        
    #def set_barrier(self, barrier):
    #    assert isinstance(barrier, Nexus)
    #    self.barrier = barrier
    
    def on_barrier(self):
        return self._on_barrier
        
    #def build(self):
    #    assert self.barrier
        
    def tick(self):
        is_obstacle = self.is_left_border() or self.is_right_border()
        
        if is_obstacle:
            log.info(f"barrier detected L={self.is_left_border()},R={self.is_right_border()}")
            self._on_barrier()
    
    def is_left_border(self):
        on_data = self.on_data
        off_data = self.off_data
        length = self.len
        
        for j in range(self.top, self.bottom):
            if on_data[j * length] or off_data[j * length]:
                return False
            
        return True
    
    def is_right_border(self):
        on_data = self.on_data
        off_data = self.off_data
        length = self.len
        offset = length - 1
        
        for j in range(self.top, self.bottom):
            if on_data[j * length + offset] or off_data[j * length + offset]:
                return False
            
        return True
    