import unittest

from world.world import World
from world.world_actor import WorldActor

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class WorldActorTest(unittest.TestCase):
    def test_basic_2_2_p1(self):
        world = World((2, 2))
        actor = WorldActor(world, 'test', (0, 0), 0.1)
        
        log.debug(actor)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert not actor.moveto(0, 0)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert actor.moveto(0.5, 0.5)
        assert str(actor) == 'test-Actor(0.50,0.50,d=0)'
        
        assert not actor.moveto(-1, 0.5)
        assert str(actor) == 'test-Actor(0.50,0.50,d=0)'        
        
        assert not actor.moveto(0.09, 0.5)
        assert str(actor) == 'test-Actor(0.50,0.50,d=0)'
        
        assert actor.moveto(0.11, 0.5)
        assert str(actor) == 'test-Actor(0.11,0.50,d=0)'
        
        assert not actor.moveto(3, 0.5)
        assert str(actor) == 'test-Actor(0.11,0.50,d=0)'
        
        assert not actor.moveto(1.91, 0.5)
        assert str(actor) == 'test-Actor(0.11,0.50,d=0)'
        
        assert actor.moveto(1.89, 0.5)
        assert str(actor) == 'test-Actor(1.89,0.50,d=0)'
        
        assert not actor.moveto(0.5, -1)
        assert str(actor) == 'test-Actor(1.89,0.50,d=0)'        
        
        assert not actor.moveto(0.5, 0.09)
        assert str(actor) == 'test-Actor(1.89,0.50,d=0)'
        
        assert actor.moveto(0.5, 0.11)
        assert str(actor) == 'test-Actor(0.50,0.11,d=0)'
        
        assert not actor.moveto(0.5, 3)
        assert str(actor) == 'test-Actor(0.50,0.11,d=0)'
        
        assert not actor.moveto(0.5, 1.91)
        assert str(actor) == 'test-Actor(0.50,0.11,d=0)'
        
        assert actor.moveto(0.5, 1.89)
        assert str(actor) == 'test-Actor(0.50,1.89,d=0)'
        pass
    
    def test_basic_10_10_p2(self):
        world = World((10, 10))
        actor = WorldActor(world, 'test', (0, 0), 0.2)
        
        log.debug(actor)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert not actor.moveto(0, 0)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert actor.moveto(0.5, 0.5)
        assert str(actor) == 'test-Actor(0.50,0.50,d=0)'
        
        assert not actor.moveto(-1, 0.5)
        assert str(actor) == 'test-Actor(0.50,0.50,d=0)'        
        
        assert not actor.moveto(0.19, 0.5)
        assert str(actor) == 'test-Actor(0.50,0.50,d=0)'
        
        assert actor.moveto(0.21, 0.5)
        assert str(actor) == 'test-Actor(0.21,0.50,d=0)'
        
        assert not actor.moveto(11, 0.5)
        assert str(actor) == 'test-Actor(0.21,0.50,d=0)'
        
        assert not actor.moveto(9.81, 0.5)
        assert str(actor) == 'test-Actor(0.21,0.50,d=0)'
        
        assert actor.moveto(9.79, 0.5)
        assert str(actor) == 'test-Actor(9.79,0.50,d=0)'
        
        assert not actor.moveto(0.5, -1)
        assert str(actor) == 'test-Actor(9.79,0.50,d=0)'        
        
        assert not actor.moveto(0.5, 0.19)
        assert str(actor) == 'test-Actor(9.79,0.50,d=0)'
        
        assert actor.moveto(0.5, 0.21)
        assert str(actor) == 'test-Actor(0.50,0.21,d=0)'
        
        assert not actor.moveto(0.5, 11)
        assert str(actor) == 'test-Actor(0.50,0.21,d=0)'
        
        assert not actor.moveto(0.5, 9.81)
        assert str(actor) == 'test-Actor(0.50,0.21,d=0)'
        
        assert actor.moveto(0.5, 9.79)
        assert str(actor) == 'test-Actor(0.50,9.79,d=0)'
        pass
    
    def test_basic_10_2_p1(self):
        world = World((10, 2))
        actor = WorldActor(world, 'test', (0, 0), 0.1)
        
        log.debug(actor)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert not actor.moveto(0, 0)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert actor.moveto(0.11, 0.5)
        assert str(actor) == 'test-Actor(0.11,0.50,d=0)'
        
        assert not actor.moveto(0.09, 0.5)
        assert str(actor) == 'test-Actor(0.11,0.50,d=0)'
        
        assert actor.moveto(9.89, 0.5)
        assert str(actor) == 'test-Actor(9.89,0.50,d=0)'
        
        assert not actor.moveto(9.91, 0.5)
        assert str(actor) == 'test-Actor(9.89,0.50,d=0)'
        
        assert actor.moveto(0.5, 0.11)
        assert str(actor) == 'test-Actor(0.50,0.11,d=0)'
        
        assert not actor.moveto(0.5, 0.09)
        assert str(actor) == 'test-Actor(0.50,0.11,d=0)'
        
        assert actor.moveto(0.5, 1.89)
        assert str(actor) == 'test-Actor(0.50,1.89,d=0)'
        
        assert not actor.moveto(0.5, 1.91)
        assert str(actor) == 'test-Actor(0.50,1.89,d=0)'
    
    def test_block_5_5_p1(self):
        world = World((10, 10))
        world.set_map((5,5), 0.1)
        actor = WorldActor(world, 'test', (0, 0), 0.1)
        
        assert not actor.moveto(5, 5)
        assert str(actor) == 'test-Actor(0.00,0.00,d=0)'
        
        assert actor.moveto(4.89, 5)
        assert str(actor) == 'test-Actor(4.89,5.00,d=0)'
        
        assert not actor.moveto(4.91, 5)
        assert str(actor) == 'test-Actor(4.89,5.00,d=0)'
        
        assert actor.moveto(6.11, 5)
        assert str(actor) == 'test-Actor(6.11,5.00,d=0)'
        
        assert not actor.moveto(6.09, 5)
        assert str(actor) == 'test-Actor(6.11,5.00,d=0)'
        
        assert actor.moveto(5, 4.89)
        assert str(actor) == 'test-Actor(5.00,4.89,d=0)'
        
        assert not actor.moveto(5, 4.91)
        assert str(actor) == 'test-Actor(5.00,4.89,d=0)'
        
        assert actor.moveto(5, 6.11)
        assert str(actor) == 'test-Actor(5.00,6.11,d=0)'
        
        assert not actor.moveto(5, 6.09)
        assert str(actor) == 'test-Actor(5.00,6.11,d=0)'


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()