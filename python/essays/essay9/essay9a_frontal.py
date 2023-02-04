import unittest


import logging

from front.action_replay import FrontAction

from body.body import Body
from body.action_probability import ActionProbabilityGroup
from event.grid import Grid, TorusBuffer
from motion.move_probability_actions import MoveNode

logging.basicConfig(level=logging.INFO,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')
logging.getLogger().setLevel(logging.DEBUG)

log = logging.getLogger("Test")
        

class Essay9aFrontal(unittest.TestCase):


    def test_basic(self):
        body = Body()
        body.seed(8)
        body.random_max(True)
        body.moveto(5, 5)
        self.build_actions(body)
        
        grid = Grid(body, "grid", 3)
        
        front = FrontAction(body)
        
        front.actions(self.moves)
        front.when_action_copy(self.action_group)
        front.when_action_idle(self.action_group)
        front.when_grid(grid)
        
        body.build()
        
        for i in range(20):
            body.tick()

        pass

    def build_actions(self, body):
        self.speed = speed = 0.5
        group = self.action_group = ActionProbabilityGroup(body, 'random_walk_actions')
        group.ticks(4)
        
        angles = self.angles = [0, 1/6, -1/6, 2/6, -2/6, 0.49, -0.49]
            
        move_0 = MoveNode(group, 'move_0', speed, angles[0])
        move_2 = MoveNode(group, 'move_2', speed, angles[1])
        move_10 = MoveNode(group, 'move_10', speed, angles[2])
        move_4 = MoveNode(group, 'move_4', speed, angles[3])
        move_8 = MoveNode(group, 'move_8', speed, angles[4])
        move_6 = MoveNode(group, 'move_6_cw', speed, angles[5])
        move_6_ccw = MoveNode(group, 'move_6_ccw', speed, angles[6])
        
        self.moves = [move_0, move_2, move_10, move_4, move_8, move_6, move_6_ccw]
        
if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()