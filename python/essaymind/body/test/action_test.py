import unittest

import logging
from body.body import Body
from body.action import ActionGroup, ActionNode

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")


class ActionTest(unittest.TestCase):
    def xtest_basic(self):
        body = Body()
        
        group = ActionGroup(body, "action").ticks(2)
        action = TestAction(group)
        
        body.build()
        
        log.info(action)
        assert str(action) == 'test[]'
        body.tick()
        assert str(action) == 'test[]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        
        action.request(1)
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[action stop]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        
        # chain
        action.request(1)
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        action.request(1)
        body.tick()
        log.info(action)
        assert str(action) == 'test[action stop]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[action stop]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        
        # chain-2
        action.request(1)
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        action.request(1)
        body.tick()
        log.info(action)
        assert str(action) == 'test[action stop]'
        
        action.request(1)
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[action stop]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        pass
    
    def test_default_ticks(self):
        body = Body()
        
        group = ActionGroup(body, "action")
        action = TestAction(group, 'test')
        
        body.build()
        
        log.info(action)
        assert str(action) == 'test[]'
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
        
        action.request(1)
        log.info(action)
        assert str(action) == 'test[]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[action stop]'
        
        body.tick()
        log.info(action)
        assert str(action) == 'test[]'
    
    def xtest_two(self):
        body = Body()
        
        group = ActionGroup(body, "action").ticks(2)
        a_action = TestAction(group, 'a')
        b_action = TestAction(group, 'b')
        
        body.build()
        
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        
        a_action.request(1)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action stop]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        
        b_action.request(1)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[action]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[action stop]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        
        # a vs b
        a_action.request(0.5)
        b_action.request(1)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[action]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[action stop]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        
        # a vs b
        a_action.request(1)
        b_action.request(0.5)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        log.info(b_action)
        assert str(a_action) == 'a[action]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action stop]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        
        # a vs b - chain sum - a
        a_action.request(1)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        log.info(b_action)
        assert str(a_action) == 'a[action]'
        assert str(b_action) == 'b[]'
        a_action.request(1)
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action stop]'
        assert str(b_action) == 'b[]'
        a_action.request(0.5)
        b_action.request(1)
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action stop]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        
        # a vs b - chain sum - b
        a_action.request(1)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'
        body.tick()
        log.info(a_action)
        log.info(b_action)
        assert str(a_action) == 'a[action]'
        assert str(b_action) == 'b[]'
        b_action.request(1)
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[action stop]'
        assert str(b_action) == 'b[]'
        a_action.request(1)
        b_action.request(0.5)
        body.tick()
        log.info(b_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[action]'
        body.tick()
        log.info(b_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[action stop]'
        body.tick()
        log.info(a_action)
        assert str(a_action) == 'a[]'
        assert str(b_action) == 'b[]'

class TestAction(ActionNode):
    def __init__(self, parent, name='test'):
        super().__init__(parent, name)
        
        self.state=""
        
    def start(self):
        super().start()
        self.state = "start"
        
    def tick(self):
        self.state = ""
        super().tick()
        
    def action(self):
        if self.state:
            self.state += " action"
        else:
            self.state = "action"
        
    def stop(self):
        super().stop()
        self.state += " stop"
        
    def __str__(self):
        return f"{self.name}[{self.state}]"
        

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()