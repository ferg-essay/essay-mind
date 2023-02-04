import logging

from essaymind.body.body import MindNode

log = logging.getLogger('OdorHab')

class OdorHab(MindNode):
    def __init__(self, body, odor_name):
        self.name = odor_name
        self.body = body
        self.body['odor.hab'] = self
        self.value = 0
        self.value_next = 0
        
    def build(self, body):
        self.body = body
        
        nose = body['body.nose']
        nose[self.name] = self
        
        self.hab = body['habenula']
    
    def odor(self, odor, value_left, value_right):
        self.odor_value = odor
        self.value_left = value_left
        self.value_right = value_right
        value = value_left + value_right
        self.value_next = value
        if value > 0:
            self.turn_request = min(0.25, max(-0.25, (value_right - value_left) / value))
        
    
    def tick(self, body):
        turn = self.turn_request
        self.turn_request = 0
        
        value = self.value
        self.value = self.value_next
        self.value_next = 0
        
        dv = self.value - value
        speed = 1
        # if moving away, force a turn_request
        if dv < 0 and abs(turn) < 1e-2:
            turn = 0.1
        elif dv < 0:
            speed = 0.1
            
        if turn:
            log.debug(f"{self.name} turn_request={turn_request:.2f} {self.odor_value} L={self.value_left:.2f} R={self.value_right:.2f}")
            
            self.hab.add_left_toward(max(0, -turn))
            self.hab.add_right_toward(max(0, turn))
            self.hab.add_toward(speed)