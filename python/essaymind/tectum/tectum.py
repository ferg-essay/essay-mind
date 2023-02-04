import logging

from essaymind.body import MindNode
from essaymind.fiber.fiber import FiberAngle
from essaymind.body.action_probability import ActionProbabilityGroup
from essaymind.motion.move_probability_actions import MoveNode

log = logging.getLogger("Tectum")

class Tectum(MindNode):
    '''
    Model of the midbrain tectum as a topographic locomotive map.
    
    The abstract tectum doesn't necessarily include optic input, but can have topographic
    senses like touch or odor.
    '''

    def __init__(self, parent, name='tectum'):
        super().__init__(parent, name)
        
        self.count(6)
        self._toward = []
        self._away = []
        
        self.action_group = ActionProbabilityGroup(self, "actions")
        
    def count(self, count):
        assert not self.is_build()
        assert count > 1
        
        self._count = count
        self._arc = 1.0 / count
        self._arc2  = 0.5 * self._arc
        
        return self
    
    def move_ticks(self, ticks):
        self.action_group.ticks(ticks)
        
        return self
    
    def when_toward(self, name, fiber):
        assert not self.is_build()
        assert isinstance(fiber, FiberAngle)
    
        toward = Toward(self, name, fiber)    
        self._toward.append(toward)
    
    def when_away(self, name, fiber):
        assert not self.is_build()
        assert isinstance(fiber, FiberAngle)
    
        away = Away(self, name, fiber)    
        self._away.append(away)
        
    def build_items(self):
        self._actions = []
        
        self._items = []
        self._dir_items = []
        for i in range(2 * self._count):
            item = Item(self, i)
            
            self._items.append(item)
        
    def build_actions(self):
        self.action_group.on_idle(self.action_idle)
    
    def build(self):
        super().build()
        
        self.build_items();
        self.build_actions();
        
        self._is_action_idle = False
        
    def action_idle(self, key):
        self._is_action_idle = True
        
    def item(self, angle):
        assert 0 <= angle and angle <= 1
        
        i = (int) (angle / self._arc2)
        
        return self._items[i]
    
    def tick(self, ticks):
        is_idle = self._is_action_idle
        self._is_action_idle = False
        
        if is_idle:
            self.calculate_action()
            
    def calculate_action(self):
        is_action = 0
        for item in self._items:
            is_action += item.calculate_action()
            
        if not is_action:
            log.info("idle")
            
     
class Toward:
    '''
    Input from an attractive source.
    '''
    def __init__(self, tectum, name, fiber):
        self._tectum = tectum
        self._name = name
        self._i = len(tectum._toward)
        
        fiber.to(self.sense)
        
    def sense(self, key, angle):
        item = self._tectum.item(angle)
        item.toward(self._i, key)
     
class Away:
    '''
    Input from a threat source.
    '''
    def __init__(self, tectum, name, fiber):
        self._tectum = tectum
        self._name = name
        self._i = len(tectum._away)
        
        fiber.to(self.sense)
        
    def sense(self, key, angle):
        item = self._tectum.item(angle)
        item.away(self._i, key)
        
class Item:
    def __init__(self, tectum, i):
        self._tectum = tectum
        self._i = i
        
        self._to_angle = to_angle = self.to_angle(i)
        
        away_angle = self.away_angle(i)
        
        self.angle = to_angle
        
        self._toward = [None] * len(self._tectum._toward)
        self._away = [None] * len(self._tectum._away)
        
        self.move_to = self.add_move("to", to_angle)
        self.move_away = self.add_move("away", away_angle)
        
        #log.info(f"{self} {i} {to_angle:.2f} {away_angle:.2f} {self.move_to} {self.move_away}")
            
    def to_angle(self, i):
        count2 = 2 * self._tectum._count
        arc2 = 1 / count2
        return i * arc2 if i % 2 == 0 else ((i + 1) % count2) * arc2
            
    def away_angle(self, i):
        count = self._tectum._count
        count2 = 2 * count
        
        j = (i + count) % count2
        j += (j % 2 + 1) % 2

        angle = j * 0.5 / count
        
        if 0.25 < angle and angle <= 0.5:
            angle = 0.25
        elif 0.5 <= angle and angle <= 0.75:
            angle = 0.75
            
        return angle
            
    def add_move(self, name, angle):
        name = name + ".%.0f" % round(angle * 100)
        move = self._tectum.action_group.node(name)
        if move:
            return move
        else:
            return MoveNode(self._tectum.action_group, name, angle)
        
    def toward(self, i, key):
        self._toward[i] = key
        
    def away(self, i, key):
        self._away[i] = key
        
    def calculate_action(self):
        value = 0
        
        for i, item in enumerate(self._away):
            if item:
                value = min(0, value) - 1
                self._away[i] = None
                
        for i, item in enumerate(self._toward):
            if item:
                value += 1
                self._toward[i] = None
                
        if value < 0:
            self.move_away.excite(max(1, value))
            
            return True
        elif value > 0:
            self.move_to.excite(max(1, value))
            
            return True
        else:
            return False
        
    def __str__(self):
        return f"{self._tectum.name}-Item.{self._i}[{self._to_angle:.2f}]"