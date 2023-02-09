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


class PlainShader:
    def __init__(self):
        self.init_shaders()
        
    def init_shaders(self):
        VERTEX_SHADER = shaders.compileShader("""
varying vec4 vertex_color;
void main() {
   gl_Position = gl_ModelViewProjectionMatrix * gl_Vertex;
   vertex_color = gl_Color;
}""", GL_VERTEX_SHADER)

        FRAGMENT_SHADER = shaders.compileShader("""
        varying vec4 vertex_color;
void main() {
  gl_FragColor = vertex_color;
}""", GL_FRAGMENT_SHADER)
#gl_FragColor = vec4(0.5, 1, 1, 1);
        
        self.shader = shaders.compileProgram(VERTEX_SHADER, FRAGMENT_SHADER)
        
phong_weight = '''
float weightCalc(
  in vec3 light_pos,
  in vec3 frag_normal)
{
  return max(0.0, dot(frag_normal, light_pos));
}  
'''

vertex_light = '''
attribute vec3 vertex_position;
attribute vec3 vertex_normal;

uniform mat4 mvp;

//uniform vec4 global_ambient;
//uniform vec4 light_ambient;
//uniform vec4 light_diffuse;
uniform vec4 material;
uniform float ambient;
uniform float light0;
uniform vec3 light0_location;
uniform float light1;
uniform vec3 light1_location;

varying vec4 baseColor;

void main()
{
  // gl_Position = gl_ModelViewProjectionMatrix * vec4(vertex_position, 1.0);
  gl_Position = mvp * vec4(vertex_position, 1.0);
  
//  vec3 ec_light_location = normalize(gl_NormalMatrix * light_location);
//  vec3 ec_vertex_normal = normalize(gl_NormalMatrix * vertex_normal);

  vec3 ec_light0_location = normalize(light0_location);
  vec3 ec_light1_location = normalize(light1_location);
  vec3 ec_vertex_normal = normalize(vertex_normal);
  
  float dot0 = max(0.0, dot(ec_vertex_normal, ec_light0_location));
  float dot1 = max(0.0, dot(ec_vertex_normal, ec_light1_location));

  //baseColor = clamp((global_ambient * material_ambient)
  //                  + (light_ambient * material_ambient)
  //                  + (light_diffuse * material_diffuse * diffuse_weight),
  //                  0.0, 1.0);
                                      
  baseColor = clamp((ambient + light0 * dot0 + light1 * dot1) * material, 0.0, 1.0);
}
'''

fragment_light = '''
varying vec4 baseColor;
void main()
{
  gl_FragColor = baseColor;
}
'''

class LightShader:
    def __init__(self):
        self.init_shaders()
        #self.camera = camera
        
        #self.global_ambient_loc = self.get_uniform_loc('global_ambient')
        #self.light_ambient_loc = self.get_uniform_loc('light_ambient')
        #self.light_diffuse_loc = self.get_uniform_loc('light_diffuse')
        
        self.material_loc = self.get_uniform_loc('material')
        
        self.ambient_loc = self.get_uniform_loc('ambient')
        
        self.light0_loc = self.get_uniform_loc('light0')
        self.light0_location_loc = self.get_uniform_loc('light0_location')
        self.light1_loc = self.get_uniform_loc('light1')
        self.light1_location_loc = self.get_uniform_loc('light1_location')
        
        self.mvp_loc = self.get_uniform_loc('mvp')
        
        self.vertex_position_loc = self.get_attribute_loc('vertex_position')
        self.vertex_normal_loc = self.get_attribute_loc('vertex_normal')
        
        self.ambient = 0.1
        self.light0 = 0.7
        self.light1 = 0.3
        self.light0_location = (-1, 3, 2)
        self.light1_location = (1, 3, -2)
        self.color = (1, 1, 1, 1)
        self.is_greyscale = False
        
    def set_light0_location(self, loc):
        assert len(tuple) == 3
        self.light0_location = loc
        return self
        
    def set_light1_location(self, loc):
        assert len(tuple) == 3
        self.light1_location = loc
        return self
        
    def init_shaders(self):
        #VERTEX_SHADER = shaders.compileShader(phong_weight + vertex_light, GL_VERTEX_SHADER)
        VERTEX_SHADER = shaders.compileShader(vertex_light, GL_VERTEX_SHADER)

        FRAGMENT_SHADER = shaders.compileShader(fragment_light, GL_FRAGMENT_SHADER)

        self.shader = shaders.compileProgram(VERTEX_SHADER, FRAGMENT_SHADER)
            
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
                self.enable(view, vbo)

                glDrawArrays(GL_TRIANGLES, 0, n)
            finally:
                vbo.unbind()
                self.disable()
        finally:
            shaders.glUseProgram(0)
            
    def set_color(self, color, grey):
        self.color = color
        self.grey = grey
            
    def set_ambient(self, ambient):
        self.ambient = ambient
        
    def set_greyscale(self, is_greyscale):
        self.is_greyscale = is_greyscale
    
    def set_mvp(self, mvp):
        self.mvp = mvp
        
    def enable(self, view, vbo):
        #glUniform4f(self.global_ambient_loc, 0.2, 0.2, 0.2, 1.0)
        #glUniform4f(self.global_ambient_loc, 0.8, 0.9, 0.9, 1.0)
        #glUniform4f(self.global_ambient_loc, 0.2, 0.2, 0.2, 1.0)
        #glUniform4f(self.global_ambient_loc, 0, 0, 0, 1.0)
        
        color = self.color
        #glUniform4f(self.light_ambient_loc, 1, 1, 1, 1)
        
        if self.is_greyscale:
            grey = self.grey
            glUniform4f(self.material_loc, grey, grey, grey, grey)
        else:
            glUniform4f(self.material_loc, color[0], color[1], color[2], color[3])
        
        
        #glUniform4f(self.light_ambient_loc, ambient, ambient, ambient, 1)
        glUniform1f(self.ambient_loc, self.ambient)
        glUniform1f(self.light0_loc, (1 - self.ambient) * self.light0)
        glUniform1f(self.light1_loc, (1 - self.ambient) * self.light1)
        
        #glUniformMatrix4fv(self.mvp_loc, 1, GL_FALSE, self.camera.get_mvp_ptr())
        glUniformMatrix4fv(self.mvp_loc, 1, GL_FALSE, glm.value_ptr(self.mvp)) # view.get_mvp()))
        #glUniformMatrix4fv(self.mvp_loc, 1, GL_FALSE, view.camera.get_mvp_ptr())

        self.set_uniform_3f(self.light0_location_loc, self.light0_location)
        self.set_uniform_3f(self.light1_location_loc, self.light1_location)
        
        
        glEnableVertexAttribArray(self.vertex_position_loc)
        glEnableVertexAttribArray(self.vertex_normal_loc)
        glVertexAttribPointer(self.vertex_position_loc, 3, GL_FLOAT, False, 6 * 4, vbo); 
        glVertexAttribPointer(self.vertex_normal_loc, 3, GL_FLOAT, False, 6 * 4, vbo + 3 * 4)
        
    def set_uniform_3f(self, loc, values):
        glUniform3f(loc, values[0], values[1], values[2])
        
    def disable(self):
        glDisableVertexAttribArray(self.vertex_position_loc) 
        glDisableVertexAttribArray(self.vertex_normal_loc) 
