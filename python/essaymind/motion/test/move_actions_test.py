import unittest

from body.body import Body
from body.action import ActionGroup
from motion.move_actions import MoveNode
import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class Test(unittest.TestCase):


    def test_basic(self):
        body = Body()
        
        group = ActionGroup(body, 'test').ticks(4)
        #forward = Forward(group)
        #left = Left(group)
        #right = Right(group)
        #forward_left = ForwardLeft(group)
        #forward_right = ForwardRight(group)
        
        forward = MoveNode(group, "forward", 0.5, 0)
        left = MoveNode(group, "left", 0, -0.25)
        right = MoveNode(group, "right", 0, 0.25)
        forward_left = MoveNode(group, "fwd_left", 0.5, -0.25)
        forward_right = MoveNode(group, "fwd_right", 0.5, 0.25)
        
        body.build()
        
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        forward.request(1)
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.12,d=0,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.25,d=0,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.38,d=0,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0,s=0)[]'
        
        left.request(1)
        body.tick()
        assert str(body) == 'Body(0.00,0.50,d=0,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.94,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.88,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.81,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.75,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.75,s=0)[]'
        
        right.request(1)
        body.tick()
        assert str(body) == 'Body(0.00,0.50,d=0.75,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.81,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.88,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0.94,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,0.50,d=0,s=0)[]'
        
        forward_left.request(1)
        body.tick()
        assert str(body) == 'Body(0.00,0.50,d=0,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.05,0.62,d=0.94,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.14,0.70,d=0.88,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.25,0.75,d=0.81,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.38,0.75,d=0.75,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.38,0.75,d=0.75,s=0)[]'
        
        forward_right.request(1)
        body.tick()
        assert str(body) == 'Body(-0.38,0.75,d=0.75,s=0)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.49,0.80,d=0.81,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.58,0.89,d=0.88,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.63,1.00,d=0.94,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.63,1.13,d=0,s=0.12)[]'
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.63,1.13,d=0,s=0)[]'
        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()