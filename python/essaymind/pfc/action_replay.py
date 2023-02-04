import logging

from essaymind.core.node import MindNode

log = logging.getLogger("Front")

class FrontAction(MindNode):
    '''
    Action-selection based on replay with E.mec/E.hc
    '''


    def __init__(self, parent, name='front_action'):
        super().__init__(parent, name)

    def actions(self, actions):
        assert not self.is_build()
        
        self._actions = actions
        
        return self
    
    def when_action_copy(self, node):
        assert not self.is_build()
        node.on_action_copy(self.action_copy)
        
        return self
    
    def when_action_idle(self, node):
        assert not self.is_build()
        node.on_idle(self.action_idle)
        
        return self
    
    def when_grid(self, node):
        assert not self.is_build()
        node.on_grid(self.grid_data)
        
        return self
    
    def build(self):
        super().build()
        
        self._is_idle = False
        self._idle_ticks = 0
    
    # active actions
    
    def action_idle(self, key, value, p):
        self._is_idle = True
    
    def action_copy(self, key, value, p):
        log.info(f"action {key}, {value}, {p}")
    
    def grid_data(self, key, value, p):
        log.info(f"grid {key}, {value}, {p}")
    
    def tick(self):
        super().tick()
        
        is_idle = self._is_idle
        self._is_idle = False
        
        if is_idle:
            if self._idle_ticks >= 2:
                log.info(f"idle action")
                for action in self._actions:
                    action.excite(0.5)
                self._actions[1].excite(0.6)
            self._idle_ticks += 1
        else:
            self._idle_ticks = 0
        
        
        
        
        return