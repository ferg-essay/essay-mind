import logging
from essaymind import BodyNode, FiberKeyValue

log = logging.getLogger("Valence")

class TaskValence(BodyNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._targets = []
        
        self._active = False
        
        self._on_succeed = FiberKeyValue()
        self._on_fail = FiberKeyValue()
        
        self._context_map = dict()
        
        self._context_default = self.context('default')

    # build
    
    def on_succeed(self):
        assert not self.is_build()
        
        return self._on_succeed
    
    def to_succeed(self, target):
        self.on_succeed().to(target)
        
        return self
    
    def on_fail(self):
        assert not self.is_build()
        
        return self._on_fail
    
    def to_fail(self, target):
        self.on_fail().to(target)
        
        return self
    
    def context(self, name):
        assert not self.is_build()
        context = self._context_map.get(name)
        if not context:
            context = TaskContext(self, name)
            self._context_map[name] = context
            
        return context
    
    def success(self, name):
        self._context_default.success(name)
        
        return self
    
    def failure(self, name):
        self._context_default.failure(name)
        
        return self
    
    def when_context(self, fiber):
        assert not self.is_build()
        
        fiber.to(self.choose_context)

        return self
    
    def when_sense(self, fiber):
        assert not self.is_build()
        
        fiber.to(self.sense)

        return self
    
    def build(self):
        super().build()
        
        self._context = self._context_default
        
    # active
    
    def choose_context(self, key, value, p):
        log.info(f"{self.name} context ({key},{value},{p})")
        
        self._context = self._context_map(key)
    
    def sense(self, key, value, p):
        self._context.sense(key, value, p)
        
    def succeed(self, key, value, p):
        log.info(f"{self} succeed ({key},{value},{p})")
        
        self._on_succeed(key, value, p)
        
    def fail(self, key, value, p):
        log.info(f"{self} fail ({key},{value},{p})")
        
        self._on_fail(key, value, p)
        
    def __str__(self):
        return f"{self.name}-TaskValence({self._context.name})"
        

class TaskContext:
    def __init__(self, parent, name):
        self.parent = parent
        self.name = name
        
        self._sense_map = dict()

    # build
    
    def success(self, name):
        assert not self.parent.is_build()
        
        self._sense_map[name] = self.parent.succeed
        
        return self
    
    def failure(self, name):
        assert not self.parent.is_build()
        
        self._sense_map[name] = self.parent.fail
        
        return self
        
    # active
        
    def sense(self, key, value, p):
        target = self._sense_map.get(key)
        
        if target:
            target(key, value, p)
