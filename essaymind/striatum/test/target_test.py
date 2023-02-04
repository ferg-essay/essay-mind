import logging
import unittest

'''
Selector test as selecting a target
'''
from world.world import World
from body.body import Body, BodyNode
from selector.selector import Selector
from odor.body_nose import BodyNose
from odor.olfactory_bulb import OlfactoryBulb

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class SelectorTargetTest(unittest.TestCase):


    def xtest_target(self):
        body = Body(World())
        
        nose = BodyNose(body)
        ob = OlfactoryBulb(body).when(nose.on_odor())
        
        action = TestAction(body)
        
        sel = Selector(body, 'target')
        
        s_left = sel.choose(action.left, 'left')
        s_forward = sel.choose(action.forward, 'forward')
        s_right = sel.choose(action.right, 'right')
        
        body.build()
        pass
    
class TestAction(BodyNode):
    def __init__(self, body, name='test'):
        body[name] = self
        self.actions = []
    
    def left(self):
        self.next_actions.append('left')
        
    def right(self):
        self.next_actions.append('right')
        
    def forward(self):
        self.next_actions.append('forward')
        
    def tick(self):
        self.actions = self.next_actions
        self.next_actions = []
        
    def __str__(self):
        return f"Test{self.actions}"

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()