from OpenGL.GL import *
import logging
import math
import numpy as np
from mood.mood import Moods
from body.category_area import CategoryArea

log = logging.getLogger("MoodView")
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
    
class MoodView:
    def __init__(self, area, bounds):
        assert isinstance(area, CategoryArea)
        self.area = area
        
        self.values = self.area.categories
        
        self.dws = []
        
        h = len(self.values)
        w = 1
        for category in self.values:
            dw_cat = 0.9 / max(1, len(category))
            self.dws.append(dw_cat)
            
            w = max(w, len(category))
            
        self.h = max(1, h)
        self.w = max(1, w)
        
        self.data = np.zeros((self.h, self.w), 'f')
        
        self.bounds = bounds

        self.colors = [(1.0, 0.25, 0.25),
                       (0.75, 0.5, 0.25),
                       (0.50, 0.7, 0.50),
                       (0.25, 0.5, 0.75),
                       (0.10, 0.5, 0.85),
                       (0.0, 0.5, 1.0)]
        
        self.border = 0.05
        self.rects = []
        
        dw = 0.9 / self.w
        dh = 0.9 / self.h
        
        self.dx = dw
        self.dy = dh
        self.x0 = self.border
        self.y0 = self.border
        
    def render(self):
        bounds = self.bounds
        glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        
        w = self.w
        h = self.h
        
        border = self.border
        x0 = self.border
        dx = self.dx
        dy = self.dy
        
        values = self.values
        
        for j in range(len(values)):
            color = self.colors[j]
            category = values[j]
            
            dx = self.dws[j]
            
            for i in range(len(category)):
                value = category[i].value
                
                if value:
                    glColor3f(value * color[0], value * color[1], value * color[2])
                 
                    x = x0 + dx * i
                    y = x0 + dy * (h - j - 1)
                
                    glRectf(x, y, x + dx, y + dy)
                
                #if data[j][j]:
                #    glRectf(x, y, x + dx, y + dy)
                

        color = (1, 1, 1)
        glColor3f(color[0], color[1], color[2])
        glRectf(0, 1 - border, 1, 1)
        glRectf(0, 0, 1, border)
        glRectf(0, 0, border, 1)
        glRectf(1 - border, 0, 1, 1)
