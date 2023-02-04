from body.body import BodyNode
import numpy as np
import logging
from motion.motion_vote import MotionVote

log = logging.getLogger("Optic")

class Optic(BodyNode):
    def __init__(self, body, data, side):
        assert side == 'l' or side == 'r'
        body['tectum.' + side] = self
        
        self.data = data
        length = (int) (np.sqrt(len(self.data)))
        assert length * length == len(self.data)
        
        self.length = length

        horizon = (length + 1) // 2
        
        if side == 'l':
            self.m_upper_horizon = horizon * length - 1
            self.dx = -1
        else:
            self.m_upper_horizon = (horizon - 1) * length
            self.dx = 1
            
        self.n_border = length // 2 + 1
        
        log.info(f"side={side} horizon={self.m_upper_horizon} length={length} n_border={self.n_border} dx={self.dx}")
        
    def build(self, body):
        self.locomotion = MotionVote.from_body(body)
        
    def is_border_empty_1(self):
        data = self.data
        m_upper_horizon = self.m_upper_horizon
        length = self.length
        
        for i in range(self.n_border):
            if data[m_upper_horizon + i * length] >= 1e-2:
                return False
            
        return True
        
    def is_border_empty_n(self, n):
        data = self.data
        m_upper_horizon = self.m_upper_horizon
        length = self.length
        dx = self.dx
        
        for j in range(self.n_border):
            for i in range(n):
                if data[m_upper_horizon + j * length + i * dx] >= 1e-2:
                    return False
            
        return True
    
    def tick(self, body):
        if self.is_border_empty_1():
            self.locomotion.stop(1)
            