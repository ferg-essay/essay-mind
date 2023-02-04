import pygame
from pygame.locals import *

from OpenGL.GL import *
import logging

log = logging.getLogger("PyGame")
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

class PyGameView(object):
    def __init__(self, bounds):
        assert len(bounds) == 4
        
        self.bounds = bounds
        
    def render(self):
        return

class PyGameController(object):
    def pre_update(self):
        return
    
    def keydown(self, key):
        return
    
    def keypressed(self, pressed_keys):
        return
        
    def post_update(self):
        return
                        
class PyGameLoop(object):
    def __init__(self, bounds):
        assert len(bounds) == 2
        
        pygame.init()

        self.display = bounds
        self.surface = pygame.display.set_mode(bounds, DOUBLEBUF|OPENGL)
        
        self.views = []
        self.controllers = []
        self.tickers = []
        
    def add_view(self, view):
        self.views.append(view)
        
    def add_controller(self, controller):
        self.controllers.append(controller)
        
    def add_ticker(self, ticker):
        self.tickers.append(ticker)
        
    def stop(self, is_stop):
        self.is_active = not is_stop
        
    def pygame_loop(self):
        glEnable(GL_DEPTH_TEST)
        glDepthFunc(GL_LESS)
        
        self.is_active = True
        
        while self.is_active:
            self.update()
                
            glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT)
            
            for view in self.views:
                view.render()
            
            pygame.display.flip()
            pygame.time.wait(30)
            
    def update(self):
        for controller in self.controllers:
            controller.pre_update()
            
        for ticker in self.tickers:
            ticker.tick()
            
        pressed_keys = pygame.key.get_pressed()
        
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                quit()
            elif event.type == KEYDOWN:
                for controller in self.controllers:
                    controller.keydown(event.key)

        for controller in self.controllers:
            controller.keypressed(pressed_keys)
            
        for controller in self.controllers:
            controller.post_update()
