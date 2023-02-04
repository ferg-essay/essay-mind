from essaymind.body.action_probability import ActionNode
from essaymind.body.body_move import BodyMove

import logging

log = logging.getLogger("Move")

class MoveNode(ActionNode):
    def __init__(self, parent, name, turn=0, speed=None):
        super().__init__(parent, name)
        
        if speed == None:
            speed = speed_def(parent)
        
        self.speed(speed)
        self.turn(turn)
        
    def speed(self, speed):
        assert not self.is_build()
        self._speed = speed
        return 
        
    def turn(self, turn):
        assert not self.is_build()
        assert -0.5 <= turn <= 1.0
        
        if turn > 0.5:
            turn = turn - 1
            
        self._turn = turn
        return 

    def build(self):
        super().build()
        self.move = BodyMove.from_body(self.top)
        
        self._turn_delta = self._turn / self._ticks
        self._speed_delta = self._speed / self._ticks
        
    def action(self, value):
        #log.debug(f"move {self.name} {value}")
        self.move.request_speed(self._speed_delta)
        
        if self._turn_delta:
            self.move.turn(self._turn_delta)

# basic directions        
class Forward(MoveNode):
    def __init__(self, parent, name='forward'):
        super().__init__(parent, name, 0, speed_med(parent))
        
class Stop(MoveNode):
    def __init__(self, parent, name='stop'):
        super().__init__(parent, name, 0, 0)
        
class Left(MoveNode):
    def __init__(self, parent, name='left'):
        super().__init__(parent, name, -turn_def(parent), 0)
        
class Right(MoveNode):
    def __init__(self, parent, name='right'):
        super().__init__(parent, name, turn_def(parent), 0)
        
class ForwardLeft(MoveNode):
    def __init__(self, parent, name='forward.left'):
        super().__init__(parent, name, -turn_def(parent), speed_def(parent))
        
class ForwardRight(MoveNode):
    def __init__(self, parent, name='forward.right'):
        super().__init__(parent, name, turn_def(parent), speed_def(parent))
        
# other speeds
        
class ForwardLow(MoveNode):
    def __init__(self, parent, name='forward_low'):
        super().__init__(parent, name, 0, speed_low(parent))
        
class ForwardMed(MoveNode):
    def __init__(self, parent, name='forward_med'):
        super().__init__(parent, name, 0, speed_low(parent))
        
class ForwardHigh(MoveNode):
    def __init__(self, parent, name='forward_high'):
        super().__init__(parent, name, 0, speed_high(parent))
        
class LeftLow(MoveNode):
    def __init__(self, parent, name='left_low'):
        super().__init__(parent, name, -turn_low(parent), 0)
        
class RightLow(MoveNode):
    def __init__(self, parent, name='right_low'):
        super().__init__(parent, name, turn_low(parent), 0)
        
class ForwardLowLeft(MoveNode):
    def __init__(self, parent, name='forward_low.left'):
        super().__init__(parent, name, -turn_low(parent), speed_low(parent))
        
class ForwardLowRight(MoveNode):
    def __init__(self, parent, name='forward_low.right'):
        super().__init__(parent, name, speed_low(parent), turn_low(parent))
        
def speed_def(node):
    return speed_med(node)
        
def speed_low(node):
    return node.top.config.get("speed.low", 0.1)
        
def speed_med(node):
    return node.top.config.get("speed.med", 0.2)
        
def speed_high(node):
    return node.top.config.get("speed.high", 0.4)
        
def turn_def(node):
    return turn_med(node)
        
def turn_low(node):
    return node.top.config.get("turn.low", 0.02)
        
def turn_med(node):
    return node.top.config.get("turn.med", 0.05)
        
def turn_high(node):
    return node.top.config.get("turn.high", 0.1)
