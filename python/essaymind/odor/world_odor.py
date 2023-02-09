import math
import logging

from essaymind.world.world import WorldObject

log = logging.getLogger("Odor")

class Odor(WorldObject):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, attrs)

        self.name = odor_name
        self.attrs['name'] = odor_name
        self.value = 1
        
    def calculate_value(self, x, y):
        # currently point source, but future odors might not be point sources
        
        dx = (x - self.loc[0])
        dy = (y - self.loc[1])
        
        d2 = dx * dx + dy * dy
        
        if d2 <= 1:
            return self.value
        else:
            return self.value / d2 
        
    def calculate_dir(self, x, y):
        rad = math.atan2((self.loc[1] - y), (self.loc[0] - x))
        
        return ((-rad / (2 * math.pi)) + 1.25) % 1.0
    
    def tick(self, body):
        log.info(f"tick {self}")
        body['nose'].odor(self)
    
    def __str__(self):
        return f'Odor-{self.name}[]'
