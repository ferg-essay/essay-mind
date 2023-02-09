import unittest


from body.body import Body
from body.body_move import BodyMove
from locomotion.locomotion_vote import LocomotionVote
from world.world import World

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class HabenulaMoveTest(unittest.TestCase):
    def xtest_basic(self):
        body = Body(World())
        BodyMove(body)
        LocomotionVote(body)
        body.build()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        assert body.dir_ego == 0
        assert body.speed_request == 0
        
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        pass
    
    def test_turn(self):
        body = Body(World())
        BodyMove(body)
        hb = LocomotionVote(body)
        body.build()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        
        hb.left_request(1)
        hb.approach_request(1)
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        
        body.set_dir(0)
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        
        hb.left_request(1)
        hb.approach_request(1)
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        
        hb.left_request(1)
        hb.approach_request(1)
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        
        hb.left_request(1)
        hb.approach_request(1)
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.5,s=0)[][]'
        
        hb.left_request(1)
        hb.approach_request(1)
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.25,s=0)[][]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        pass
    
    def test_turn_conflict(self):
        body = Body(World())
        BodyMove(body)
        hb = LocomotionVote(body)
        body.build()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        
        hb.left_request(1)
        hb.right_request(1)
        hb.approach_request(1)
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        
        body.set_dir(0.5)
        hb.left_request(0.5)
        hb.right_request(1)
        hb.approach_request(1)
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.5,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.75,s=0)[][]'
        
        body.set_dir(0.5)
        hb.left_request(1)
        hb.right_request(0.5)
        hb.approach_request(1)
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.5,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.25,s=0)[][]'
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0.25,s=0)[][]'


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()