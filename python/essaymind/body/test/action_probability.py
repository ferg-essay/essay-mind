import unittest
import logging

from body.body import Body
from body.action_probability import ActionNode, ActionProbabilityGroup

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")


class ActionProbabilityTest(unittest.TestCase):

    def xtest_probability_probe(self):
        body = Body()
        group = ActionProbabilityGroup(body, "group")
        test_a = TestAction(group, "a")
        test_b = TestAction(group, "b")
        body.build()
        
        test = TestResult()
        
        log.info(group)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0'
        
        test_b.excite(0.5)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0.5'
        
        test_b.inhibit(0.5)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0'
        
        test_b.inhibit(0.5)
        group.probe(0.5, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0.25'
        
        test_b.inhibit(0.5)
        group.probe(0, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0.5'
        
        test_a.inhibit(0.5)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:-0.5 b:0'
        
        test_a.excite(0.5)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0'
        
        test_a.excite(1)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:0.5 b:0'
        
        test_a.excite(0.5)
        group.probe(1, test.reset())
        log.info(test)
        assert str(test) == 'a:0 b:0'
         
        pass
    
    def test_probability(self):
        body = Body()
        body.seed(1)
        group = ActionProbabilityGroup(body, "group")
        group.ticks(2)
        test_a = TestAction(group, "a")
        test_b = TestAction(group, "b")
        
        test_copy = TestActionCopy()
        group.on_action(test_copy)
        body.build()
        
        log.info(test_a)
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        
        test_a.excite(0.5)
        
        body.tick()
        assert str(test_a) == 'a[1]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[0]'
        assert str(test_b) == 'b[None]'
        log.info(test_copy)
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        
        test_b.excite(0.5)
        
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[1]'
        assert test_copy.get_and_clear() == "b:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[0]'
        log.info(test_copy)
        assert test_copy.get_and_clear() == "b:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        
        test_a.excite(0.5)
        test_b.excite(0.5)
        
        body.tick()
        assert str(test_a) == 'a[1]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[0]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        
        test_a.excite(0.5)
        test_b.excite(0.5)
        
        body.tick()
        assert str(test_a) == 'a[1]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[0]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        
        test_a.excite(0.5)
        test_b.excite(0.5)
        
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[1]'
        assert test_copy.get_and_clear() == "b:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[0]'
        assert test_copy.get_and_clear() == "b:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
        
        test_a.excite(0.5)
        test_b.excite(0.5)
        
        body.tick()
        assert str(test_a) == 'a[1]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[0]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == "a:1"
        body.tick()
        assert str(test_a) == 'a[None]'
        assert str(test_b) == 'b[None]'
        assert test_copy.get_and_clear() == None
    
class TestResult:
    def __init__(self):
        self._values = []
        
    def reset(self):
        self._values = []
        
        return self
    
    def __call__(self, name, value):
        self._values.append(f"{name}:{value:.2g}")
        self._values.sort()
        
    def __str__(self):
        return ' '.join(self._values)
    
class TestActionCopy:
    def __init__(self):
        self._value = None
        
    def reset(self):
        self._value = None
        
        return self
    
    def __call__(self, name, value, p):
        assert not self._value
        self._value = f"{name}:{value:.2g}"

    def get_and_clear(self):
        value = self._value
        self._value = None
        return value
        
    def __str__(self):
        return self._value

class TestAction(ActionNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        self._value = None
        
    def tick(self):
        self._value = None
        super().tick()
        
    def action(self, value):
        self._value = value
        
        log.info(f"action {self}")
        
    def __str__(self):
        return f"{self.name}[{self._value}]"
        
if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()