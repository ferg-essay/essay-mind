import unittest

from world.world import World
from world.world_actor import WorldActor

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class ActorTouch(unittest.TestCase):


    def xtest_world_touch_boundary_dir(self):
        world = World((10, 20))
        
        assert 0.50 == world.touch_direction(5, 0.5, 0.5)
        
        assert 0.00 == world.touch_direction(5, 19.5, 0.5)
        
        assert 0.75 == world.touch_direction(0.5, 5, 0.5)
        
        assert 0.25 == world.touch_direction(9.5, 5, 0.5)

    def xtest_world_touch_obj_dir(self):
        world = World((10, 20))
        world.world_map[(5, 10)] = 1
        
        assert_dir(0.00, world, 5.5, 9.8, 0.25)
        assert_dir(0.50, world, 5.5, 11.2, 0.25)
        assert_dir(0.25, world, 4.8, 10.5, 0.25)
        assert_dir(0.75, world, 6.2, 10.5, 0.25)
        
        assert_dir(0.13, world, 4.8, 9.8, 0.25)
        assert_dir(0.38, world, 4.8, 11.2, 0.25)
        assert_dir(0.63, world, 6.2, 11.2, 0.25)
        assert_dir(0.88, world, 6.2, 9.8, 0.25)

    def test_actor_touch_obj_dir(self):
        world = World((10, 20))
        world.world_map[(5, 10)] = 1
        actor = WorldActor(world.world_map, "actor", (0, 0), 0.25)
        
        actor.set_dir(0)
        assert_actor_dir(0.00, actor, 5.5, 9.8)
        assert_actor_dir(0.50, actor, 5.5, 11.2)
        assert_actor_dir(0.25, actor, 4.8, 10.5)
        assert_actor_dir(0.75, actor, 6.2, 10.5)
        
        actor.set_dir(0.25)
        assert_actor_dir(0.75, actor, 5.5, 9.8)
        assert_actor_dir(0.25, actor, 5.5, 11.2)
        assert_actor_dir(0.00, actor, 4.8, 10.5)
        assert_actor_dir(0.50, actor, 6.2, 10.5)
        
        actor.set_dir(0.10)
        assert_actor_dir(0.90, actor, 5.5, 9.8)
        assert_actor_dir(0.40, actor, 5.5, 11.2)
        assert_actor_dir(0.15, actor, 4.8, 10.5)
        assert_actor_dir(0.65, actor, 6.2, 10.5)
        
        actor.set_dir(0.90)
        assert_actor_dir(0.10, actor, 5.5, 9.8)
        assert_actor_dir(0.60, actor, 5.5, 11.2)
        assert_actor_dir(0.35, actor, 4.8, 10.5)
        assert_actor_dir(0.85, actor, 6.2, 10.5)

def assert_dir(d, world, x, y, radius):
    if abs(d - world.touch_direction(x, y, radius)) < 1e-4:
        return
    else:
        test_dir = world.touch_direction(x, y, radius)
        log.info(f"{d} != {test_dir} ({x},{y};{radius})")
        assert abs(d - world.touch_direction(x, y, radius)) < 1e-4
        
def assert_actor_dir(d, actor, x, y):
    if abs(d - actor.touch_direction(x, y)) < 1e-4:
        return
    else:
        test_dir = actor.touch_direction(x, y)
        log.info(f"{d} != {test_dir} ({x},{y})")
        assert abs(d - actor.touch_direction(x, y)) < 1e-4
        


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()