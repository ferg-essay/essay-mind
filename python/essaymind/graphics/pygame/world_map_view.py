import logging
import pygame
from pygame.locals import *

from essaymind import World, WorldActor
from .pygame_loop import PyGameLoop

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

class Viewport:
    def __init__(self, pygame, window, view, border='0.05'):
        self.pygame = pygame
        self.window_bounds = window
        self.x0 = window[0]
        self.x1 = window[2]
        self.y0 = window[1]
        self.y1 = window[3]
        
        self.dx = 0.9 / view[0]
        self.dy = 0.9 / view[1]

class PathView:
    def __init__(self, world_view):
        self._world_view = world_view
        self._points = []
        
    def add(self, x, y):
        self._points.append((self._world_view.to_x(x), self._world_view.to_y(y)))
        
    def draw(self):
        color = (50, 240, 240)
        
        if len(self._points) > 2:
            pygame.draw.lines(self._world_view.surface, color, False, self._points) 
    
class WorldMapView:
    def __init__(self, window, world, bounds):
        assert isinstance(window, PyGameLoop)
        assert isinstance(world, World)
        
        self.window = window
        self.surface = self.window.surface
        assert self.surface
        
        self.world = world
        world_map = world.world_map
        self.world_map = world_map
        self.border = 10
        self.bounds = bounds
        self.rects = []
        self.actors = []
        self._paths = []
        self._path_map = dict()
        
        self.width = self.bounds[2]
        self.height = self.bounds[3]
        
        dw = (self.width - 2 * self.border) / world_map.width
        dl = (self.height - 2 * self.border) / world_map.length
        
        self.dx = dw
        self.dy = dl
        self.x0 = self.bounds[0] + self.border
        self.y0 = self.bounds[1] + self.height - self.border
        
        self.color = (255, 255, 255)
        
        for j in range(world_map.length):
            for i in range(world_map.width):
                value = world_map[(i, j)]
                
                if value:
                    self.rects.append(pygame.Rect(self.to_x(i), self.to_y(j) - dl, dw, dl))
                    
    def add_actor(self, actor):
        assert isinstance(actor, WorldActor)
        
        self.actors.append(actor)
        
    def to_x(self, i):
        return self.x0 + i * self.dx
    
    def to_y(self, j):
        return self.y0 - j * self.dy
        
    def path(self, name):
        path = self._path_map.get(name)
        if not path:
            path = PathView(self)
            self._paths.append(path)
            self._path_map[name] = path
            
        return path
    
    def render(self):
        #self.viewport()
        
        #bounds = self.bounds
        #glViewport(bounds[0] - bounds[2], bounds[1] - bounds[3], 2 * bounds[2], 2 * bounds[3])
        
        border = self.border
        x0, y0, width, height = self.bounds
        
        self.color = (200, 180, 100)
        
        self.draw_rect(x0, y0, border, height)
        self.draw_rect(x0 + width - border, y0, border, height)
        
        self.draw_rect(x0, y0, width, border)
        self.draw_rect(x0 , y0 + height - border, width, border)
        
        #glColor3f(0.7, 0.6, 0.3)
        for rect in self.rects:
            pygame.draw.rect(self.surface, self.color, rect)
            
        for actor in self.actors:
            self.draw_actor(actor)
            
        for path in self._paths:
            path.draw()
            
        return
            
    def draw_rect(self, x0, y0, width, height):
        pygame.draw.rect(self.surface, self.color,
                         pygame.Rect(x0, y0, width, height))
            
    def draw_path(self, path):
        border = self.border
        dx = self.dx
        dy = self.dy
        
        x0 = border + path[0][0] * dx 
        y0 = border + path[0][1] * dy
        
        for point in path:
            x1 = border + point[0] * dx 
            y1 = border + point[1] * dx
            
    def draw_actor(self, actor):
        x0 = actor.x
        y0 = actor.y
        
        color = (230, 50, 50)
        
        dx = actor.dir_dx * 0.25
        dy = actor.dir_dy * 0.25
        
        to_x = self.to_x
        to_y = self.to_y
        
        pygame.draw.polygon(self.surface, color,
                            ((to_x(x0 + dx), to_y(y0 + dy)),
                             (to_x(x0 - dx - 0.5 * dy), to_y(y0 - dy + 0.5 * dx)),
                             (to_x(x0 - dx + 0.5 * dy), to_y(y0 - dy - 0.5 * dx))))
