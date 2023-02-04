import logging
from enum import Enum
from body.node import BodyNode

log = logging.getLogger("Motion")

IDLE = 0
EXPLORE = 1
TOWARD = 2
AWAY = 3
STOP = 4
ATTACK = 5
ESCAPE = 6
FREEZE = 7
TAIL = FREEZE + 1

motion_names = ["idle", "explore", "toward", "away", "stop", "attack", "escape", "freeze"]
motion_speed = [0, 0.5, 0.5, 0.5, 0, 1, 1, 0]

class MotionVote(BodyNode):
    
    def __init__(self, body):
        body['motion.vote'] = self
        self.body = body

    def build(self):
        self.body = self.body
        self.move = self.body['move']
        
        self.request_values = [0] * TAIL
        
        self.left = 0
        self.right = 0
        
        self.action = 0
        self.item = 0
        self.away_value = 0
        self.turn_value = 0
        self.action = None
        self.turn_request = 0
        self.speed_request = 0
        
    @staticmethod
    def from_body(area):
        return area.top['motion.vote']
        
    def freeze(self, value):
        self.request(FREEZE, value, 0, 0)
        
    def escape(self, value, left, right):
        self.request(ESCAPE, value, right, left)
        
    def attack(self, value, left, right):
        self.request(ESCAPE, value, left, right)
        
    def stop(self, value):
        self.request(STOP, value, 0, 0)
        
    def away(self, value, left, right):
        self.request(AWAY, value, right, left)
        
    def toward(self, value, left, right):
        self.request(TOWARD, value, left, right)
        
    def explore(self, value, left, right):
        self.request(EXPLORE, value, left, right)
        
    def request(self, index, value, left, right):
        assert 0 <= value and value <= 1
        assert 0 <= left and left <= 1
        assert 0 <= right and right <= 1
        
        value = max(1e-3, value)
        
        self.request_values[index] = max(self.request_values[index], value)
        self.left = max(self.left, left)
        self.right = max(self.right, right)
        
    def tick(self):
        #body = self.body
        #dir_ego = body.dir()
        
        left = min(0.25, self.left)
        self.left = 0
        
        right = min(0.25, self.right)
        self.right = 0
        
        turn = right - left
        
        if not turn and right > 0:
            turn = right
        
        values = self.request_values
        self.values = values
        self.request_values = [0] * TAIL
        
        best_i = 0
        best_value = 0
        for i in range(TAIL):
            if values[i] > 0:
                best_i = i
                best_value = values[i]
                
        name = motion_names[best_i]
        speed = motion_speed[best_i]
        
        if best_i == 0:
            self.action = None
            self.speed_request = 0
            self.turn_request = 0
            return
        
        if best_value < 0.1:
            speed *= 0.1
        
        if speed == 0:
            turn = 0
            
        self.move.request_speed(speed)
        self.move.turn((1.0 + turn) % 1.0)
        
        self.action = name
        self.speed_request = speed
        self.turn_request = turn
        self.value = best_value
        
        log.info(f"{name} turn_request={turn} speed_request={speed} value={best_value} left={left} right={right}")
            
    def __str__(self):
        return f"{type(self).__name__}[{self.action},turn_request={self.turn_request:.2f},speed_request={self.speed_request:.2f}]"
    