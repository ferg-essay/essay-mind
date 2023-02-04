import pygame
from pygame.locals import *
import OpenGL
from OpenGL.GL import *
from OpenGL.GL import shaders
from OpenGL.GLUT import *
from OpenGL.GLU import *
from OpenGL.arrays import vbo
from vision.eye import Eye6
import glm
import numpy
import math
import logging
from vision.scene import Scene, SceneComponent, Cube
from vision.shader import LightShader
from world.world_map import WorldMap
from world.world_actor import WorldActor
from vision.world_view import WorldView
from vision.world_map_view import WorldMapView
from vision.actor_view import ActorViewport
from vision.camera_view import Camera, View

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
                
class VisualOpenGL(object):
    def init_pygame(self):
        pygame.init()
        #display = (800, 600)
        display = (1000, 600)
        self.display = display
        self.surface = pygame.display.set_mode(display, DOUBLEBUF|OPENGL)
        
    def pygame_loop(self):
        scene = Scene()
       
        world_map = WorldMap(10, 10)
        world_map[(2, 2)] = 0.1
        world_map[(4, 4)] = 0.25
        
        world_view = WorldView(world_map)

        world_view.build(scene)
        
        actor = WorldActor(world_map, 'actor')
        self.actor = actor
        actor.moveto(0.5, 0.5)
        
        '''        
        scene.color((0.9, 0.85, 0.7, 1))
        scene.add_item(Cube((-1, 0, -10), (12, 1, 1)))
        scene.add_item(Cube((-1, 0,   1), (12, 1, 1)))
        scene.add_item(Cube((-1, 0,  0), (1, 1, 10)))
        scene.add_item(Cube((10, 0,  0), (1, 1, 10)))
        
        scene.add_item(Cube((1, 0, -1), (1, 1, 1)))
        scene.add_item(Cube((2, 0, -2), (1, 1, 1)))
        '''
        
        scene.build()
        
        #self.camera = Camera()
        #self.camera = ActorCamera(actor)
        shader = LightShader()
        
        self.view_main = ActorViewport(actor, (0, 0, 600, 600))
        self.view_main.set_fov(45, 0.1)
        self.view_main.set_pos(0, 0.1, 0)
        
        #map_camera = Cawmera()
        #map_camera.rotate(-90, 1, 0, 0)
        #map_view = View((600, 0, 100, 100), map_camera, shader)
        #map_camera.set_pos(0, 0, -5)
        #map_view.set_scale(10, 10, 1)
        
        #eye = Eye(10, 3)
        #eye = Eye6(8, 2)
        eye_pixels = 6
        eye_fov = 120
        eye_rotate = 60
        left_eye = Eye6(eye_pixels)
        
        left_view = ActorViewport(actor, (610, 510, left_eye.pixels, left_eye.pixels))
        
        left_view.set_fov(eye_fov, 0.1)
        left_view.set_greyscale(True)
        left_view.rotate(-eye_rotate, 0, 1, 0)
        left_view.set_pos(0, 0.1, 0.05)
        self.view_pov = left_view
        
        right_eye = Eye6(eye_pixels)
        
        right_view = ActorViewport(actor, (910, 510, left_eye.pixels, right_eye.pixels))
        
        right_view.set_fov(eye_fov, 0.1)
        right_view.set_greyscale(True)
        right_view.rotate(eye_rotate, 0, 1, 0)
        right_view.set_pos(0, 0.1, 0.05)
        self.right_view = right_view
        
        world_map_view = WorldMapView(world_map, ((700, 400, 200, 200)))
        world_map_view.add_actor(actor)
        
        glEnable(GL_DEPTH_TEST)
        glDepthFunc(GL_LESS)
        
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    quit()
                    
            self.update()
        
            glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT)
            
            #glViewport(0, 0, 600, 600)    
            scene.render(self.view_main, shader)
        
            scene.render(left_view, shader)
            glReadPixels(610, 510, left_eye.pixels, left_eye.pixels, GL_RED, GL_FLOAT, left_eye.pixel_data)
            left_eye.analyze()
            
            left_eye.draw_grey((600, 400, 100, 100))
            
            left_eye.draw_on_off((600, 300, 100, 100))
            left_eye.draw_off_on((600, 200, 100, 100))
            left_eye.draw_horiz((600, 100, 100, 100))
            
            #left_eye.draw_grey_remap((700, 400, 100, 100))
            #left_eye.draw_horiz_remap((700, 300, 100, 100))
            
            scene.render(right_view, shader)
            glReadPixels(910, 510, right_eye.pixels, right_eye.pixels, GL_RED, GL_FLOAT, right_eye.pixel_data)
            right_eye.analyze()
            
            right_eye.draw_grey((900, 400, 100, 100))
            
            right_eye.draw_on_off((900, 300, 100, 100))
            right_eye.draw_off_on((900, 200, 100, 100))
            right_eye.draw_horiz((900, 100, 100, 100))
            
            #right_eye.draw_grey_remap((800, 400, 100, 100))
            #right_eye.draw_horiz_remap((800, 300, 100, 100))
            #eye.draw_horiz((700, 100, 100, 100))
            #scene.render(map_view)
            world_map_view.draw()
            
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
            self.camera.up(0.1)
        if pressed_keys[K_v]:
            self.camera.up(-0.1)

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    open_gl = VisualOpenGL()
    open_gl.init_pygame()    
    open_gl.pygame_loop()