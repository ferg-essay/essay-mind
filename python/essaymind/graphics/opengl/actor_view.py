from OpenGL.GL import *
import glm
from math import pi
import logging

from world.world_actor import WorldActor

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

class ActorViewport:
    def __init__(self, actor, viewport):
        assert isinstance(actor, WorldActor)
        
        self.actor = actor
        
        self.height = 0.1
        
        self.matrix = glm.mat4(1.0)
        self.rot_0 = glm.mat4(1.0)
        self.rot_y = glm.vec3(0, 1, 0)
        self.viewport = viewport
        
        self.rot = glm.mat4(1.0)
        self.rotate(0, 0, 1, 0)
        self.set_fov(45, 0)
        self.set_pos(0, 0, 0)
        self.is_greyscale = False
        
    def set_fov(self, angle, z):
        self.per = glm.perspective(glm.radians(angle),
                                   self.viewport[2] / self.viewport[3],
                                   min(0.1, z / 4), 50)
        
    def set_pos(self, x, y, z):
        self.view_translate = glm.translate(glm.vec3(0, 0, z))
    
    def set_greyscale(self, is_greyscale):
        self.is_greyscale = is_greyscale
        
    def rotate(self, angle, x, y, z):
        self.view_rot = glm.rotate(self.rot_0, glm.radians(angle), glm.vec3(x, y, z))
        
    def pre_render(self, shader):
        viewport = self.viewport
        glViewport(viewport[0], viewport[1], viewport[2], viewport[3])
        
        actor = self.actor
        
        rot_actor = glm.rotate(self.rot_0, actor.dir_ego * 2 * pi, self.rot_y)
        pos_actor = glm.vec3(actor.x, self.height, -actor.y)
        
        mvp = (self.per
               * self.view_rot
               * rot_actor
               * self.view_translate
               * glm.translate(- pos_actor))

        shader.set_mvp(mvp)
        shader.set_greyscale(self.is_greyscale)
