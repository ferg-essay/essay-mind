import unittest
from world.world import World
from world.world_actor import WorldActor
from body.body import Body
from body.body_move import BodyMove
from motion.motion_vote import MotionVote
from motion.explore import MoodExplore
from touch.touch import BodyTouch

from graphics.actor_controller import ActorController
from graphics.pygame_loop import PyGameLoop
from graphics.world_map_view import WorldMapView

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")


class LocomotionTest(unittest.TestCase):


    def test_locomotion(self):
        world = World((10, 10))
        body = Body(world)
        body.config['hz'] = 10
        move = BodyMove(body)
        locomotion = MotionVote(body)
        explore = MoodExplore(body)
        touch = BodyTouch(body)
        body.build()
        
        pygame_loop = PyGameLoop((400, 400))
        
        actor_controller = ActorController(body.actor)
        pygame_loop.add_controller(actor_controller)
        actor_controller.add_ticker(body)
        
        world_map_view = WorldMapView(world, ((000, 000, 400, 400)))
        world_map_view.add_actor(body.actor)
        world_map_view = world_map_view
        pygame_loop.add_view(world_map_view)
        
        pygame_loop.pygame_loop()
        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()