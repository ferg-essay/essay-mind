import unittest

from body.body import Body
from world.world import World
#from symbol.body.body_move import BodyMove

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class BodyLocationBaseTest(unittest.TestCase):
    '''
    direction/turn is clockwise [0,1) where north/straight is 0.0, east/right is 0.25
    
    speed_request is [0,1]
    
    Body itself does not move itself; that's managed by BodyMove.
    Body only manages the ego direction, including dx and dy, and speed_request variables.
    ''' 

    def test_basic(self):
        body = Body(World())
        #BodyMove(body)
        body.build()
        
        log.info(body)
        assert str(body) == 'Body(1.00,1.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.speed() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(1.00,1.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.speed() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        pass

    def test_set_dir(self):
        body = Body(World())
        #BodyMove(body)
        body.build()
        
        assert str(body) == 'Body(1.00,1.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.set_dir(0) # north
        assert str(body) == 'Body(1.00,1.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.set_dir(0.25) # east
        assert str(body) == 'Body(1.00,1.00,d=0.25,s=0)[]'
        assert body.dir() == 0.25
        assert body.dx() == 1
        assert body.dy() == 0
        
        body.set_dir(0.5) # south
        assert str(body) == 'Body(1.00,1.00,d=0.5,s=0)[]'
        assert body.dir() == 0.5
        assert body.dx() == 0
        assert body.dy() == -1
        
        body.set_dir(0.75) # west
        assert str(body) == 'Body(1.00,1.00,d=0.75,s=0)[]'
        assert body.dir() == 0.75
        assert body.dx() == -1
        assert body.dy() == 0
        
        body.set_dir(0.25 / 3) # nne
        log.debug(body)
        assert str(body) == 'Body(1.00,1.00,d=0.083,s=0)[]'
        assert body.dir() == 0.0833
        assert body.dx() == 0.5
        assert body.dy() == 0.866
        
        body.set_dir(0.25 / 2) # ne
        log.debug(body)
        assert str(body) == 'Body(1.00,1.00,d=0.12,s=0)[]'
        assert body.dir() == 0.125
        assert body.dx() == 0.7071
        assert body.dy() == 0.7071
        
        body.set_dir(2 * 0.25 / 3) # ene
        log.debug(body)
        assert str(body) == 'Body(1.00,1.00,d=0.17,s=0)[]'
        assert body.dir() == 0.1667
        assert body.dx() == 0.866
        assert body.dy() == 0.5
        pass
    
    def test_speed(self):
        '''
        Note: Body doesn't move by itself, but only manages the variables.
        '''
        world = World()
        world.set_ignore_boundary(True)
        body = Body(world)
        body.build()
        move = body.move
        
        body.moveto(0, 0)
        body.set_dir(0)
        move.set_speed(1)
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=1)[]'
        assert body.dir() == 0
        assert body.speed() == 1
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,1.00,d=0,s=1)[]'
        assert body.dir() == 0
        assert body.speed() == 1
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,2.00,d=0,s=1)[]'
        assert body.dir() == 0
        assert body.speed() == 1
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.moveto(0, 0)
        body.set_dir(0)
        move.set_speed(0)
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.speed() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.speed() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        assert body.dir() == 0
        assert body.speed() == 0
        assert body.dx() == 0
        assert body.dy() == 1
        pass

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()