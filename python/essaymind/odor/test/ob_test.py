import unittest

from body.body import Body
from body.node import BodyNode
from world.world import World
from odor.body_nose import BodyNose
from odor.world_odor import Odor
from odor.olfactory_bulb import OlfactoryBulb

import logging
from selector.selector import Selector

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class OlfactoryBulbTest(unittest.TestCase):

    def xtest_basic(self):
        world = World()
        world.set_ignore_boundary(True)
        body = Body(world)
        nose = BodyNose(body)
        ob = OlfactoryBulb(body)
        
        nose.on_odor().to(ob.odor)
        
        target = ObTargetTest()
        ob.on_odor().to(target.odor)
        
        body.build()
        
        body.moveto(0, 0)

        target.reset()
        log.info(target)
        assert str(target) == "odor(None,0,0)"
        
        odor_north = Odor((0, 5), "north")
        odor_south = Odor((0, -5), "south")
        odor_east = Odor((5, 0), "east")
        odor_west = Odor((-5, 0), "west")
        
        nose.odor(odor_north)
        log.info(target)
        assert str(target) == "odor(b'3Q3F46OU',0.0,1)"
        
        nose.odor(odor_south)
        log.info(target)
        assert str(target) == "odor(b'FWVI2SQF',0.5,1)"
        
        nose.odor(odor_east)
        log.info(target)
        assert str(target) == "odor(b'NPNSMKYC',0.25,1)"
        
        nose.odor(odor_west)
        log.info(target)
        assert str(target) == "odor(b'O2TZMIEL',0.75,1)"
        pass

    def test_attention(self):
        # world = World()
        # world.set_ignore_boundary(True)
        #body = Body(world)
        body = Body()
        nose = BodyNose(body)
        ob = OlfactoryBulb(body)
        
        nose.on_odor().to(ob.odor)
        
        ob_food = ob.choose("food")
        
        odor_target = ObTargetTest(body, 'odor')
        food_target = ObTargetTest(body, 'food')
        
        ob.to(odor_target.odor)
        ob_food.to(food_target.odor)
        
        body.build()

        log.info(odor_target)
        assert str(odor_target) == "odor(None,0,0)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        odor_north = Odor((0, 5), "north")
        odor_south = Odor((0, -5), "south")
        odor_east = Odor((5, 0), "east")
        odor_west = Odor((-5, 0), "west")
        
        nose.odor(odor_north)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'3Q3F46OU',0.0,1)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        ob.categorize("food")
        
        nose.odor(odor_north)
        body.tick()

        # note: keys differ for same odor but different output
        assert str(odor_target) == "odor(b'3Q3F46OU',0.0,1)"
        assert str(food_target) == "food(b'KA5LM3QX',0.0,1)"
        
        nose.odor(odor_south)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'FWVI2SQF',0.5,1)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        nose.odor(odor_south)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'FWVI2SQF',0.5,1)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        body.set_dir(0.25)
        nose.odor(odor_south)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'FWVI2SQF',0.25,1)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(None,0,0)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        body.set_dir(0)
        nose.odor(odor_south)
        body.tick()
        assert str(odor_target) == "odor(b'FWVI2SQF',0.5,1)"
        assert str(food_target) == "food(None,0,0)"
        
        ob.categorize("food")
        
        nose.odor(odor_south)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'FWVI2SQF',0.5,1)"
        log.info(food_target)
        assert str(food_target) == "food(b'XRNOOF63',0.5,1)"
        
        nose.odor(odor_east)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'NPNSMKYC',0.25,1)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"
        
        nose.odor(odor_west)
        body.tick()
        log.info(odor_target)
        assert str(odor_target) == "odor(b'O2TZMIEL',0.75,1)"
        log.info(food_target)
        assert str(food_target) == "food(None,0,0)"

    def xtest_select(self):
        world = World()
        world.set_ignore_boundary(True)
        body = Body(world)
        nose = BodyNose(body)
        ob = OlfactoryBulb(body)
        
        nose.on_odor().to(ob.odor)
        
        target = SelectTest()
        sel = Selector(body, 'sel').choose_value(target)
        ob.on_odor().to(sel.select)
        
        body.build()
        
        body.moveto(0, 0)

        target.reset()
        log.info(target)
        assert str(target) == "odor_sel(0,0)"
        
        body.tick()
        log.info(target)
        assert str(target) == "odor_sel(0,0)"
        
        odor_north = Odor((0, 5), "north")
        odor_south = Odor((0, -5), "south")
        odor_east = Odor((5, 0), "east")
        odor_west = Odor((-5, 0), "west")
        
        nose.odor(odor_north)
        body.tick()
        log.info(target)
        assert str(target) == "odor_sel(0.0,1)"
        
        nose.odor(odor_south)
        body.tick()
        log.info(target)
        assert str(target) == "odor_sel(0.5,1)"
        
        nose.odor(odor_east)
        body.tick()
        log.info(target)
        assert str(target) == "odor_sel(0.25,1)"
        
        nose.odor(odor_west)
        body.tick()
        log.info(target)
        assert str(target) == "odor_sel(0.75,1)"
        pass
    
class ObTargetTest(BodyNode):
    def __init__(self, parent, name='odor'):
        super().__init__(parent, name)
        
        self.next_odor = None
        self.reset()
        
    def reset(self):
        self.key = None
        self.angle = 0
        self.p = 0
        
    def odor(self, key, angle, p):
        self.next_odor = (key, angle, p)
        
    def tick(self):
        if self.next_odor:
            self.key, self.angle, self.p = self.next_odor
        else:
            self.reset()
        
        self.next_odor = None
        
    def __str__(self):
        return f"{self.name}({self.key},{self.angle},{self.p})"
    
class SelectTest(BodyNose):
    def __init__(self, name='odor_sel'):
        self.name=name
        self.reset()
        
    def reset(self):
        self.value = 0
        self.p = 0
        
    def __call__(self, value, p):
        self.value = value
        self.p = p
        
    def __str__(self):
        return f"{self.name}({self.value},{self.p})"



if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()