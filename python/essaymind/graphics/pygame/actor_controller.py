import logging

from pygame.locals import *

from essaymind import WorldActor
from .pygame_loop import PyGameController

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
                
class ActorController(PyGameController):
    def __init__(self, actor):
        assert isinstance(actor, WorldActor)
        
        self.actor = actor
        self.tickers = []
        
        self.speed_request = 0.1
        
        self.is_auto_tick = False
        self.is_space = False
        
    def add_ticker(self, ticker):
        self.tickers.append(ticker)
        
    def stop_ticker(self):
        self.is_auto_tick = False
        
    def start_ticker(self):
        self.is_auto_tick = True
        
    def keydown(self, key):
        if key == K_SPACE:
            self.tick()
            self.is_auto_tick = False
        elif key == K_t:
            self.is_auto_tick = not self.is_auto_tick
            
    def keypressed(self, pressed_keys):
        speed = self.speed_request
               
        if pressed_keys[K_a]:
            self.actor.right(-speed)
        if pressed_keys[K_d]:
            self.actor.right(speed)
        
        if pressed_keys[K_w]:
            self.actor.forward(speed)
        if pressed_keys[K_s]:
            self.actor.forward(-speed)
                    
        if pressed_keys[K_q]:
            self.actor.turn(-1e-2)
        if pressed_keys[K_e]:
            self.actor.turn(1e-2)

    def post_update(self):
        if self.is_auto_tick:
            self.tick()
                
    def tick(self):
        for ticker in self.tickers:
            ticker.tick()
            
class NullTicker:
    def tick(self):
        return
