import logging
import math

from essaymind.core import Ticker
from .world_map import WorldMap

log = logging.getLogger("World")

class World(Ticker):
    def __init__(self, bounds=(10, 10)):
        self.objects = []
        self.node_map = dict()
        
        #Temp(self)
        self.world_map = WorldMap(bounds[0], bounds[1])
        
        self.time = 0
        return
    
    # build
            
    def ignore_boundary(self, is_ignore):
        #self.is_ignore_boundary = is_ignore
        self.world_map.set_ignore_boundary(is_ignore)
        return self
    
    # actions
    
    def __setitem__(self, name, node):
        self.node_map[name] = node
        
    def __getitem__(self, name):
        return self.node_map[name]
    
    def add_object(self, obj):
        self.objects.append(obj)
        return self
        
    def get_object(self, loc):
        for obj in self.objects:
            if math.dist(obj.loc, loc) < 0.1:
                return obj
            
        return None
    
    def get_objects(self):
        return self.objects
    
    def set_map(self, loc, value):
        self.world_map[loc] = value
    
    def get_map(self, loc):
        return self.world_map[loc]
    
    def tick(self, body, ticks):
        self.time = ticks
        for ticker in self.objects:
            ticker.tick(body)
        
    def is_ignore_boundary(self):
        return self.world_map.is_ignore_boundary
        
    def remove_local_object(self, loc):
        while True:
            obj = self.get_object(loc)
            if not obj:
                return
            self.remove(obj)
        
    def get(self, loc):
        for obj in self.objects:
            if obj.loc == loc:
                return obj
            
        return None
    
    def get_near_obj(self, loc, attr):
        best_obj = None
        best_dist = 10e10
        
        for obj in self.objects:
            if obj.attr(attr):
                dist = math.dist(obj.loc, loc)
                if dist < best_dist:
                    best_obj = obj
                    best_dist = dist

        return best_obj
    
    def apply_obj(self, attr, map_lambda):
        for obj in self.objects:
            if obj.attr(attr):
                map_lambda(obj)
    
    def is_move(self, x, y, radius):
        return self.world_map.is_move(x, y, radius)
    
    def touch_direction(self, x, y, radius):
        return self.world_map.touch_direction(x, y, radius)
        
    def to_egocentric(self, loc_self, loc_obj):
        dx = loc_obj[0] - loc_self[0]
        dy = loc_obj[1] - loc_self[1]
        
        dist = math.dist(loc_obj, loc_self)
        rad = math.atan2(dy, dx)
        dir_clock = math.degrees(rad) / 30
        
        return (dir_clock, dist)
        
    def remove(self, obj):
        self.objects.remove(obj)
        
class WorldTicker:
    def tick(self, body):
        return        
        
class WorldObject(WorldTicker):
    def __init__(self, loc, attrs = {}):
        assert isinstance(attrs, dict)

        self.loc = loc
        self.attrs = attrs
        
    def attr(self, name):
        return self.attrs.get(name)
    
    def tick(self, body):
        return
    
    def __str__(self):
        return f'WorldObject{self.loc}{self.attrs}'
