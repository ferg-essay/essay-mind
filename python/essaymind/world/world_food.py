from odor.world_odor import Odor

class Food(Odor):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, odor_name, attrs)
        assert isinstance(attrs, dict)

        self.ticks = 4
        attrs['food'] = True
    
    def eat(self):
        ticks = self.ticks
        
        if ticks > 0:
            self.ticks = ticks - 1
            return True
        else:
            return False
        
    def is_eaten(self):
        return self.ticks <= 0
    
    def is_bitter(self):
        return self.attrs.get('bitter')
    
    def is_salty(self):
        return self.attrs.get('salty')
    
    def is_savory(self):
        return self.attrs.get('savory')
    
    def is_sour(self):
        return self.attrs.get('sour')
    
    def is_sweet(self):
        return self.attrs.get('sweet')
    
    def is_sick(self):
        return self.attrs.get('sick')
    
    def get_taste(self):
        taste = []
        
        if self.is_bitter():
            taste.append('bitter')
            
        if self.is_salty():
            taste.append('salty')
            
        if self.is_savory():
            taste.append('savory')
            
        if self.is_sour():
            taste.append('sour')
            
        if self.is_sweet():
            taste.append('sweet')
            
        return taste
    
    def __str__(self):
        taste = self.get_taste()
        
        return f'Food-{self.name}{taste}'
    

class FoodBitter(Food):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, odor_name, attrs)

        attrs['bitter'] = True

class FoodSalty(Food):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, odor_name, attrs)

        attrs['salty'] = True

class FoodSavory(Food):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, odor_name, attrs)

        attrs['savory'] = True

class FoodSour(Food):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, odor_name, attrs)

        attrs['sour'] = True

class FoodSweet(Food):
    def __init__(self, loc, odor_name, attrs = {}):
        super().__init__(loc, odor_name, attrs)

        attrs['sweet'] = True

class FoodSick(Food):
    def __init__(self, loc, odor_name, attrs = {}, tastes=None):
        super().__init__(loc, odor_name, attrs)

        attrs['sick'] = True
        
        if tastes:
            for taste in tastes:
                attrs[taste] = True
