import unittest
import numpy as np
from world.world import World
from body.body import Body
from tectum.optic import Optic

import logging
from motion.motion_vote import MotionVote
from body.body_move import BodyMove

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class OpticTest(unittest.TestCase):

    def xtest_basic_optic_4(self):
        world = World((4, 4))
        body = Body(world)
        
        border = np.asarray([0, 0, 0, 0,
                             0, 0, 0, 0,
                             0, 0, 0, 0,
                             0, 0, 0, 0], 'f')
        
        left_optic = Optic(body, border, 'l')
        assert left_optic.length == 4
        assert left_optic.m_upper_horizon == 7
        assert left_optic.n_border == 3
        assert left_optic.dx == -1
        
        assert left_optic.is_border_empty_1()
        assert left_optic.is_border_empty_n(2)
        
        right_optic = Optic(body, border, 'r')
        assert right_optic.length == 4
        assert right_optic.m_upper_horizon == 4
        assert right_optic.n_border == 3
        assert right_optic.dx == 1
        
        assert right_optic.is_border_empty_1()
        assert right_optic.is_border_empty_n(2)
        
        border_xy = border.reshape((4, 4))
        border_xy[0][0] = 1
        border_xy[0][1] = 1
        border_xy[0][2] = 1
        border_xy[0][3] = 1
        
        log.info(border_xy)

        assert left_optic.is_border_empty_1()
        assert left_optic.is_border_empty_n(2)
        assert left_optic.is_border_empty_n(3)

        assert right_optic.is_border_empty_1()
        assert right_optic.is_border_empty_n(2)
        assert right_optic.is_border_empty_n(3)
        
        border_xy[1][3] = 1
        log.info(border_xy)

        assert not left_optic.is_border_empty_1()
        assert not left_optic.is_border_empty_n(2)
        assert not left_optic.is_border_empty_n(3)

        assert right_optic.is_border_empty_1()
        assert right_optic.is_border_empty_n(2)
        assert right_optic.is_border_empty_n(3)
        
        border_xy[1][3] = 0
        border_xy[3][3] = 1
        log.info(border_xy)

        assert not left_optic.is_border_empty_1()
        assert not left_optic.is_border_empty_n(2)
        assert not left_optic.is_border_empty_n(3)

        assert right_optic.is_border_empty_1()
        assert right_optic.is_border_empty_n(2)
        assert right_optic.is_border_empty_n(3)
        
        border_xy[3][3] = 0
        border_xy[1][0] = 1
        log.info(border_xy)

        assert left_optic.is_border_empty_1()
        assert left_optic.is_border_empty_n(2)
        assert left_optic.is_border_empty_n(3)

        assert not right_optic.is_border_empty_1()
        assert not right_optic.is_border_empty_n(2)
        assert not right_optic.is_border_empty_n(3)
        
        border_xy[1][0] = 0
        border_xy[3][0] = 1
        log.info(border_xy)

        assert left_optic.is_border_empty_1()
        assert left_optic.is_border_empty_n(2)
        assert left_optic.is_border_empty_n(3)

        assert not right_optic.is_border_empty_1()
        assert not right_optic.is_border_empty_n(2)
        assert not right_optic.is_border_empty_n(3)
        
        border_xy[3][0] = 0
        border_xy[3][1] = 1
        log.info(border_xy)

        assert left_optic.is_border_empty_1()
        assert left_optic.is_border_empty_n(2)
        assert not left_optic.is_border_empty_n(3)

        assert right_optic.is_border_empty_1()
        assert not right_optic.is_border_empty_n(2)
        assert not right_optic.is_border_empty_n(3)
        
        border_xy[3][1] = 0
        border_xy[3][2] = 1
        log.info(border_xy)

        assert left_optic.is_border_empty_1()
        assert not left_optic.is_border_empty_n(2)
        assert not left_optic.is_border_empty_n(3)

        assert right_optic.is_border_empty_1()
        assert right_optic.is_border_empty_n(2)
        assert not right_optic.is_border_empty_n(3)

    def xtest_basic_optic_5(self):
        world = World((4, 4))
        body = Body(world)
        
        border = np.asarray([0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0], 'f')
        
        left_optic = Optic(body, border, 'l')
        assert left_optic.length == 5
        assert left_optic.m_upper_horizon == 14
        assert left_optic.n_border == 3
        assert left_optic.dx == -1
        
        assert left_optic.is_border_1()
        assert left_optic.is_border_n(2)
        
        right_optic = Optic(body, border, 'r')
        assert right_optic.length == 5
        assert right_optic.m_upper_horizon == 10
        assert right_optic.n_border == 3
        assert right_optic.dx == 1
        
        assert right_optic.is_border_1()
        assert right_optic.is_border_n(2)

    def test_optic_brake_4(self):
        world = World((4, 4))
        body = Body(world)
        
        border = np.asarray([0, 0, 0, 0,
                             0, 0, 0, 0,
                             0, 0, 0, 0,
                             0, 0, 0, 0], 'f')
        
        left_optic = Optic(body, border, 'l')
        #right_optic = Optic(body, border, 'r')
        
        move = BodyMove(body)
        locomotion = MotionVote(body)
        
        body.build()
        
        body.speed_request(0.1)
        
        body.tick()
        log.info(locomotion)
        body.tick()
        log.info(locomotion)
        body.tick()
        log.info(locomotion)
        body.tick()
        log.info(locomotion)

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()