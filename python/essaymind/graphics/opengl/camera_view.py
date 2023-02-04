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
#from opengl.world_map import WorldMap
from graphics.world_view import WorldView

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

        
class Camera(object):
    def __init__(self):
        self.matrix = glm.mat4(1.0)
        
        #self.display = display
        self.pos = glm.vec3(0.2, 0.2, 0)
        
        self.dir_forward = glm.vec4(0, 0, -1, 0)
        self.dir_right = glm.vec4(1, 0, 0, 0)
        self.dir_up = glm.vec4(0, 1, 0, 0)
        
        self.rot = glm.mat4(1.0)
        
        #self.view_pos = glm.vec3(0, 0, -2)
        
        #self.per = glm.perspective(glm.radians(120),
        #                           1,
        #                           0.01, 50)

        self.update()
        
    def set_pos(self, x, y, z):
        self.pos = glm.vec4(x, y, z, 1)
        self.update()
        
    def rotate(self, angle, x, y, z):
        #self.rot = glm.rotate(glm.radians(angle), x, y, z) * self.rot
        self.rot = glm.rotate(self.rot, glm.radians(angle), glm.vec3(x, y, z))
        self.update()
        
    def forward(self, factor):
        self.pos = self.pos + factor * glm.vec3(self.dir_forward)
        self.update()
        
    def right(self, factor):
        self.pos = self.pos + factor * glm.vec3(self.dir_right)
        self.update()
        
    def up(self, factor):
        self.pos = self.pos + factor * glm.vec3(self.dir_up)
        self.update()
        
    def update(self):
        rot_inv = glm.inverse(self.rot)
        self.dir_forward = rot_inv * glm.vec4(0, 0, -1, 0)
        self.dir_right = rot_inv * glm.vec4(1, 0, 0, 0)
        self.dir_up = rot_inv * glm.vec4(0, 1, 0, 0)

        #self.mvp = (self.per * self.rot
        #            * glm.translate(self.view_pos) 
        #            * glm.translate(-self.pos) * glm.mat4(1.0))
        
        #self.mvp = (self.per * self.rot
        #            * glm.translate(self.view_pos) 
        #            * glm.translate(-self.pos))
        
    #def get_mvp_ptr(self):
    #    return glm.value_ptr(self.mvp)

class View:
    def __init__(self, viewport, camera, shader):
        self.viewport = viewport
        self.shader = shader
        self.camera = camera
        
        self.set_fov(45, 1)
        self.is_greyscale = False
        self.rot = glm.mat4(1.0)
        
    def set_fov(self, angle, z):
        self.per = glm.perspective(glm.radians(angle),
                                   self.viewport[2] / self.viewport[3],
                                   min(0.1, z / 4), 50)
        
        self.view_pos = glm.vec3(0, 0, -z)
        
    def set_scale(self, sx, sy, sz):
        self.per = glm.scale(glm.vec3(sx, sy, sz))
        
    def set_greyscale(self, is_greyscale):
        self.is_greyscale = is_greyscale
        
    def rotate(self, angle, x, y, z):
        #self.rot = glm.rotate(glm.radians(angle), x, y, z) * self.rot
        self.rot = glm.rotate(self.rot, glm.radians(angle), glm.vec3(x, y, z))
        #self.update()
        
    def pre_render(self):
        viewport = self.viewport
        glViewport(viewport[0], viewport[1], viewport[2], viewport[3])

        self.shader.set_mvp(self.get_mvp())
        self.shader.set_greyscale(self.is_greyscale)
        
    def get_mvp(self):
        camera = self.camera
        mvp = (self.per
               * self.rot
               * camera.rot
               * glm.translate(self.view_pos)
               * glm.translate(- camera.pos))
        
        self.mvp = mvp

        return mvp
