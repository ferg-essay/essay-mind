import random
import logging

from essaymind.core import MindNode, MindNodeRoot, Config
from essaymind.world import World, WorldActor

from .body_move import BodyMove

log = logging.getLogger("Body")

class Body(MindNodeRoot):
    def __init__(self, world=None, config=Config()):
        super().__init__('body')
        
        if not world:
            world = World().ignore_boundary(True)
            
        assert isinstance(world,World)
                
        #self.ticker.add_ticker(BodyTicker(world, self))
        
        #super().__init__('body', config)
        
        self.config = config
        self.world = world
        
        self._is_random_max = False
        self._random = random.uniform
        
        BodyWorldNode(self, 'world', world)
        
        self.actor = WorldActor(world.world_map, 'body')
        self.move = BodyMove(self)
        
        
        #self.loc = (0.0, 0.0)
        # direction is [0,1] clockwise
        # where 0==1 is north, 0.5 is south, 0.25 is east and 0.75 is west
        if world.is_ignore_boundary():
            assert self.moveto(0, 0)
        else:
            assert self.moveto(1, 1)
            
        self.set_dir(0)
        self.set_speed(0)
        # turn is clockwise [0,1]
        # where 0==1 is straight/forward, 0.5 is reverse, 0.25 is right, 0.75 is left
        
        self._action_name = None
        self._next_action_name = None

    def add_node(self, name, node):
        super().add_node(name, node)
            
        return node
        
    def set_dir(self, dir_ego):
        self.actor.set_dir(dir_ego)
        
    def set_speed(self, speed):
        self.move.set_speed(speed)
        
    def x(self):
        return self.actor.x
        
    def y(self):
        return self.actor.y
        
    def dx(self):
        return self.actor.dir_dx
        
    def dy(self):
        return self.actor.dir_dy
    
    def dir(self):
        return self.actor.dir_ego
    
    def speed(self):
        return self.move.speed
        
    def moveto(self, x, y):
        return self.actor.moveto(x, y)
    
    def get_near_obj(self, attr):
        obj = self.world.get_near_obj(self.loc, attr)
        
        if obj:
            obj.loc_ego = self.to_egocentric(obj.loc)
            log.info(f"sense {obj}")
        return obj
    
    def apply_obj(self, attr, lambda_map):
        self.world.apply_obj(attr, lambda_map)
    
    def get_object(self):
        return self.world.get_object(self.actor.loc)
    
    def remove_local_object(self):
        return self.world.remove_local_object(self.actor.loc)
    
    def remove_object(self, obj):
        return self.world.remove(obj)
    
    def seed(self, value):
        random.seed(value)
        
        return self
    
    def random_max(self, value):
        self._is_random_max = value
        
        return self
    
    def random(self, value):
        if self._is_random_max:
            return random.uniform(max(0, value - 1e-3), value)
        else:
            return random.uniform(0, value)
        
    def action_name(self, name):
        self._next_action_name = name
    
    '''
    def ticks(self):
        return self.ticker.ticks
    
    def tick(self):
        assert self._is_build
        
        log.info(f"[{self.ticks():3d}] {self} tick")
        
        self.ticker.tick()
        '''
        
    def tick(self):
        super().tick()
        
        self._action_name = self._next_action_name
        self._next_action_name = None
            
    def __str__(self):
        mood = self.node_map.get('mood')
        
        if mood:
            mood_names = mood.get_active_mood_names()
        else:
            if self._next_action_name:
                mood_names = [self._next_action_name]
            elif self._action_name:
                mood_names = [self._action_name]
            else:
                mood_names = []
            
        actor = self.actor
            
        return f'Body({actor.x:.2f},{actor.y:.2f},d={actor.dir_ego:.2g},s={self.speed():.2g}){mood_names}'

class BodyWorldNode(MindNode):
    def __init__(self, parent, name, world):
        super().__init__(parent, name)
        self._world = world
        
    def tick(self, ticks):
        self._world.tick(self.top, ticks)
        