from essaymind.body.body import MindNode
#from symbol.event.event_navigation import EventNavigation
import logging

log = logging.getLogger("Amy")

class AmyMemory(MindNode):
    def __init__(self, body):
        self.body = body
        # self.nav = EventNavigation(body, self)
        #self.mood = EventMood(body, self)
        self.body.add_node('amy.memory', self)
        self.node_map = dict()
        self.sources = []
        
    def add_node(self, name, target):
        assert not self.node_map.get(name)
        
        node = AmyNode(self, name, target)
        self.node_map[name] = node
        
        return node
    
    def add_source(self, source):
        self.sources.append(source)
        
    def __getitem__(self, name):
        return self.node_map[name]
    
    def build(self, body):
        for source in self.sources:
            for node in self.node_map.values():
                node.add_source(source)
    
    '''
    def ensure_range(self, index):
        self.nav.ensure_range(index)
    
    def get_node(self, index):
        return self.nav.get_node(index)
    '''

class AmyNode(MindNode):
    def __init__(self, amy, name, target):
        self.amy = amy
        self.body = amy.body
        self.name = name
        self.target = target
        self.body.add_node("amy.node." + name, self)
        
        self.sources = []
        self.dict = {}
        
        self.senses = []
        
    def add_source(self, source):
        self.sources.append(source)
        
        source.add_target(self)
        
    def add_node(self, name, value):
        node = EventMoodNode(self.mem, len(self.dict), value)
        self.dict[name] = node
        node[name] = value
        return node
    
    def sensor(self, obj, name, value):
        log.debug(f"{self.name} sense {name} {value} {obj}")
        self.senses.append(SenseValue(name, value))
    
    def __setitem__(self, name, value):
        self.dict[name] = value
        
    def get_node(self, name):
        return self.dict[name]
    
    def tick(self, body):
        senses = self.senses
        
        if not len(senses):
            return
        
        self.senses = []
        
        hash_value = None
        for item in senses:
            if hash_value:
                hash_value = hash_value + '-' + item.name
            else:
                hash_value = item.name
        
        sense_value = self.dict.get(hash_value)
        if sense_value:
            log.info(f"{hash_value} -> {sense_value} {self.target}")
            self.target.sensor(hash_value, 0.25)
            self.target.unsuppress(1)
    
class SenseValue:
    def __init__(self, name, value):
        self.name = name
        self.value = value    
    
class EventMoodNode(MindNode):
    def __init__(self, mem, index, odor_name):
        self.mem = mem
        self.index = index
        self.name = odor_name
        self.key_map = dict()
        
        mem.body.add_node("event.mood." + odor_name, self)
        self.pattern_index = 0
        self.obj = None
        self.pattern = None
        self.is_enable = False
        
    def build(self, body):
        self.body = body
        self.nav = self.mem.nav
        assert self.nav
        
    def set_enable(self, is_enable):
        self.is_enable = is_enable
        
    def __setitem__(self, pattern, key):
        self.key_map[pattern] = key
        
    def get_node(self, index):
        return self
        
    def sensor(self, index, pattern, value, obj):
        log.info(f'Sensor.{self.index}.{self.name} {pattern}->{index} {value:.2f} {obj}')

        self.pattern_index = index
        self.pattern = pattern
        self.obj = obj

    def tick(self):
        pattern = self.pattern
        self.pattern = None
        
        is_enable = self.is_enable
        self.is_enable = False
        
        if pattern:
            key = self.key_map.get(pattern)
            
            if key:
                log.info(f'Key-M.{self.index}.{self.name} {key} {is_enable} {pattern}->{self.pattern_index}')
            
    def __str__(self):
        return f'EventMood({self.index},{self.name})'
    