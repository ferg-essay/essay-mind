import logging

from essaymind.motion.motion_vote import MotionVote
from mood.mood import MoodAway


log = logging.getLogger("AwayBar")

class ActionObstacle(MoodAway):
    def __init__(self, area, name, left, right):
        super().__init__(area, name)
        
        self.left = left
        self.right = right
        
        #area.action(name).add(self)
        
    def build(self):
        super().build()
        self.motion = MotionVote.from_body(self.area)
        
        #self.area.action('avoid').add(self.send)
        #selector = Selector.add_from_body(body, 'locomotion', 'avoid.obstacle', self.select)
        #selector.add_node('avoid.obstacle', self)
        
    def action(self):
        log.info(f"turn away obstacle L={self.left},R={self.right}")
        self.motion.explore(0, self.left, self.right)
        
class ActionObstacleLeft(ActionObstacle):
    def __init__(self, area, name='obstacle.left'):
        super().__init__(area, name, 1, 0)
        
class ActionObstacleRight(ActionObstacle):
    def __init__(self, area, name='obstacle.right'):
        super().__init__(area, name, 0, 1)
        