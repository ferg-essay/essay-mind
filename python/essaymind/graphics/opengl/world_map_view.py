import pygame
from pygame.locals import *
import OpenGL
from OpenGL.GL import *
from OpenGL.GL import shaders
from OpenGL.GLUT import *
from OpenGL.GLU import *
from OpenGL.arrays import vbo
import glm
import numpy
import math
from world.world import World
from world.world_actor import WorldActor
import logging

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
    
class WorldMapView:
    def __init__(self, world, bounds):
        assert isinstance(world, World)
        self.world = world
        world_map = world.world_map
        self.world_map = world_map
        self.border = 0.05
        self.bounds = bounds
        self.rects = []
        self.actors = []
        
        dw = 0.9 / world_map.width
        dl = 0.9 / world_map.length
        
        self.dx = dw
        self.dy = dl
        self.x0 = self.border
        self.y0 = self.border
        
        for j in range(world_map.length):
            for i in range(world_map.width):
                value = world_map[(i, j)]
                
                if value:
                    x0 = self.border + i * dw 
                    y0 = self.border + j * dw 
                    self.rects.append((x0, y0, x0 + dw, y0 + dl))
                    
    def add_actor(self, actor):
        assert isinstance(actor, WorldActor)
        
        self.actors.append(actor)
        
    def render(self):
        border = self.border
        bounds = self.bounds
        glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        
        glColor3f(0.8, 0.7, 0.4)
        glRectf(0.0, 0.0, border, 1.0)
        glRectf(0.0, 0.0, 1.0, border)
        glRectf(1.0 - border, 0.0, 1.0, 1.0)
        glRectf(0.0, 1.0 - border, 1.0, 1.0)
        
        glColor3f(0.7, 0.6, 0.3)
        for rect in self.rects:
            glRectf(rect[0], rect[1], rect[2], rect[3])
            
        for actor in self.actors:
            self.draw_actor(actor)
            
        x0 = self.border
        dx = self.dx
        dy = self.dy
        dx4 = 0.25 * self.dx
        dy4 = 0.25 * self.dy
        
        glColor3f(0.2, 0.8, 0.8)
        for obj in self.world.get_objects():
            x = x0 + dx * obj.loc[0]
            y = x0 + dy * obj.loc[1]
            glRectf(x, y, x + dx4, y + dy4)
            
    def draw_actor(self, actor):
        dx = self.dx
        dy = self.dy
        
        x = self.x0 + actor.x * dx        
        y = self.y0 + actor.y * dy
        
        dx2 = actor.dir_dx * 0.25 * dx 
        dy2 = actor.dir_dy * 0.25 * dy
        
        glColor3f(1.0, 1.0, 1.0)
        glRectf(x + dx2 - 0.1 * dx, y + dy2 - 0.1 * dy,
                x + dx2 + 0.1 * dx, y + dy2 + 0.1 * dy)
        
        glColor3f(1.0, 0.2, 0.2)
        glRectf(x - 0.25 * dx, y - 0.25 * dy, x + 0.25 * dx, y + 0.25 * dy)
        