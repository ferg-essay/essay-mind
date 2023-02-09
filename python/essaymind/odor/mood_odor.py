import logging

from essaymind.body.body import MindNode
from essaymind.mood.mood_suppress import MoodNode, MoodAutoUnsuppressNode
 
log = logging.getLogger('OdorMood')

class OdorMood(MoodAutoUnsuppressNode):
    def __init__(self, mood, odor_name, toward_value, away_value):
        super().__init__(mood, 'odor.' + odor_name)
        
        assert toward_value >= 0 and away_value >= 0
        assert toward_value > 0 or away_value > 0
        
        self.name = odor_name

        self.toward_value = toward_value
        self.away_value = away_value
        
    def build(self, body):
        self.body = body
        
        nose = body['body.nose']
        nose[self.name] = self
        
        self.hab = body['habenula']
        
        self.turn_request = 0
        self.value = 0
        self.value_next = 0
    
    def odor(self, odor, value_left, value_right):
        assert value_left >= 0 or value_right >= 0
        
        self.odor_value = odor
        self.value_left = value_left
        self.value_right = value_right
        
        value = 0.5 * (value_left + value_right)
        
        if value > 0:
            self.value_next = value
            self.turn_request = min(0.25, max(-0.25, (value_right - value_left) / value))
    
    def tick(self, body):
        value = self.value_next
        self.value_next = 0
        
        self.value = self.update_mood(value)
        
        self.auto_unsuppress()
        
        turn = self.turn_request
        self.turn_request = 0
        
        if not value:
            return
        
        if self.toward_value:
            self.hab.add_toward(self.toward_value)
            self.hab.add_right_toward(max(0, turn))
            self.hab.add_left_toward(max(0, -turn))
            
        if self.away_value:
            self.hab.add_away(self.away_value)
            self.hab.add_right_away(max(0, turn))
            self.hab.add_left_away(max(0, -turn))
            
        log.debug(f"{self.name} turn_request={turn_request:.2f} V={self.value:.3f} L={self.value_left:.3f} R={self.value_right:.3f} to={self.toward_value} away={self.away_value}")
            

class OdorMoodApproach(OdorMood):
    def __init__(self, mood, name, approach=0.25):
        super().__init__(mood, name, approach, 0)

class OdorMoodAvoid(OdorMood):
    def __init__(self, mood, name, avoid=0.25):
        super().__init__(mood, name, 0, avoid)
        