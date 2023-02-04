import logging

from essaymind import MindNode, FiberKey, FiberKeyValue

log = logging.getLogger("Move")

class BodyMove(MindNode):
    def __init__(self, parent, name='move'):
        super().__init__(parent, name)
        
        self.body = body = parent.top
        self.world = body.world
        
        self._on_left_touch = FiberKeyValue()
        self._on_right_touch = FiberKeyValue()
        self._on_touch = FiberKey()
        
        self.actor = body.actor
        
        config = body.config
        
        self._speed_max = config.get("speed_max", 1)
        self._accel_max = config.get("accel_max", 1)
        self._turn_max = config.get("turn_max", 0.25)
        
    # build methods
        
    @staticmethod
    def from_body(node):
        return node.top['move']
    
    def speed_max(self, speed):
        assert not self.is_build()
        assert 0 <= speed and speed <= 1
        
        self._speed_max = speed
        
        return self
    
    def accel_max(self, accel_max):
        assert not self.is_build()
        assert 0 <= accel_max and accel_max <= 1
        
        self._accel_max = accel_max
        
        return self
    
    def turn_max(self, turn_max):
        assert not self.is_build()
        assert 0 <= turn_max and turn_max <= 0.5
        
        self._turn_max = turn_max
        
        return self
    
    def on_touch(self):
        assert not self.is_build()
        
        return self._on_touch
    
    def on_left_touch(self):
        assert not self.is_build()
        
        return self._on_left_touch
    
    def on_right_touch(self):
        assert not self.is_build()
        
        return self._on_right_touch
        
    def build(self):
        self._turn_request = 0
        self._speed_request = 0
        
        self.speed = 0
        
    # actions
        
    def set_speed(self, speed):
        assert 0 <= speed and speed <= 1
        
        self.speed = speed
        
    def turn(self, turn):
        assert -0.5 <= turn and turn <= 1
        
        if turn > 0.5:
            turn = turn - 1
            
        self._turn_request = max(min(turn, self._turn_max), -self._turn_max)
        
    def request_speed(self, speed):
        assert 0 <= speed and speed <= 1
        
        self._speed_request = min(speed, self._speed_max)
        
    # ticking

    def tick(self, ticks):
        #if self.body.ticks() % self.theta_ticks == 0:
        #    self.tick_update()
            
        self.tick_turn()
        self.tick_speed()
        self.tick_move()
        
    def tick_turn(self):
        turn = self._turn_request
        self._turn_request = 0
            
        actor = self.actor
        
        if turn: 
            dir_ego = (actor.dir_ego + turn + 1) % 1.0
        
            log.debug(f"turn_request dir={dir_ego:.3f}({actor.dir_ego:.3f}) turn_request={turn:.3f}")
        
            actor.set_dir(dir_ego)
            
    def tick_speed(self):
        speed_delta = self._speed_request - self.speed
        self._speed_request = 0
        
        if speed_delta:
            speed_delta = max(min(speed_delta, self._accel_max), -self._accel_max)
            
            self.speed += speed_delta
            
    def tick_move(self):
        actor = self.actor
            
        speed = self.speed

        if speed != 0:
            dx = actor.dir_dx * speed
            dy = actor.dir_dy * speed
            
            x = actor.x + dx
            y = actor.y + dy
        
            log.debug(f"move: (dx={dx:.2f},dy={dy:.2f}) -> ({x:.2f},{y:.2f}) dir_ego={actor.dir_ego:.3f} speed={self.speed:.2g}")
            
            if not actor.moveto(x, y):
                touch_dir = actor.touch_direction(x, y)

                self._on_touch()
                
                if touch_dir < 0.5:
                    log.debug(f"moveto-fail right: {x,y} {touch_dir}")
                    self._on_right_touch("right-touch", 1, 1)
                else:
                    log.debug(f"moveto-fail left: {x,y} {touch_dir}")
                    self._on_left_touch("left-touch", 1, 1)
