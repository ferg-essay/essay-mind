'''
Created on Sep 15, 2022

@author: ferg
'''
import pygame
from pygame.locals import *

from OpenGL.GL import *
import logging

from world.world import World
from world.world_map import WorldMap
from world.world_actor import WorldActor
from graphics.world_map_view import WorldMapView
from graphics.actor_controller import ActorController
from graphics.pygame_loop import PyGameLoop

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
                
class MapWindow(object):
    def __init__(self, world, actor, ticker=None):
        assert isinstance(world, World)
        assert isinstance(actor, WorldActor)
        
        self.pygame = PyGameLoop((600, 400))
        
        self.world = world
        self.world_map = world.world_map
        self.actor = actor
        self.ticker = ticker
        
        self.actor_controller = ActorController(actor, ticker)
        self.pygame.add_controller(self.actor_controller)
        
        world_map_view = WorldMapView(self.world, ((000, 000, 400, 400)))
        world_map_view.add_actor(self.actor)
        self.world_map_view = world_map_view
        self.pygame.add_view(world_map_view)
        
    def pygame_loop(self):
        self.pygame.pygame_loop()

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    world = World((10, 10))
    world_map = world.world_map
    world_map[(2,2)] = 1
    world_map[(8,5)] = 10
    actor = WorldActor(world_map, 'actor')
    actor.moveto(0.5, 0.5)

    window = MapWindow(world, actor)
    window.pygame_loop()
