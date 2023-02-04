import pygame
from pygame.locals import *
import OpenGL
from OpenGL.GL import *
from OpenGL.GL import shaders
from OpenGL.GLUT import *
from OpenGL.GLU import *
from OpenGL.arrays import vbo
from graphics.eye import Eye6
import glm
import numpy
import math
import logging
from graphics.scene import Scene, SceneComponent, Cube
from graphics.shader import LightShader
from world.world_map import WorldMap
from world.world_actor import WorldActor
from graphics.world_view import WorldView
from graphics.world_map_view import WorldMapView
from graphics.actor_view import ActorViewport
from graphics.camera_view import Camera, View

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
                
class MapWindow(object):
    def __init__(self, world_map, actor, ticker=None):
        assert isinstance(world_map, WorldMap)
        assert isinstance(actor, WorldActor)
        
        self.world_map = world_map
        self.actor = actor
        self.ticker = ticker
        
        self.is_auto_tick = False
        
        pygame.init()
        #display = (800, 600)
        display = (1000, 600)
        self.display = display
        self.surface = pygame.display.set_mode(display, DOUBLEBUF|OPENGL)
        
        self.views = []
        
        self.init_scene()
        self.init_main_view()
        self.init_eyes()
        self.init_map()
        
    def init_scene(self):
        scene = Scene()
       
        world_map = WorldMap(10, 10)
        world_map[(2, 2)] = 0.1
        world_map[(4, 4)] = 0.25
        
        world_view = WorldView(world_map)

        world_view.build(scene)
        
        scene.build()
        
        self.scene = scene
        
    def init_main_view(self):
        self.shader = LightShader()
        
        self.view_main = ActorViewport(actor, (0, 0, 600, 600))
        self.view_main.set_fov(45, 0.1)
        self.view_main.set_pos(0, 0.1, 0)
        
        self.views.append(self.view_main)
        
    def init_eyes(self):
        eye_pixels = 6
        eye_fov = 120
        eye_rotate = 60
        left_eye = Eye6(eye_pixels)
        self.left_eye = left_eye
        
        left_view = ActorViewport(actor, (610, 510, left_eye.pixels, left_eye.pixels))
        self.left_view = left_view
        
        left_view.set_fov(eye_fov, 0.1)
        left_view.set_greyscale(True)
        left_view.rotate(-eye_rotate, 0, 1, 0)
        left_view.set_pos(0, 0.1, 0.05)
        self.view_pov = left_view
        
        right_eye = Eye6(eye_pixels)
        self.right_eye = right_eye
        
        right_view = ActorViewport(actor, (910, 510, left_eye.pixels, right_eye.pixels))
        
        right_view.set_fov(eye_fov, 0.1)
        right_view.set_greyscale(True)
        right_view.rotate(eye_rotate, 0, 1, 0)
        right_view.set_pos(0, 0.1, 0.05)
        self.right_view = right_view
        
        self.views.append(left_view)
        self.views.append(right_view)
        
    def init_map(self):
        world_map_view = WorldMapView(world_map, ((700, 400, 200, 200)))
        world_map_view.add_actor(actor)
        self.world_map_view = world_map_view
        
    def render(self):
        #glViewport(0, 0, 600, 600)
        scene = self.scene    
        scene.render(self.view_main, self.shader)
        
        left_eye = self.left_eye
        self.scene.render(self.left_view, self.shader)
        glReadPixels(610, 510, left_eye.pixels, left_eye.pixels, GL_RED, GL_FLOAT, left_eye.pixel_data)
        left_eye.analyze()
            
        left_eye.draw_grey((600, 400, 100, 100))
            
        left_eye.draw_on_off((600, 300, 100, 100))
        left_eye.draw_off_on((600, 200, 100, 100))
        left_eye.draw_horiz((600, 100, 100, 100))
            
        right_eye = self.right_eye
        scene.render(self.right_view, self.shader)
        glReadPixels(910, 510, right_eye.pixels, right_eye.pixels, GL_RED, GL_FLOAT, right_eye.pixel_data)
        right_eye.analyze()
            
        right_eye.draw_grey((900, 400, 100, 100))
            
        right_eye.draw_on_off((900, 300, 100, 100))
        right_eye.draw_off_on((900, 200, 100, 100))
        right_eye.draw_horiz((900, 100, 100, 100))
        
        self.world_map_view.draw()
        
    def pygame_loop(self):
        glEnable(GL_DEPTH_TEST)
        glDepthFunc(GL_LESS)
        
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    quit()
                    
            glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT)
            self.render()
            
            self.update()
            
            if self.ticker and self.is_auto_tick:
                self.ticker.tick()
            
            pygame.display.flip()
            pygame.time.wait(30)
            
    def update(self):
        pressed_keys = pygame.key.get_pressed()
        
        factor = 0.05
        if pressed_keys[K_a]:
            self.actor.right(-factor)
        if pressed_keys[K_d]:
            self.actor.right(factor)
        
        if pressed_keys[K_w]:
            self.actor.forward(factor)
        if pressed_keys[K_s]:
            self.actor.forward(-factor)
                    
        if pressed_keys[K_q]:
            self.actor.turn_request(-1e-2)
        if pressed_keys[K_e]:
            self.actor.turn_request(1e-2)
        
        if pressed_keys[K_t]:
            self.is_auto_tick = not self.is_auto_tick
            
        if pressed_keys[K_SPACE]:
            if self.ticker:
                self.ticker.tick

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    world_map = WorldMap(10, 10)
    world_map[(2,2)] = 1
    world_map[(8,5)] = 10
    actor = WorldActor(world_map, 'actor')
    actor.moveto(0.5, 0.5)

    window = MapWindow(world_map, actor)
    window.pygame_loop()
