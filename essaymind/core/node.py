import logging

from .ticker import TickManager, Ticker

log = logging.getLogger("BodyNode")

class Buildable:
    def build(self):
        return  

class MindNode(Ticker, Buildable):
    def __init__(self, parent, name):
        assert isinstance(parent, MindNode)
        
        if not parent:
            parent = self

        self.parent = parent
        
        if parent == self:
            assert isinstance(self, MindNodeRoot)
            self.top = self
            self.side = self 
        else:
            self.top = parent.top
            
        if isinstance(self, MindNodeSide):
            self.side = self
        else:
            self.side = parent.side
            
        self.name = name
        self.full_name = full_name = self.get_full_name()
        
        self.top[full_name] = self
        
    # build methods
    
    def is_build(self):
        return self.top._is_build
    
    def prefix(self):
        if self == self.top:
            return ""
        else:
            return self.parent.prefix() + self.name + "."
    
    def get_full_name(self):
        if self == self.top:
            return self.name
        else:
            return self.parent.prefix() + self.name
            
    def node(self, name):
        return self.top.node(self.prefix() + name)
    
    def __getitem__(self, name):
        return self.top.__getitem__(self.prefix() + name)
    
    def __setitem__(self, name, node):
        self.top.__setitem__(self.prefix() + name, node)
    
    def __str__(self):
        return f"{self.full_name}-{type(self).__name__}[]"

class MindNodeSide(MindNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self.side = self
        
class MindNodeRoot(MindNodeSide):
    def __init__(self, name):
        self._is_build = False
        
        self.node_map = {}
        self.build_list = []
        self.ticker = TickManager()
        
        super().__init__(self, name)
        
        self.top = self
        
    def node(self, name):
        return self.node_map.get(name)
    
    def __getitem__(self, name):
        return self.node_map[name]
        
    def __setitem__(self, name, node):
        assert not self.is_build()
        assert not self.node_map.get(name)
        
        self.node_map[name] = node
        
        if isinstance(node, Buildable) and node != self:
            self.build_list.append(node)
        
        if isinstance(node, Ticker) and node != self:
            self.ticker.add_ticker(node)
    
    def build(self):
        assert not self._is_build
        
        i = 0
        while i < len(self.build_list):
            node = self.build_list[i]
            node.build()
            i += 1
            
        self._is_build = True
    
    def ticks(self):
        return self.ticker.ticks
        
    def tick(self):
        log.info(f"[{self.ticks():3d}] {self} tick")
        
        self.ticker.tick()
        