import unittest

from body.body import Body
from body.body_move import BodyMove
from world.world import World

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class MoveTest(unittest.TestCase):
    def test_move_90(self):
        world = World()
        world.set_ignore_boundary(True)
        
        body = Body(world)
        body.config['hz'] = 1
        body.config['body.speed'] = 1
        body.config['body.turn'] = 1

        move = body.move
        
        body.build()
        
        assert body.moveto(0, 0)
        body.set_dir(0) # north
        move.request_speed(1)
        body.tick()
        log.info(body)
        assert str(body) == "Body(0.00,1.00,d=0,s=1)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0.5) # south
        move.request_speed(1)
        body.tick()
        assert str(body) == "Body(0.00,-1.00,d=0.5,s=1)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0.25) # east
        move.request_speed(1)
        body.tick()
        assert str(body) == "Body(1.00,0.00,d=0.25,s=1)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0.75) # west
        move.request_speed(1)
        body.tick()
        assert str(body) == "Body(-1.00,0.00,d=0.75,s=1)[]"
        
        # 30/45 degrees
        assert body.moveto(0, 0)
        body.set_dir(0.25 / 3) # NNE
        move.request_speed(1)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.50,0.87,d=0.083,s=1)[]"
        
    def test_move_30_45_60(self):
        world = World()
        world.set_ignore_boundary(True)
        
        body = Body(world)
        body.config['hz'] = 1
        body.config['body.speed'] = 1
        body.config['body.turn'] = 1

        move = body.move
        
        body.build()
        
        # 30/45 degrees
        assert body.moveto(0, 0)
        body.set_dir(0.25 / 3) # NNE
        move.request_speed(1)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.50,0.87,d=0.083,s=1)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0.25 / 2) # NE
        move.request_speed(1)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.71,0.71,d=0.12,s=1)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0.25 * 2 / 3) # ENE
        move.request_speed(1)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.87,0.50,d=0.17,s=1)[]"
        
    def test_turn_left_right(self):
        world = World()
        world.set_ignore_boundary(True)
        
        body = Body(world)
        body.config['hz'] = 1
        body.config['body.speed'] = 1
        body.config['body.turn'] = 1

        move = body.move
        
        body.build()
        move.set_speed(0)
        
        # dir = 0
        assert body.moveto(0, 0)
        body.set_dir(0)
        move.turn(0)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        
        move.turn(0.25)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.25,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.25,s=0)[]"
        
        move.turn(0.25)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        
        move.turn(0.25)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.75,s=0)[]"
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.75,s=0)[]"
        
        move.turn(0.25)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        
        # dir = 0 - north
        assert body.moveto(0, 0)
        body.set_dir(0)
        move.turn(0)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0)
        move.turn(0.75)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.75,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.75,s=0)[]"
        move.turn(0.75)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0)
        move.turn(0.25)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.25,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.25,s=0)[]"
        
        move.turn(0.25)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        
        assert body.moveto(0, 0)
        body.set_dir(0)
        move.turn(0.75)
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0.75,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.75,s=0)[]"
        move.turn(0.75)
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.5,s=0)[]"
        move.turn(0.75)
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.25,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0.25,s=0)[]"
        move.turn(0.75)
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0,s=0)[]"


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()