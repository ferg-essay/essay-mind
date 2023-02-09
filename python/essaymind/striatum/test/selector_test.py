import unittest

from world.world import World
from body.body import Body, BodyNode
from selector.selector import Selector
import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class SelectorTest(unittest.TestCase):


    def test_selector_single_action(self):
        body = Body(World())
        
        selector = Selector(body, 'selector_test')
        selector.select_tick_max = 2
        selector.unselect_tick_max = 2
        
        action = TestAction(body)
        
        sel_action = selector.add(action.name, action.send)
        
        sel1 = selector.nexus("select")
        unsel1 = selector.nexus("unselect")
        sel_action.add_select(sel1)
        sel_action.add_unselect(unsel1)
        
        body.build()
        
        body.tick()
        log.info(action)
        assert str(action) == "test[False]"
        
        sel1()
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[True]"
        body.tick()
        assert str(action) == "test[True]"
        body.tick()
        assert str(action) == "test[False]"
        
        sel1()
        unsel1()
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        
        unsel1()
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        
        unsel1()
        body.tick()
        assert str(action) == "test[False]"
        sel1()
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[True]"
        
        sel1()
        body.tick()
        assert str(action) == "test[False]"
        unsel1()
        body.tick()
        assert str(action) == "test[True]"
        body.tick()
        assert str(action) == "test[False]"
        body.tick()
        assert str(action) == "test[False]"
        
        pass

class TestAction(BodyNode):
    def __init__(self, body, name='test'):
        self.name = name
        
        body[name] = self
        
        self.value_send = False
        self.value = False
        
    def send(self):
        self.value_send = True
        
    def tick(self):
        self.value = self.value_send
        self.value_send = False
        
        if self.value:
            log.info(f"{self.name}[{self.value}]")
            
    def __str__(self):
        return f"{self.name}[{self.value}]"

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()