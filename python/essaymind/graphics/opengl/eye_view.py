from OpenGL.GL import *
import logging
import math

log = logging.getLogger("EyeView")
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
    
class EyePipelineView:
    def __init__(self, eye, bounds):
        self.eye = eye
        
        x0 = bounds[0]
        y0 = bounds[1]
        w = bounds[2]
        #w2 = w // 2
        dw = w
        h = bounds[3]
        dh = h // 5
        dw = min(dh, w)
        
        yt = y0 + h - dh
        
        self.eye_view = EyeView(eye.eye_ray.data, (x0, yt, dw, dh))
        
        self.blur_view = Eye6View(eye.eye_blur.data, (x0, yt - dh, dw, dh))
        
        self.on_view = Eye6View(eye.eye_onoff.on_data, (x0, yt - 2 * dh, dw, dh))
        self.off_view = Eye6View(eye.eye_onoff.off_data, (x0, yt - 3 * dh, dw, dh))
        
        #self.border_view = Eye6View(eye.eye_border.data, (x0, y0, dw, dh))
        
    def render(self):
        self.eye_view.render()
        self.blur_view.render()
        self.on_view.render()
        self.off_view.render()
        #self.border_view.render()
        
class EyeView:
    def __init__(self, data, bounds, color=(0.5, 0.8, 0.7)):
        self.bounds = bounds

        w = (int) (math.sqrt(len(data)))
        assert w * w == len(data)
        
        self.w = w
        self.h = w

        self.data = data
        self.color = color
        
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
        
        data = self.data
        
        for j in range(h):
            for i in range(w):
                grey = data[j * w + i]
                
                glColor3f(grey, grey, grey)
                 
                x = x0 + dx * i
                y = x0 + dy * (h - j - 1)
                
                glRectf(x, y, x + dx, y + dy)
                

        color = self.color
        glColor3f(color[0], color[1], color[2])
        glRectf(0, 1 - border, 1, 1)
        glRectf(0, 0, 1, border)
        glRectf(0, 0, border, 1)
        glRectf(1 - border, 0, 1, 1)
    
class Eye6View:
    def __init__(self, data, bounds, color=(0.5, 0.8, 0.7)):
        self.bounds = bounds

        w = (int) (math.sqrt(len(data)))
        assert w * w == len(data)
        
        self.w = w
        self.h = w
        self.data = data
        self.color = color
        
        self.border = 0.05
        self.rects = []
        
        dw = 0.9 / (2 * self.w + 1)
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
        
        data = self.data
        
        for j in range(h):
            y = x0 + dy * (h - j - 1)
            x1 = x0 + dx * (j % 2)
            
            for i in range(w):
                grey = data[j * w + i]
                
                glColor3f(grey, grey, grey)
                 
                x = x1 + 2 * dx * i
                
                glRectf(x, y, x + 2 * dx, y + dy)

        color = self.color
        glColor3f(color[0], color[1], color[2])
        glRectf(0, 1 - border, 1, 1)
        glRectf(0, 0, 1, border)
        glRectf(0, 0, border, 1)
        glRectf(1 - border, 0, 1, 1)
