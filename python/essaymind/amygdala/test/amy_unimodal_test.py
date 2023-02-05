import unittest
import logging

from essaymind import MindNode, Body, FiberKey
from essaymind.amygdala import AmyUnimodal

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class AmyUnimodalTest(unittest.TestCase):


    def test_sense(self):
        body = Body()
        
        sense = FiberKey()
        
        amy = AmyUnimodal(body, 'amy')
        amy.when_sense(sense)
        
        test = TestValue(body, 'test')
        amy.on_value(test)

        body.build()
        
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        
        sense("sense")
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        
        amy.value("sense")
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        
        sense("sense")
        assert str(test) == 'test[]'
        body.tick()
        log.debug(test)
        assert str(test) == 'test[sense]'
        body.tick()
        log.debug(test)
        assert str(test) == 'test[]'
        body.tick()
        log.debug(test)
        assert str(test) == 'test[]'
        
        sense("bogus")
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
        
        sense("sense")
        body.tick()
        assert str(test) == 'test[sense]'
        body.tick()
        assert str(test) == 'test[]'
        body.tick()
        assert str(test) == 'test[]'
                
        pass
    
class TestValue(MindNode):
    def __init__(self, parent, name):
        super().__init__(parent, name)
        
        self._value = None
        self._next = None
        
    def sense(self, key):
        self._next = key
        
    def __call__(self, key):
        self.sense(key)
        
    def tick(self, ticks):
        self._value = self._next
        self._next = None
        
    def __str__(self):
        if self._value: 
            return f"test[{self._value}]"
        else:
            return f"test[]"


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()