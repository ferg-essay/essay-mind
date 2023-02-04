import logging
import pygame
from pygame.locals import *


from essaymind import World, WorldMap, WorldActor

from essaymind.graphics.pygame.world_map_view import WorldMapView
from essaymind.graphics.pygame.pygame_loop import PyGameLoop
from essaymind.graphics.pygame.actor_controller import ActorController
from .ray import RayScene, RayTracing
from graphics.eye6 import Eye6Blur, EyeRay, Eye6OnOff, Eye6Border, EyePipeline, eye_update_gain
from graphics.eye_view import EyeView, Eye6View, EyePipelineView

log = logging.getLogger("RayWindow")
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
    
class RayWindow:
    def __init__(self, world, actor, bounds):
        assert isinstance(world, World)
        assert isinstance(actor, WorldActor)
        self.bounds = bounds
        
        w = 10
        
        self.eye_pipeline = EyePipeline(90, w, world, actor)
        self.eye_view = EyePipelineView(self.eye_pipeline, bounds)
        '''
        self.eye_ray = EyeRay(90, w, w, world, actor)
        
        x0 = bounds[0]
        y0 = bounds[1]
        w = bounds[2]
        w2 = w // 2
        h = bounds[2]
        h2 = h // 3
        
        yt = y0 + h - h2
        
        self.data = self.eye_ray.data
        self.eye_view = EyeView(self.data, (x0, yt, w2, h2))
        
        self.eye_blur = Eye6Blur(self.eye_ray.data)
        self.blur_view = Eye6View(self.eye_blur.data, (x0 + w2, yt, w2, h2))
        
        self.eye_onoff = Eye6OnOff(self.eye_blur.data)
        #self.eye_onoff = Eye6OnOff(self.eye_ray.data)
        self.on_view = Eye6View(self.eye_onoff.on_data, (x0, yt - h2, w2, h2))
        self.off_view = Eye6View(self.eye_onoff.off_data, (x0 + w2, yt - h2, w2, h2))
        
        self.eye_border = Eye6Border(self.eye_onoff.on_data, self.eye_onoff.off_data)
        self.border_view = Eye6View(self.eye_border.data, (x0 + w2, y0, w2, h2))
        '''
        
        
    def render(self):
        self.eye_pipeline.tick()
        self.eye_view.render()
        '''
        self.eye_ray.tick()
        self.eye_blur.tick()
        self.eye_onoff.tick()
        eye_update_gain(self.eye_onoff.on_data, self.eye_onoff.width, 3)
        eye_update_gain(self.eye_onoff.off_data, self.eye_onoff.width, 3)
        self.eye_border.tick()
        eye_update_gain(self.eye_border.data, self.eye_border.width, 3)
        
        self.eye_view.render()
        self.blur_view.render()
        self.on_view.render()
        if self.off_view:
            self.off_view.render()
        if self.border_view:
            self.border_view.render()
        '''
        
def ray_test():
    world = World((10, 10))
    world_map = world.world_map

    world_map[(5,5)] = 1
    
    actor = WorldActor(world_map, 'actor')
    actor.moveto(0.5, 0.5)
    
    pygame = PyGameLoop((800, 400))
    
    actor_controller = ActorController(actor)
    pygame.add_controller(actor_controller)
        
    world_map_view = WorldMapView(world, ((000, 000, 400, 400)))
    world_map_view.add_actor(actor)
    pygame.add_view(world_map_view)
    
    fov = 90
    w = 10
    angle = 0.125
    
    eye_left = EyePipeline(fov, w, world, actor, 1.0 - angle)
    pygame.add_ticker(eye_left)
    
    eye_left_view = EyePipelineView(eye_left, (400, 000, 200, 400))
    pygame.add_view(eye_left_view)
    
    eye_right = EyePipeline(fov, w, world, actor, angle)
    pygame.add_ticker(eye_right)
    
    eye_right_view = EyePipelineView(eye_right, (600, 000, 200, 400))
    pygame.add_view(eye_right_view)
    
    #ray_view = RayWindow(world, actor, ((400, 000, 400, 400)))
    #pygame.add_view(ray_view)
    
    pygame.pygame_loop()

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    ray_test()
