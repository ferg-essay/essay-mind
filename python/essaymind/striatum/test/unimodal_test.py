import unittest
import logging

from essaymind import Body, MindNode, FiberAngle, FiberKey

from essaymind.striatum import StriatumAngle

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class SelectorUnimodalTest(unittest.TestCase):

    def test_base(self):
        body = Body()
        
        odor = FiberAngle()
        amy = FiberKey()
        
        striatum = StriatumAngle(body, "angle")
        
        test = TestAction(body)
        
        body.build()
        
        body.tick()
        log.info(test)
        pass
    
class TestAction(MindNode):
    def __init__(self, parent, name='test'):
        super().__init__(parent, name)
        
        self._value = None
        
    def __str__(self):
        if self._value:
            return f"{self.name}[{self._value}]"
        else:
            return f"{self.name}[]"


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()