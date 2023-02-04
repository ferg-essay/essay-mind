import logging
import unittest

from body.body import Body
from body.body_move import BodyMove
from world.world import World
from odor.body_nose import BodyNose
from odor.world_odor import Odor
from odor.odor_hab import OdorHab
from locomotion.locomotion_vote import LocomotionVote

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class OdorHabMoveTest(unittest.TestCase):
    def test_thermotaxis_zero_speed_1hz(self):
        world = World()
        body = Body(world)
        config = body.config
        config['hz'] = 1
        nose = BodyNose(body)
        move = BodyMove(body)
        hb = LocomotionVote(body)
        OdorHab(body, 'test')
        #temp = Olfactory(body, "temp")
        # body.add_node("temp.triangulate", Triangulate(temp))
        
        body.build()
        
        world.add_object(OdorTest((2, 0)))

        log.debug(world.get((2, 0)))
        assert str(world.get((2, 0))) == "Odor-test[]"
        
        # note the overshoot because of the low hz
        values = [0, 0, 0.061, 0.12, 0.18,
                  0.24, 0.29, 0.31, 0.31, 0.3,
                  0.28, 0.25, 0.23, 0.22, 0.22]
        
        for value in values:
            body.tick()
            log.debug(body)
            assert str(body) == f"Body(0.00,0.00,d={value},s=0)[][]"
        pass
    
    def xtest_thermotaxis_zero_speed_2hz(self):
        world = World()
        body = Body(world)
        config = body.config
        config['hz'] = 2
        nose = BodyNose(body)
        move = BodyMove(body)
        hb = LocomotionVote(body)
        OdorHab(body, 'test')
        #temp = Olfactory(body, "temp")
        # body.add_node("temp.triangulate", Triangulate(temp))
        
        body.build()
        
        world.add_object(OdorTest((2, 0)))

        log.debug(world.get((2, 0)))
        assert str(world.get((2, 0))) == "Odor-test[]"
        
        values = [0, 0, 0.031, 0.062, 0.092,
                  0.12, 0.15, 0.18, 0.2, 0.22,
                  0.23, 0.24, 0.25, 0.25, 0.25]
        
        for value in values:
            body.tick()
            log.debug(body)
            assert str(body) == f"Body(0.00,0.00,d={value},s=0)[][]"
        pass

class OdorTest(Odor):
    def __init__(self, loc, odor_name='test'):
        super().__init__(loc, odor_name)

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
    