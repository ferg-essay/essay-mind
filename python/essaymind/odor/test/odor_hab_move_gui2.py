import logging
import unittest

from body.body import Body
from body.body_move import BodyMove
from world.world import World
from odor.body_nose import BodyNose
from odor.world_odor import Odor
from odor.odor_hab import OdorHab
from locomotion.locomotion_vote import LocomotionVote
from graphics.map_window import MapWindow

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class OdorHabMoveGui(unittest.TestCase):
    def test_thermotaxis_zero_speed_2hz(self):
        world = World()
        body = Body(world)
        config = body.config
        config['hz'] = 10
        nose = BodyNose(body)
        move = BodyMove(body)
        hb = LocomotionVote(body)
        OdorHab(body, 'test')
        #temp = Olfactory(body, "temp")
        # body.add_node("temp.triangulate", Triangulate(temp))
        body.speed_request(0.1   )
        body.build()
        
        world.add_object(OdorTest((5, 5)))

        map_window = MapWindow(world, body.actor, body)
        map_window.pygame_loop()

class OdorTest(Odor):
    def __init__(self, loc, odor_name='test'):
        super().__init__(loc, odor_name)

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
    