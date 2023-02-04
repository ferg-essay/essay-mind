from math import sin, cos, pi
import logging

from .world_map import WorldMap

log = logging.getLogger("Actor")

class WorldActor(object):
    def __init__(self, world, name, loc=(0.0, 0.0), radius=0.1):
        assert isinstance(world, WorldMap)
        
        self.world = world
        self.name = name
        self.x = loc[0]
        self.y = loc[1]
        self.radius = radius
        # direction is [0,1] clockwise
        # where 0==1 is north, 0.5 is south, 0.25 is east and 0.75 is west
        self.set_dir(0)
        
    def set_dir(self, dir_ego):
        assert 0 <= dir_ego and dir_ego < 1
        
        self.dir_ego = round(dir_ego, 4)
        # [0-1] clockwise with 0==1 north (dx=0,dy=1)
        self.dir_dx, self.dir_dy = calculate_dxdy(dir_ego)
        
    def forward(self, length):
        x = self.x + length * self.dir_dx
        y = self.y + length * self.dir_dy
        
        return self.moveto(x, y)
        
    def right(self, length):
        x = self.x + length * self.dir_dy
        y = self.y - length * self.dir_dx
        
        return self.moveto(x, y)
        
    def turn(self, turn):
        assert -1 <= turn and turn <= 1
        
        dir_ego = (self.dir_ego + turn + 1.0) % 1.0
        
        self.set_dir(dir_ego)
        
    def moveto(self, x, y):
        if self.world.is_move(x, y, self.radius):
            self.x = x
            self.y = y
            return True
        else:
            #log.info(f"MOVTO {self.world.is_ignore_boundary}")
            return False
        
    def is_collision_ahead(self, dir_ego, length):
        dx, dy = calculate_dxdy(dir_ego)
        
        return not self.world.is_move(self.x + dx * length, self.y + dy * length, self.radius)
        
    def touch_direction(self, x, y):
        return (self.world.touch_direction(x, y, self.radius) - self.dir_ego + 1) % 1
        
    def __str__(self):
        return f"{self.name}-Actor({self.x:.2f},{self.y:.2f},d={self.dir_ego:.2g})"

def calculate_dxdy(dir_ego):
    dir_dx = round(sin(2 * pi * dir_ego), 4)
    dir_dy = round(cos(2 * pi * dir_ego), 4)
    
    return dir_dx, dir_dy
    