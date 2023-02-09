import logging

from body.body import Body, BodyNode
from world.world_food import Food

log = logging.getLogger("Mouth")

class BodyMouth(BodyNode):
    def __init__(self, body):
        assert isinstance(body, Body)
        self.body = body
        self.world = body.world
        # body['body.mouth'] = self
        self.sense = MouthSense(body)
        self.eat = MouthEat(body)
        self.sick = MouthSick(body)
        
    def __str__(self):
        return f'Mouth[{self.sense.value},{self.eat.value}]'
    
class MouthSense(BodyNode):
    def __init__(self, body):
        self.body = body
        body['mouth.sense'] = self
        
        self.food = None
        self.value = 0
        
    def tick(self):
        obj = self.body.get_object()
        
        self.food = None
        self.value = 0
        
        #if isinstance(obj,Food):
        if obj:
            self.food = obj
            self.value = 1
            
            log.debug(f'mouth {obj} -> {self.value}')

    def __str__(self):
        return f'MouthSense[{self.value}]'        
        
class MouthEat(BodyNode):
    def __init__(self, body):
        self.body = body
        body['mouth.eat'] = self
        
        self.disgust = Disgust(body, self)
        self.bitter = Bitter(body, self)
        self.salty = Salty(body, self)
        self.savory = Savory(body, self)
        self.sour = Sour(body, self)
        self.sweet = Sweet(body, self)
        
        self.value = 0
        self.delay = 0
        self.delay_threshold = 4
        
    def build(self, body):
        self.sense = body['mouth.sense']
        self.sick = body['mouth.sick']
        
    def request_eat(self):
        self.value = 1
        
    def tick(self):
        value = self.value
        self.value = 0
        
        if value:
            self.delay += 1
            
            if self.delay < self.delay_threshold:
                return
            
            obj = self.body.get_object()
            
            if not obj:
                return

            self.body.add_action(f"eat-{obj}")
            if not isinstance(obj, Food):
                log.info(f'eat disgust non-food {obj}')
                self.value = None
                self.disgust.sense(1)
            elif obj.eat():
                log.info(f'eat {obj}')
                self.value = obj
                
                # taste
                if obj.is_bitter():
                    self.bitter.sense(1)
                if obj.is_salty():
                    self.salty.sense(1)
                if obj.is_savory():
                    self.savory.sense(1)
                if obj.is_salty():
                    self.salty.sense(1)
                if obj.is_sweet():
                    self.sweet.sense(1)
                    
                if obj.is_sick():
                    self.sick.request_sick()
            else:
                self.body.remove_object(obj)
                log.info(f'eat-finish {obj}')
        else:
            self.delay = 0

    def __str__(self):
        return f'MouthEat[{self.value}]'        
        
class MouthSick(BodyNode):
    def __init__(self, body):
        self.body = body
        body['mouth.sick'] = self
        
        self.is_sick = False
        self.delay = 0
        self.delay_sick_start = 20
        self.delay_sick_end = 30
        
    def build(self, body):
        self.disgust = body['mouth.taste.disgust']
        
    def request_sick(self):
        log.debug('sick request')
        self.is_sick = True
        
    def tick(self):
        is_sick = self.is_sick
        self.is_sick = False
        
        if not is_sick and self.delay == 0:
            return
        
        self.delay += 1
        
        if self.delay_sick_start <= self.delay:
            if not is_sick and self.delay_sick_end < self.delay:
                self.delay = 0
            
            log.info("sick")
            self.body.add_action("sick")
            self.disgust.sense(1)
            

    def __str__(self):
        return f'MouthSick[{self.value}]'        

class Taste(BodyNode):
    def __init__(self, odor_name, body, eat):
        self.eat = eat
        self.name = odor_name
        
        body['mouth.taste.' + odor_name] = self
        self.value = 0
        self.decay = 0.5
        self.sense_value = 0
        self.min_value = 0.1
        
    def sense(self, value):
        self.sense_value = value
        
    def tick(self):    
        sense_value = self.sense_value
        self.sense_value = 0
        
        self.value = max(sense_value, self.decay * self.value)
        
        if self.min_value <= self.value:
            log.info(f"{self.name} taste {self.value}")
        
class Disgust(Taste):
    def __init__(self, body, eat):
        super().__init__('disgust', body, eat)
        
class Bitter(Taste):
    def __init__(self, body, eat):
        super().__init__('bitter', body, eat)
        
class Salty(Taste):
    def __init__(self, body, eat):
        super().__init__('salty', body, eat)
        
class Savory(Taste):
    def __init__(self, body, eat):
        super().__init__('savory', body, eat)
        
class Sour(Taste):
    def __init__(self, body, eat):
        super().__init__('sour', body, eat)
        
class Sweet(Taste):
    def __init__(self, body, eat):
        super().__init__('sweet', body, eat)
