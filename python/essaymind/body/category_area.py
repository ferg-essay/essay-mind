import numpy as np
import logging

from essaymind import BodyNode
from essaymind import Fiber

log = logging.getLogger("Category")

class Category:
    def __init__(self, name, index):
        self.name = name
        self.index = index
        
    def __str__(self):
        return f"{self.name}[{self.index}]"

class CategoryNode(BodyNode):
    def __init__(self, parent, name, category, ticks_max=1):
        super().__init__(parent, name)
        assert isinstance(category, Category)
        
        self.category = category
        
        ticks_max = max(1, ticks_max)
        
        self.ticks_max = ticks_max
        
        self._on_active = None
    
    def on_active(self):
        assert not self.area.is_build()
        
        if not self._on_active:
            self._on_active = self.area.nexus(self.name + '.on_active')
            
        return self._on_active
        
    def when(self, fiber):
        assert not self.area.is_build()
        
        fiber.to(self)
        
        return self
    
    def to(self, target):
        self.on_active().to(target)
        
        return self
        
    def build(self):
        self.value = 0
        self.ticks = 0
        
    def send(self):
        self.ticks = self.ticks_max
        
    def send_p(self, p):
        self.send()
        
    def __call__(self):
        self.send()
        
    def tick(self):
        self.value = self.ticks > 0
        #if self.value:
        #    log.info(f"{self} {self.value} {self.ticks}")
        self.ticks = max(0, self.ticks - 1)
        self.area.data[self.category.index][self.index] = self.value
        
        if self.value and self._on_active:
            self._on_active()
        
    def __str__(self):
        return f"{self.name}[{self.value},{self.category}]"
    
class CategoryArea(BodyNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)

        self.categories = []
        
    def add_node(self, name, node):
        super().add_node(name, node)
        
        if not isinstance(node, CategoryNode):
            return
        
        while len(self.categories) <= node.category.index:
            self.categories.append([])
            
        node_list = self.categories[node.category.index]
                
        node_list.append(node)
        
        node.index = len(node_list) - 1
    
    def build(self):
        super().build()
        
        h = len(self.categories)
        w = 1
        for category in self.categories:
            w = max(w, len(category))
            
        self.h = max(1, h)
        self.w = max(1, w)
        
        self.data = np.zeros((self.h, self.w))
        
    def get_active_mood_names(self):
        names = []
        
        if not self.is_build:
            return
        
        for category in self.categories:
            for mood in category:
                if mood.value:
                    names.append(mood.name)
                    
        return names
    
    def __str__(self):
        names = self.get_active_mood_names()
        
        return f"{self.full_name}-{type(self).__name__}{names}" 