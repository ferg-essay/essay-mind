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

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

#log = logging.getLogger("Test")

cube_vertices_old = (
    ( 1, -1, -1),
    ( 1,  1, -1),
    (-1,  1, -1),
    (-1, -1, -1),
    
    ( 1, -1,  1),
    ( 1,  1,  1),
    (-1, -1,  1),
    (-1,  1,  1)
    )

cube_vertices_new = (
    (-1, -1, -1),
    (-1,  1, -1),
    ( 1,  1, -1),
    ( 1, -1, -1),
    
    (-1, -1,  1),
    (-1,  1,  1),
    ( 1,  1,  1),
    ( 1, -1,  1),
    )

cube_vertices = (
    (0, 0, 0),
    (0, 1, 0),
    (1, 1, 0),
    (1, 0, 0),
    
    (0, 0, 1),
    (0, 1, 1),
    (1, 1, 1),
    (1, 0, 1),
    )

cube_surfaces = (
    (2, 3, 7, 6), # right
    (0, 4, 5, 1), # left
    (1, 5, 6, 2), # top
    (0, 3, 7, 4), # bottom
    (0, 3, 2, 1), # back
    (4, 7, 6, 5), # front
    )

cube_normals = [
    [ 1,  0,  0],
    [-1,  0,  0],
    [ 0,  1,  0],
    [ 0, -1,  0],
    [ 0,  0,  1],
    [ 0,  0, -1],
    ]

cube_edges = (
    (0, 1),
    (0, 3),
    (0, 4),
    (2, 1),
    (2, 3),
    (2, 7),
    (6, 3),
    (6, 4),
    (6, 7),
    (5, 1),
    (5, 4),
    (5, 7)
    )

class Cube:
    def __init__(self, pos=(0, 0, 0), stretch=(1, 1, 1)):
        self.vertices = self.translate_stretch(pos, stretch)
        self.corner_low = self.vertices[0]
        self.corner_high = self.vertices[4]
        self.normals = cube_normals
        self.surfaces = cube_surfaces
        
    def translate_stretch(self, pos, stretch):
        vertices = []
        
        for vertix in cube_vertices:
            vertices.append([vertix[0] * stretch[0] + pos[0],
                             vertix[1] * stretch[1] + pos[1],
                             vertix[2] * stretch[2] + pos[2]])
            
        return vertices
    
    def calculate_vertices(self, values):
        for i in range(len(self.surfaces)):
            surface = self.surfaces[i]
            normal = self.normals[i]
            
            values.append(self.vertices[surface[0]] + normal)
            values.append(self.vertices[surface[1]] + normal)
            values.append(self.vertices[surface[2]] + normal)
            
            values.append(self.vertices[surface[2]] + normal)
            values.append(self.vertices[surface[3]] + normal)
            values.append(self.vertices[surface[0]] + normal)
            
        return values
            
    
    def invert_normals(self):
        normals = []
        for normal in cube_normals:
            normals.append([-normal[0], -normal[1], -normal[2]])
            
        self.normals = normals
            
class Scene:
    def __init__(self):
        self.component = None
        self.components = []
        
    def color(self, color):
        self.component = SceneComponent(color)
        self.components.append(self.component)
        
    def add_cube(self, loc, size):
        self.add_item(Cube(loc, size))
        
    def ambient(self, ambient):
        self.component.ambient = ambient
        
    def add_item(self, item):
        if not self.component:
            self.color((1, 1, 1, 1))
            
        self.component.add_item(item)
        
    def build(self):
        for component in self.components:
            component.build() 
        
    def render(self, view, shader):
        view.pre_render(shader)
        for component in self.components:
            component.render(view, shader)
        
class SceneComponent:
    def __init__(self, color):
        self.color = color
        self.grey = math.sqrt(color[0] * color[0] + color[1] * color[1] + color[2] * color[2])
        self.ambient = 0.1
        self.items = []
        
    def add_item(self, item):
        self.items.append(item)
        
    def build(self):
        vertices = []
        for item in self.items:
            vertices = item.calculate_vertices(vertices)

        self.vertices = vertices
        self.n = len(vertices)
        self.vbo = vbo.VBO(numpy.array(vertices, 'f'))
        
    def render(self, view, shader):
        shader.set_color(self.color, self.grey)
        shader.set_ambient(self.ambient)
        shader.render(view, self.vbo, self.n)
