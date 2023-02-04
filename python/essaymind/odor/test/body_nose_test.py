import unittest

from body.body import Body
from world.world import World
from odor.body_nose import BodyNose
from odor.world_odor import Odor

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class NoseTest(unittest.TestCase):
    def test_odor_direction(self):
        world = World()
        world.set_ignore_boundary(True)
        body = Body(world)
        nose = BodyNose(body)
        
        target = NoseTarget()
        nose.on_odor().to(target.odor)
        
        body.build()
        
        body.moveto(0, 0)

        assert str(target) == "odor(None,0)"
        
        odor_north = Odor((0, 5), "north")
        odor_south = Odor((0, -5), "south")
        odor_east = Odor((5, 0), "east")
        odor_west = Odor((-5, 0), "west")
        
        nose.odor(odor_north)
        assert str(target) == "odor(NoseOdor[north,0.0000,0.0400],1)"
        
        nose.odor(odor_south)
        assert str(target) == "odor(NoseOdor[south,0.5000,0.0400],1)"
        
        nose.odor(odor_east)
        assert str(target) == "odor(NoseOdor[east,0.2500,0.0400],1)"
        
        nose.odor(odor_west)
        assert str(target) == "odor(NoseOdor[west,0.7500,0.0400],1)"
        
        body.set_dir(0.25)
        nose.odor(odor_north)
        assert str(target) == "odor(NoseOdor[north,0.7500,0.0400],1)"
        
        nose.odor(odor_south)
        assert str(target) == "odor(NoseOdor[south,0.2500,0.0400],1)"
        
        nose.odor(odor_east)
        assert str(target) == "odor(NoseOdor[east,0.0000,0.0400],1)"
        
        nose.odor(odor_west)
        assert str(target) == "odor(NoseOdor[west,0.5000,0.0400],1)"
        
        
    def xtest_odor_direction(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        
        north_target = OldNoseTarget("north")
        south_target = OldNoseTarget("south")
        east_target = OldNoseTarget("east")
        west_target = OldNoseTarget("west")
        
        nose["north"] = north_target
        nose["south"] = south_target
        nose["east"] = east_target
        nose["west"] = west_target
        
        body.build()
        
        #world.add_object(Odor((0, 5), "north"))
        #world.add_object(Odor((0, -5), "south"))
        #world.add_object(Odor((5, 0), "east"))
        #world.add_object(Odor((-5, 0), "west"))
        
        world.add_object(Odor((0, 5), "north"))
        
        body.set_dir(0) # north
        body.tick()
        assert str(north_target) == "north(L=0.040,R=0.040,diff=0.000,dir=0.00)"
        assert str(south_target) == "south(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(east_target) == "east(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(west_target) == "west(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        
        body.set_dir(0.5) # south
        body.tick()
        assert str(north_target) == "north(L=0.040,R=0.040,diff=0.000,dir=0.00)"
        
        body.set_dir(0.25) # east
        body.tick()
        assert str(north_target) == "north(L=0.044,R=0.036,diff=0.008,dir=0.20)"

        body.set_dir(0.75) # west
        body.tick()
        assert str(north_target) == "north(L=0.036,R=0.044,diff=-0.008,dir=0.80)"

        world.remove_local_object((0, 5))
        north_target.reset()
        body.tick()
        assert str(north_target) == "north(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        
        world.add_object(Odor((0, -5), "south"))
        body.set_dir(0) # north
        body.tick()
        assert str(north_target) == "north(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(south_target) == "south(L=0.040,R=0.040,diff=0.000,dir=0.00)"
        assert str(east_target) == "east(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(west_target) == "west(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        
        body.set_dir(0.5) # south
        body.tick()
        assert str(south_target) == "south(L=0.040,R=0.040,diff=0.000,dir=0.00)"
        
        body.set_dir(0.25) # east
        body.tick()
        assert str(south_target) == "south(L=0.036,R=0.044,diff=-0.008,dir=0.80)"

        body.set_dir(0.75) # west
        body.tick()
        assert str(south_target) == "south(L=0.044,R=0.036,diff=0.008,dir=0.20)"

        world.remove_local_object((0, -5))
        south_target.reset()
        body.tick()
        assert str(south_target) == "south(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        
        world.add_object(Odor((5, 0), "east"))
        body.set_dir(0) # north
        body.tick()
        assert str(north_target) == "north(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(south_target) == "south(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(east_target) == "east(L=0.036,R=0.044,diff=-0.008,dir=0.80)"
        assert str(west_target) == "west(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        
        body.set_dir(0.5) # south
        body.tick()
        assert str(east_target) == "east(L=0.044,R=0.036,diff=0.008,dir=0.20)"
        
        body.set_dir(0.25) # east
        body.tick()
        assert str(east_target) == "east(L=0.040,R=0.040,diff=0.000,dir=0.00)"

        body.set_dir(0.75) # west
        body.tick()
        assert str(east_target) == "east(L=0.040,R=0.040,diff=0.000,dir=0.00)"

        world.remove_local_object((5, 0))
        east_target.reset()
        body.tick()
        log.debug(east_target)
        assert str(east_target) == "east(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        
        world.add_object(Odor((-5, 0), "west"))
        body.set_dir(0.5) # north
        body.tick()
        assert str(north_target) == "north(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(south_target) == "south(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(east_target) == "east(L=0.000,R=0.000,diff=0.000,dir=0.00)"
        assert str(west_target) == "west(L=0.036,R=0.044,diff=-0.008,dir=0.80)"
        
        body.set_dir(0.0) # south
        body.tick()
        assert str(west_target) == "west(L=0.044,R=0.036,diff=0.008,dir=0.20)"
        
        body.set_dir(0.25) # east
        body.tick()
        assert str(west_target) == "west(L=0.040,R=0.040,diff=0.000,dir=0.00)"

        body.set_dir(0.75) # west
        body.tick()
        assert str(west_target) == "west(L=0.040,R=0.040,diff=0.000,dir=0.00)"

        world.remove_local_object((-5, 0))
        west_target.reset()
        body.tick()
        assert str(west_target) == "west(L=0.000,R=0.000,diff=0.000,dir=0.00)"

        pass

class NoseTarget(BodyNose):
    def __init__(self, name='odor'):
        self.name=name
        self.reset()
        
    def reset(self):
        self._odor = None
        self.p = 0
        
    def odor(self, odor, p):
        self._odor = odor
        self.p = p
        
    def __str__(self):
        return f"{self.name}({self._odor},{self.p})"

class OldNoseTarget:
    def __init__(self, odor_name):
        self.name=odor_name
        self.reset()
        
    def reset(self):
        self.odor_value = None
        self.value_left = 0
        self.value_right= 0
        
    def odor(self, odor, value_left, value_right):
        self.odor_value = odor
        self.value_left = value_left
        self.value_right = value_right
        
        log.info(f"{self}")
        
    def __str__(self):
        diff = self.value_left - self.value_right
        value_sum = self.value_left + self.value_right
        if value_sum > 0:
            diff_dir = 2 * diff / value_sum
        else:
            diff_dir = 0
            
        diff_dir = (diff_dir + 1) % 1.0
        
        return f"{self.name}(L={self.value_left:.3f},R={self.value_right:.3f},diff={diff:.3f},dir={diff_dir:.2f})"
        
if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
    