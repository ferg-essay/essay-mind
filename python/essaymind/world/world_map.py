import numpy
import logging
import math

log = logging.getLogger("WorldMap")

class WorldMap(object):
    def __init__(self, width, length):
        assert width > 0
        assert length > 0
        
        self.width = width
        self.length = length
        self.world_map = numpy.zeros((length, width))
        self.is_ignore_boundary = False
        
    def __getitem__(self, point):
        x = point[0]
        y = point[1]
        assert x >= 0 and x < self.width
        assert y >= 0 and y < self.length
        
        return self.world_map[y][x]
    
    def __setitem__(self, point, value):
        x = point[0]
        y = point[1]
        assert x >= 0 and x < self.width
        assert y >= 0 and y < self.length
        #assert value >= 0 and value <= 1 
        assert value >= 0
        
        self.world_map[y][x] = value
        
    def set_ignore_boundary(self, is_ignore):
        self.is_ignore_boundary = is_ignore
        
    def is_move(self, x, y, radius):
        if (x - radius < 0 or 
            y - radius < 0 or 
            x + radius >= self.width or
            y + radius >= self.length):
            return self.is_ignore_boundary
        
        x0 = (int) (x - radius)
        x1 = (int) (x + radius)
        y0 = (int) (y - radius)
        y1 = (int) (y + radius)
        
        world_map = self.world_map

        return (not world_map[y0][x0] and
                not world_map[y0][x1] and
                not world_map[y1][x0] and
                not world_map[y1][x1] or
                self.is_ignore_boundary)
        
    def touch_direction(self, x, y, radius):
        if x - radius < 0:
            return 0.75
        elif x + radius >= self.width:
            return 0.25
        elif y - radius < 0:
            return 0.5
        elif y + radius >= self.length:
            return 0
        
        x0 = (int) (x - radius)
        x1 = (int) (x + radius)
        assert x1 - x0 <= 1
        y0 = (int) (y - radius)
        y1 = (int) (y + radius)
        assert y1 - y0 <= 1
        
        world_map = self.world_map
        
        if world_map[y0][x0]:
            return hit_dir(x, y, x0 + 0.5, y0 + 0.5)
        elif world_map[y0][x1]:
            return hit_dir(x, y, x1 + 0.5, y0 + 0.5)
        elif world_map[y1][x0]:
            return hit_dir(x, y, x0 + 0.5, y1 + 0.5)
        elif world_map[y1][x1]:
            return hit_dir(x, y, x1 + 0.5, y1 + 0.5)
        else:
            log.info(f"assert ({x:.2f},{y:.2f};{radius:.2f})")
            assert False
            
def hit_dir(x0, y0, x1, y1):
    return (round(math.atan2(x1 - x0, y1 - y0) / (2 * math.pi), 2) + 1) % 1.0        
        