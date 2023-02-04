class Fiber:
    def __init__(self, name='fiber'):
        self.name = name
        
        self.targets = []
        
    # build methods
        
    def to(self, target):
        assert target
        
        self.targets.append(target)
        
        return self
        
    def when(self, source):
        source.to(self)
        
        return self
        
    # active methods
    
    def send(self):
        self.__call__()
            
    def __call__(self):
        for target in self.targets:
            target()

class FiberProbability(Fiber):
    def __init__(self, name='fiber'):
        super().__init__(name)
    
    def send(self, p):
        self.__call__(p)
            
    def __call__(self, p):
        for target in self.targets:
            target(p)
        
class FiberObject(Fiber):
    def __init__(self, name='fiber'):
        super().__init__(name)
    
    def send(self, obj, p):
        self.__call__(obj, p)
            
    def __call__(self, obj, p):
        for target in self.targets:
            target(obj, p)
        
class FiberKey(Fiber):
    def __init__(self, name='fiber'):
        super().__init__(name)
    
    def send(self, key):
        self.__call__(key)
            
    def __call__(self, key):
        for target in self.targets:
            target(key)
        
class FiberKeyValue(Fiber):
    def __init__(self, name='fiber'):
        super().__init__(name)
    
    def send(self, key, value, p):
        self.__call__(key, value, p)
            
    def __call__(self, key, value, p):
        for target in self.targets:
            target(key, value, p)
        
class FiberAngle(Fiber):
    def __init__(self, name='fiber'):
        super().__init__(name)
    
    def send(self, key, angle):
        self.__call__(key, angle)
            
    def __call__(self, key, angle):
        for target in self.targets:
            target(key, angle)

class FiberType:
    def __init__(self):
        self.type = 0

class FiberSingle:
    def __init__(self):
        return

class FiberBuilder:
    def __init__(self):
        return

    