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
import logging

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
    
class Eye6:
    def __init__(self, width, samples=2):
        assert samples == 2
        assert width % 2 == 0
        self.width = width
        self.samples = samples
        self.pixels = width * samples + 3
        
        self.x_sample = [ 0, 1, 2, 0, 1, 2, 0, 1, 2]
        self.y_sample = [ 0, 0, 0, 1, 1, 1, 2, 2, 2]
        self.weight_sample = [0.25, 0.5, 0.25, 0.5, 1, 0.5, 0.25, 0.5, 0.25]
        weight_sum = 0
        for weight in self.weight_sample:
            weight_sum += weight
            
        self.weight_invsum = 1.0 / weight_sum
        
        self.pixel_data = numpy.zeros((self.pixels, self.pixels), 'f')
        
        self.avg_data = numpy.zeros((self.width, self.width), 'f')
        self.on_off_data = numpy.zeros((self.width - 2, self.width - 2), 'f')
        self.off_on_data = numpy.zeros((self.width - 2, self.width - 2), 'f')
        self.horiz_data = numpy.zeros((self.width - 2, self.width - 2), 'f')
        
        #self.remap_avg = self.init_remap(len(self.avg_data))
        #self.avg_remap_data = numpy.zeros((self.width, self.width), 'f')
        
        #self.remap_horiz = self.init_remap(len(self.horiz_data))
        #self.horiz_remap_data = numpy.zeros((self.width - 2, self.width - 2), 'f')
        
    def init_remap(self, width):
        remap_source = []
        remap_target = []
        
        width2 = (int) (0.5 * width)
        w2max = width2 - 1
        for j in range(width2):
            remap = []
            start_x = -(j + 1) + width2
            
            for i in range(j):
                remap.append((start_x + i, w2max - i))
                
            start_mid = width2 - 1 - (int) (j / 2)
            for i in range(2 + j):
                remap.append((i + start_mid, w2max - j))
                 
            for i in range(j):
                remap.append((i + width2 + 1, w2max - (j - i - 1)))
                
            remap_source.append(remap)
            
            target_len = min(width, 3 * j + 2)
            start = width2 - (int) (target_len / 2)
            target = []
            for i in range(target_len):
                target.append((start + i, w2max - j))
                
            remap_target.append(target)
            
        return (remap_source, remap_target)

    def analyze(self):
        self.average()
        self.on_off()
        self.border()
        #self.remap(self.remap_avg, self.avg_data, self.avg_remap_data)
        #self.remap(self.remap_horiz, self.horiz_data, self.horiz_remap_data)
        
    def average(self):
        width = self.width
        samples = self.samples
        
        x_s = self.x_sample
        y_s = self.y_sample
        w_s = self.weight_sample
        w_invsum = self.weight_invsum
        
        pixel_data = self.pixel_data
        avg_data = self.avg_data
        
        for j in range(width):
            y_off = j * samples
            x_index = j % 2
            for i in range(width):
                x_off = (i + x_index) * samples
                value = 0
                
                for k in range(len(x_s)):
                    value += w_s[k] * pixel_data[y_off + y_s[k]][x_off + x_s[k]]

                avg_data[j][i] = value * w_invsum
                
    def on_off(self):
        avg_data = self.avg_data
        on_off_data = self.on_off_data
        off_on_data = self.off_on_data
        width = self.width - 2
        invsum = 1.0 / 6.0
        
        max_on = 0
        max_off = 0
        for j in range(width):
            off = (j + 1) % 1
            for i in range(width):
                sum = 0
                sum -= avg_data[j][i + off + 0]
                sum -= avg_data[j][i + off + 1]
                
                sum -= avg_data[j + 1][i + 0]
                sum += avg_data[j + 1][i + 1] * 6
                sum -= avg_data[j + 1][i + 2]
                
                sum -= avg_data[j + 2][i + off + 0]
                sum -= avg_data[j + 2][i + off + 1]
                
                avg = invsum * sum
                
                on_off = max(0, avg)
                off_on = max(0, -avg)
                
                on_off_data[j][i] = on_off 
                off_on_data[j][i] = off_on
                
                max_on = max(on_off, max_on)
                max_off = max(off_on, max_off)
        
        if 1e-2 < max_on and max_on < 0.5:
            on_enhance = 0.5 / max_on
        else:
            on_enhance = 1
        
        if 1e-2 < max_off and max_off < 0.5:
            off_enhance = 0.5 / max_off
        else:
            off_enhance = 1
            
        for j in range(width):
            for i in range(width):
                on_off_data[j][i] *= on_enhance
                off_on_data[j][i] *= off_enhance
        #print(on_off_data)
                
    def border(self):
        on_off = self.on_off_data
        off_on = self.off_on_data
        data = self.horiz_data
        width = self.width - 2
        
        max_value = 0
        
        for j in range(width - 1):
            off0 = j % 2 - 1
            off1 = j % 2
            for i in range(width):
                i0 = max(0, (i + off0))
                i1 = min(width - 1, i + off1)
                sum = 0
                sum += on_off[j][i] * 0.5 * (off_on[j + 1][i0] + off_on[j + 1][i1])
                sum += off_on[j][i] * 0.5 * (on_off[j + 1][i0] + on_off[j + 1][i1])
                
                #data[j][i] = max(0, 0.333 * sum)
                data[j][i] = sum
                max_value = max(max_value, sum)
                
        if max_value > 1e-2:
            factor = 1.0 / max_value
            for j in range(width - 1):
                for i in range(width):
                    data[j][i] *= factor
        #print(on_off_data)
        
    def remap(self, remap, source_data, target_data):
        source_map = remap[0]
        target_map = remap[1]
        w2 = (int) (0.5 * len(source_data))
        
        for j in range(w2):
            source_j = source_map[j]
            target_j = target_map[j]
            
            if len(target_j) == len(source_j):
                for i in range(len(target_j)):
                    t_i = target_j[i]
                    s_i = source_j[i]
                 
                    target_data[t_i[1]][t_i[0]] = source_data[s_i[1]][s_i[0]]
            else:
                factor = 1.0 * len(source_j) / len(target_j)
             
                for i in range(len(target_j)):
                    t_i = target_j[i]
                    i_s = (int) (i * factor)
                    s_i0 = source_j[i_s]
                    s_i1 = source_j[i_s + 1]
                 
                    target_data[t_i[1]][t_i[0]] = (0.5 * source_data[s_i0[1]][s_i0[0]]
                                                  + 0.5 * source_data[s_i1[1]][s_i1[0]])
        
    def draw_grey(self, bounds):
        #glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        #glLoadIdentity()
        
        glColor3f(0.8, 0.7, 0.4)
        
        self.draw(bounds, self.avg_data)
        
    def draw_on_off(self, bounds):
        glColor3f(0.8, 0.7, 0.4)

        self.draw(bounds, self.on_off_data)
        
    def draw_off_on(self, bounds):
        glColor3f(0.8, 0.7, 0.4)

        self.draw(bounds, self.off_on_data)
        
    def draw_horiz(self, bounds):
        glColor3f(0.8, 0.7, 0.4)

        self.draw(bounds, self.horiz_data)
        
    def draw_grey_remap(self, bounds):
        glColor3f(0.8, 0.7, 0.4)

        self.draw(bounds, self.avg_remap_data)
        
    def draw_horiz_remap(self, bounds):
        glColor3f(0.8, 0.7, 0.4)

        self.draw(bounds, self.horiz_remap_data)
        
    def draw(self, bounds, data, offset=0):
        glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        #glLoadIdentity()
        
        width = len(data)
        dw = 1.0 / (width + 2)
        dw2 = 0.5 * dw
        dw4 = 0.25 * dw
        glRectf(0.0, 0.0, dw2, 1.0)
        glRectf(0.0, 0.0, 1.0, dw2)
        glRectf(1.0 - dw2, 0.0, 1.0, 1.0)
        glRectf(0.0, 1.0 - dw2, 1.0, 1.0)
        
        for j in range(width):
            hex_dx = dw2 * ((j + offset) % 2) + dw2 + dw4
            for i in range(width):
                grey = data[j][i]
                color = (grey * 0.8, grey, grey)
                x = i * dw + hex_dx
                y = (j + 1) * dw
                #pygame.draw.rect(surface, (255, 0, 0), 
                #                 pygame.Rect(x, y, x + dw, y + dh))
                glColor3fv(color)
                glRectf(x, y, x + dw, y + dw)
