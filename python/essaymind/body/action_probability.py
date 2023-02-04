import logging

from essaymind.core.node import MindNode
from essaymind import FiberKey, FiberKeyValue

log = logging.getLogger("Action")
        
class ActionProbabilityGroup(MindNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._ticks = self.top.config.get("theta", 10)
        self._bias = 0
        
        self._on_action_copy = FiberKeyValue()
        self._on_idle = FiberKey()
        self._actions = []
        
    def ticks(self, ticks):
        assert not self.is_build()
        assert 0 < ticks
        
        self._ticks = ticks
        
        return self
        
    def bias(self, bias):
        assert not self.is_build()
        assert 0 <= bias and bias <= 1
        
        self._bias = bias
        
        return self
    
    def on_action_copy(self, target):
        assert not self.is_build()
        self._on_action_copy.to(target)
        
        return self
    
    def on_idle(self, target):
        assert not self.is_build()
        self._on_idle.to(target)
        
        return self
    
    def action(self, action):
        assert not super().is_build()
        
        item = ActionItem(self, action)
        
        action.on_complete(self.complete)
        
        self._actions.append(item)
        
        return item
    
    def build(self):
        super().build()
        
        for item in self._actions:
            item.build()
        
        self.next_actions = []
        self._enable = False
        self._action = None
        self._random = self.top.random
        
    # active methods
    
    def request(self, action):
        self.next_actions.append(action)
        
    def enable(self):
        self._enable = True
        
    def complete(self, key, value, p):
        log.debug(f"complete {key} {value} {p}")
        self._action = None
        
    def tick(self, ticks):
        if self._action:
            self._on_action_copy(self._action.name, 1, 0)
            return
        
        if not self._enable:
            self._on_idle("idle")
            return
        self._enable = False
        
        self._action = action = self.select()
        
        if action:
            action.start()
            self._on_action_copy(self._action.name, 1, 0)
            self.top.action_name(action.full_name)

    def select(self):
        best_action = self.select_impl()
        
        for item in self._actions:
            item.clear()
        
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
        best_value = 0
        
        for item in self._actions:
            weight = item._excite - (1 - factor) * item._inhibit

            if weight > 0:
                value = self._random(weight)

                if best_value <= value:
                    best_action = item._action
                    best_value = value
        
        return best_action

    def probe(self, factor, test):
        assert 0 <= factor and factor <= 1
        
        for item in self._actions:
            weight = item._excite - factor * item._inhibit
            
            test(item.name, weight)
    
class ActionItem:
    def __init__(self, group, action):
        self._group = group
        self._action = action
        self.name = action.name
        
        assert action.start
        
    def build(self):
        self._excite = 0
        self._inhibit = 0
        self._tick_excite = 0
        self._tick_inhibit = 0
        
    # active methods
    
    def clear(self):
        self._excite = 0
        self._inhibit = 0
    
    def excite(self, value):
        assert 0 <= value and value <= 1
        
        if self._excite <= 0:
            self._group.enable()
        
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

class ActionNode(MindNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        assert (isinstance(parent, ActionProbabilityGroup))
        
        self.group = parent
        
        self._on_action_copy = FiberKeyValue()
        self._on_complete = FiberKeyValue()
        
        self._item = parent.action(self)
        self._ticks = None
        
        self._key = name
        self._value = 1
        
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
    
    def on_complete(self, target):
        self._on_complete.to(target)
    
    def action_fiber(self):
        assert not self.is_build()
        
        return self._on_action_copy
    
    def to(self, action):
        self.action_fiber().to(action)
        
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
    
    def excite(self, value):
        self._item.excite(value)
    
    def inhibit(self, value):
        self._item.inhibit(value)
        
    def start(self):
        self._active = self._ticks
        
    def stop(self):
        self._active = 0

    def tick(self, tick):
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
            self.stop()
            self._on_complete(self._key, self._value, 0)
        
    def action(self, value):
        self._on_action_copy.send(self._key, value, 1)
