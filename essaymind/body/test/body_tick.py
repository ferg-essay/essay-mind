import unittest

from essaymind.body import Body
from essaymind.core import MindNode
from essaymind.world import World

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")


class BodyTick(unittest.TestCase):


    def test_body_node_tick(self):
        world = World()
        body = Body(world)
        node = TestNode(body)

        assert str(node) == 'test[0,0,False]'
        body.build()
        
        assert str(node) == 'test[0,0,True]'
        body.tick()
        log.info(node)
        assert str(node) == 'test[0,1,True]'
        body.tick()
        assert str(node) == 'test[1,2,True]'
        pass

class TestNode(MindNode):
    def __init__(self, body, name='test'):
        self.name = name
        
        self.is_build = False
        self._count = 0
        self._ticks = 0
        
        body[name] = self
        
    def build(self):
        self.is_build = True
        
    def tick(self, ticks):
        self._count += 1
        self._ticks = ticks
        
    def __str__(self):
        return f"{self.name}[{self._ticks},{self._count},{self.is_build}]"
        
if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()