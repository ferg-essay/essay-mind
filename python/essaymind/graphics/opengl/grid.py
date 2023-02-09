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

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')


class GridShader:
    def __init__(self):
        self.init_shaders()
        self.init_loc()
        
    def init_shaders(self):
        VERTEX_SHADER = shaders.compileShader("""
attribute vec2 pos;
uniform float scale;
uniform vec4 color;
varying vec4 vertex_color;
void main() {
   gl_Position = vec4(pos[0] * scale, pos[1] * scale, 0, 1);
   vertex_color = color;
}""", GL_VERTEX_SHADER)

        FRAGMENT_SHADER = shaders.compileShader("""
        varying vec4 vertex_color;
void main() {
  gl_FragColor = vertex_color;
}""", GL_FRAGMENT_SHADER)
#gl_FragColor = vec4(0.5, 1, 1, 1);
        
        self.shader = shaders.compileProgram(VERTEX_SHADER, FRAGMENT_SHADER)

    def init_loc(self):
        self.color_loc = self.get_uniform_loc('color')
        self.scale_loc = self.get_uniform_loc('scale')
        
        self.pos_loc = self.get_attribute_loc('pos')
        
        self.color = (1, 1, 1, 1)
            
    def get_uniform_loc(self, name):
        location = glGetUniformLocation(self.shader, name)
        
        if location in (None, -1):
            log.warning(f"No uniform: {name}")
            
        return location
            
    def get_attribute_loc(self, name):
        location = glGetAttribLocation(self.shader, name)
        
        if location in (None, -1):
            log.warning(f"No attribute: {name}")
            
        return location
        
    def render(self, view, vbo, n):
        shaders.glUseProgram(self.shader)
        try:
            vbo.bind()
            try:
                self.enable(vbo)

                glDrawArrays(GL_TRIANGLES, 0, n)
            finally:
                vbo.unbind()
                self.disable()
        finally:
            shaders.glUseProgram(0)
            
    def set_color(self, color):
        self.color = color
            
    def set_scale(self, scale):
        self.scale = scale
        
    def enable(self, vbo):
        color = self.color
        
        glUniform4f(self.color_loc, color[0], color[1], color[2], color[3])
        glUniform1f(self.scale_loc, self.scale)
        
        glEnableVertexAttribArray(self.pos_loc)
        glVertexAttribPointer(self.pos_loc, 2, GL_FLOAT, False, 2 * 4, vbo); 
        
    def set_uniform_3f(self, loc, values):
        glUniform3f(loc, values[0], values[1], values[2])
        
    def disable(self):
        glDisableVertexAttribArray(self.pos_loc)

g_angles = [[-0.50, 0.87, 0.50, 0.87],
            [ 0.50, 0.87, 1.00, 0.00],
            [ 1.00, 0.00, 0.50,-0.87],
            [ 0.50,-0.87,-0.50,-0.87],
            [-0.50,-0.87,-1.00, 0.00],
            [-1.00, 0.00,-0.50, 0.87],
            ]
    
class GridView:
    def __init__(self, bounds):
        self.bounds = bounds
        self.init_triangles()
        self.shader = GridShader()
        self.shader.set_color((0.8, 0.7, 0.4, 1))
        self.shader.set_scale(0.1)
        
    def init_triangles(self):
        vertices = []
        
        self.add_hex(vertices, 0, 0)
        self.add_hex(vertices, 4, 0)
        self.add_hex(vertices, 8, 0, 2)
        
        self.init_vbo(vertices)
        
    def add_hex(self, vertices, x_i, y_i, size=1):
        for angle in range(6):
            self.add_triangle(vertices, x_i, y_i, angle, size)
        
    def add_triangle(self, vertices, x_i, y_i, angle, size=1):
        x = 0.5 * x_i
        y = 0.87 * y_i
        angles = g_angles[angle]
        vertices.append(x)
        vertices.append(y)
        vertices.append(x + angles[0] * size)
        vertices.append(y + angles[1] * size)
        vertices.append(x + angles[2] * size)
        vertices.append(y + angles[3] * size)
        
    def init_vbo(self, vertices):
        triangles = numpy.array(vertices, 'f')
        self.n = (int) (len(vertices) / 2)
        self.vbo = vbo.VBO(triangles)
        
    def draw(self):
        bounds = self.bounds
        #glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        #glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        glViewport(bounds[0], bounds[1], bounds[2], bounds[3])
        #glColor4f(1, 1, 1, 0.5)
        self.shader.render(self, self.vbo, self.n)
        #glRectf(0.0, 0.0, 1.0, 1.0)
                
class GridWindow(object):
    def __init__(self):
        pygame.init()
        #display = (800, 600)
        display = (600, 400)
        self.display = display
        self.surface = pygame.display.set_mode(display, DOUBLEBUF|OPENGL)
        
        self.views = []
        
        self.init_map()
        
    def init_map(self):
        grid_view = GridView(((000, 000, 400, 400)))
        self.grid_view = grid_view
        
    def render(self):
        self.grid_view.draw()
        
    def pygame_loop(self):
        glEnable(GL_DEPTH_TEST)
        glDepthFunc(GL_LESS)
        
        glEnable(GL_BLEND)
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
        
        while True:
            self.update()
                
            glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT)
            self.render()
            
            pygame.display.flip()
            pygame.time.wait(30)
            
    def update(self):
        pressed_keys = pygame.key.get_pressed()
        factor = 0.05
        
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                quit()

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    window = GridWindow()
    window.pygame_loop()

