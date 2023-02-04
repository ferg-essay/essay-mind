import logging

from essaymind import BodyNode
from essaymind import FiberKeyValue

log = logging.getLogger("Action")
        
class ActionGroup(BodyNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._ticks = self.top.config.get("theta", 10)
        
        self._on_start = FiberKeyValue()
        self._on_action_copy = FiberKeyValue()
        self._on_idle = FiberKeyValue()
        
        self._actions = []
        
    def ticks(self, ticks):
        assert not self.is_build()
        assert 0 < ticks
        
        self._ticks = ticks
        
        return self
    
    def on_start(self):
        assert not self.is_build()
        return self._on_start
    
    def on_action_copy(self, target):
        assert not self.is_build()
        
        self._on_action_copy.to(target)
        
        return self
    
    def on_idle(self, target):
        assert not self.is_build()
        
        self._on_idle.to(target)
        
        return self
    
    def to(self, target):
        self.on_start().to(target)
        
        return self
    
    def add(self, action):
        assert not self.is_build()
        assert isinstance(action, ActionNode)
        
        self.actions.append(action)
        
        return self
    
    def build(self):
        super().build()
        
        self.next_actions = []
        self._action = None
        
    # active methods
    
    def request(self, action):
        self.next_actions.append(action)
        
    def complete(self, action):
        self._action = None
        
    def tick(self):
        if self._action:
            return
        
        if not self.next_actions:
            self._on_start.send(self.name, 0, 1)
            return
        
        self._action = action = self.select()
        if action:
            action.start()
            self._on_start.send(self.name, 1, 1)
        else:
            self._on_start.send(self.name, 0, 1)
        

    def select(self):
        best_action = self.select_impl()
        
        for action in self.next_actions:
            action.clear()
                
        self.next_actions = []
        
        return best_action
        
    def select_impl(self):
        for factor in range(5):
            action = self.select_factor(factor / 4)
        
            if action:
                return action
            
        return None

    def select_factor(self, factor):
        assert 0 <= factor and factor <= 1
        best_action = None
        best_weight = 0
        
        for action in self.next_actions:
            weight = action._excite - (1 - factor) * action._inhibit

            if best_weight <= weight:
                best_action = action
                best_weight = weight
        
        return best_action 

class ActionNode(BodyNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        assert isinstance(parent, ActionGroup)
        
        self.group = parent
        self._ticks = None
        
        self._key = name
        self._value = 1
        
        self._on_action_copy = FiberKeyValue()
        
    def key(self, key):
        assert key
        
        self._key = key
        
        return self
        
    def value(self, value):
        assert 0 <= value and value <= 1
        
        self._value = value
        
        return self
        
    def ticks(self, ticks):
        assert not self.is_build()
        assert 0 < ticks
        
        self._ticks = ticks
        
        return self
    
    def on_action(self):
        assert not self.is_build()
        
        return self._on_action_copy
    
    def to(self, action):
        self.on_action().to(action)
        
        return self
        
    def build(self):
        super().build()
        
        if self._ticks == None:
            self._ticks = self.group._ticks
            
        self._active = 0
        
        self._excite = 0
        self._tick_excite = 0
        
        self._inhibit = 0
        self._tick_inhibit = 0
        
    # active methods
    
    def clear(self):
        self._excite = 0
        self._inhibit = 0
    
    def excite(self, value):
        assert 0 <= value and value <= 1
        if self._excite <= 0:
            self.group.request(self)
        
        self._tick_excite = max(self._tick_excite, value)
        self._excite = self._tick_excite
    
    def on_excite(self, key, value, p):
        self.excite(value)
        
        log.debug(f"excite {self} ({key},{value:.3g},{p})")
    
    def inhibit(self, value):
        assert 0 <= value and value <= 1
        
        self._tick_inhibit = max(self._tick_inhibit, value)
        self._inhibit = self._tick_inhibit
    
    def on_inhibit(self, key, value, p):
        self.inhibit(value)
        
        log.debug(f"inhibit {self} ({key},{value:.3g},{p})")
        
    def __call__(self, key, value, p):
        self.on_excite(key, value, p)
        
    def start(self):
        self._active = self._ticks
        
    def stop(self):
        self._active = 0

    def tick(self):
        self._tick_excite = 0
        self._tick_inhibit = 0
        
        if self._active <= 0:
            return
        
        if self._active == self._ticks:
            self.action(1)
        else:
            self.action(0)
        
        self._active -= 1
        
        if self._active <= 0:
            self.group.complete(self)
            self.stop()
        
    def action(self, value):
        self._on_action_copy.send(self._key, value, 1)
    
class ActionSource(BodyNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._targets = []
        
        self._active = False

    # build
    
    def target(self, target, value):
        assert not self.is_build()
        self._targets.append((target, value))

        return self
    
    def when_activate(self, fiber):
        assert not self.is_build()
        
        fiber.to(self.activate)

        return self
        
    # active
        
    def activate(self, key, value, p):
        self._active = True
        
    def __call__(self, key, value, p):
        self.activate(key, value, p)
        
    def tick(self):
        if not self._active:
            return
        self._active = False
        
        for target, value in self._targets:
            target(self.name, value, 1)
