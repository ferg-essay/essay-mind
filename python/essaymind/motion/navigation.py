from essaymind.core.node import MindNode
 
import logging

log = logging.getLogger("Nav")

class Triangulate(MindNode):
    def __init__(self, source):
        self.source = source
        self.value_theta = 0
        
    def build(self, body):
        self.body = body
        self.value_theta = self.source.get_value()

    def tick(self):
        data = self.source.get_value()
        
        data_old = self.value_theta
        
        delta = data - data_old
        self.value = delta
        
        if self.source.avoid_value() <= 0:
            return
        
        if delta > 0:
            self.body.turn_request(1)
        elif delta > -0.1:
            self.body.turn_request(0.25)
        
        log.info(f"delta {delta} {data_old} phase {self.body.theta_phase} dir {self.body.dir} speed {self.body.speed}")
        return
    
    def tick_theta(self):
        self.value_theta = self.source.value

class TriangulateNode(MindNode):
    def __init__(self, odor_name, source):
        self.name = odor_name
        self.source = source
        self.value_theta = 0
        
    def build(self, body):
        self.body = body
        self.value_theta = self.source.get_value()

    def tick(self):
        data = self.source.get_value()
        
        data_old = self.value_theta
        
        delta = data - data_old
        self.value = delta
        
        # Is there value in learned/fuzzy logic, as opposed to hard-coded calculation?
        if delta > 0:
            direction = 1
        else:
            direction = 0
        
        head_direction = self.body.get_head_direction()
        
        log.info(f"node-delta {self.name} {delta} {data_old} phase ego-dir {direction} head-dir {head_direction} speed {self.body.speed}")
        return
    
    def tick_theta(self):
        self.value_theta = self.source.value

class NavigationMux(MindNode):
    def __init__(self, body):
        self.body = body
        body.add_node('ori.triangulate.mux')
        self.name = 'ori.tri'
        self.null_source = NullSource()
        self.sinks = []
        
    def add_sink(self, name):
        self.sinks.append(NavigationSource(name, len(self.sinks)))
        
    def build(self, body):
        self.ensure_target(0)
             
        for sink in self.sinks:
            sink.build(body)
        
    def get_node(self, i):
        self.ensure_target(0)
            
        return self.sinks[i % len(self.sinks)]
    
    def ensure_target(self, index):
        while len(self.sinks) <= index:
            self.sinks.append(NavigationSource(len(self.sinks)))
    
    def tick(self):
        for sink in self.sinks:
            sink.tick()
    
    def tick_theta(self):
        for sink in self.sinks:
            sink.tick_theta()
        
class NullSource(MindNode):
    def __init__(self):
        return
    
class NavigationSource(MindNode):
    def __init__(self, index):
        self.index = index
        self.source_name = None
        self.is_active = False
        self.in_data = 0
        self.data_theta = 0
        self.delta_theta = 0
        self.delta2 = 0
        
    def build(self, body):
        self.body = body
        self.locomotion = body.node("locomotion.turn")
        assert self.locomotion
        
    def is_empty(self):
        return not self.is_active
    
    def sensor(self, name, value, obj):
        #print(f'Sensor {name} {value} {obj} {self.source}')
        self.in_data = value
        self.source_name = name
        self.obj = obj

    def tick(self):
        #in_data = self.source.get_value()
        data = self.in_data
        obj = self.obj
        self.obj = None
        
        data_old = self.data_theta
        
        delta = data - data_old
        self.value = delta
        
        self.delta = delta
        
        # Is there value in learned/fuzzy logic, as opposed to hard-coded calculation?
        if delta > 0:
            dir_ego = 0.5
        else:
            dir_ego = 0
            
        self.dir_ego = dir_ego
        dir_head = self.body.get_head_direction()
        
        dir_allo = (dir_ego + dir_head) % 1
        
        if obj:
            log.info(f"Object.{self.index}({self.source_name}) dir_ego={dir_ego} dir_allo={dir_allo} delta={delta:.2g} d2={self.delta2:.2g} data_old={data_old:.2f} phase dir-head {dir_head} speed {self.body.speed}")
            
            self.locomotion.turn_request(self.source_name, dir_ego, data)
        return
    
    def tick_theta(self):
        #self.value_theta = self.source.value
        self.data_theta = self.in_data
        old_delta = self.delta_theta
        self.delta_theta = self.delta
        
        delta2 = self.delta - old_delta
        self.delta2 = delta2
        
        # Is there value in learned/fuzzy logic, as opposed to hard-coded calculation?
        if self.delta > 0:
            direction = 0.5
        else:
            direction = 0
        
        log.info(f"node-theta-delta.{self.index} {self.source_name} d2={delta2} d={old_delta} dir={direction}")


class TriangulateApproach(MindNode):
    def __init__(self, source):
        self.source = source
        self.value_theta = 0
        
    def build(self, body):
        self.body = body
        self.value_theta = self.source.get_value()

    def tick(self):
        data = self.source.get_value()
        
        data_old = self.value_theta
        
        delta = data - data_old
        self.value = delta
        
        if self.source.approach_value() <= 0:
            return
        
        if delta < 0:
            self.body.turn_request(1)
        elif delta < 0.1:
            self.body.turn_request(0.25)
        
        log.info(f"delta {delta} {data_old} phase {self.body.theta_phase} dir {self.body.dir} speed {self.body.speed}")
        return
    
    def tick_theta(self):
        self.value_theta = self.source.get_value()
