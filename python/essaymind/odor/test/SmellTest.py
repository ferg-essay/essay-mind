import logging
import unittest

from body.body import Body
from world.world import World, WorldObject

logging.basicConfig(level=logging.INFO,
                    format='%(levelname)s:%(name)-10s: %(message)s')

#logging.getLogger('symbol.body.body').setLevel(logging.INFO)
#logging.info("test")

class SmellTest(unittest.TestCase):
    
    def test_smell_approach(self):
        world = World()
        body = Body(world)
        body.build()
        
        body.tick()
        assert body.actions == []
        assert body.node("sense").__str__() == "Senses[]"
        
        world.add_object(WorldObject((1, 1), {"odor": "coffee"}))
        assert world.get_near_obj((0, 0), "odor").__str__() == "WorldObject(1, 1){'odor': 'coffee'}"
        
        body.tick()
        assert body.actions == []
        assert body.node("sense").__str__() == "Senses[]"
        assert body.node("nose").__str__() == "Nose['coffee']"
        
        body.tick()
        assert body.__str__() == "Body(1, 1)['approach']"
        assert body.node("sense").__str__() == "Senses['coffee']"
        assert body.node("nose").__str__() == "Nose['coffee']"
        
        body.tick()
        assert body.__str__() == "Body(1, 1)[]"
        assert body.node("sense").__str__() == "Senses['coffee']"
        assert body.node("nose").__str__() == "Nose['coffee']"
        pass
    
    def xtest_smell_add_remove(self):
        world = World()
        body = Body(world)
        body.build()
        
        body.tick()
        assert body.actions == []
        assert body.node("sense").__str__() == "Senses[]"
        
        world.add_object(WorldObject((1, 1), {"odor": "coffee"}))
        print(world.get_near_obj((0, 0), "odor"))
        assert world.get_near_obj((0, 0), "odor").__str__() == "WorldObject(1, 1){'odor': 'coffee'}"
        
        body.tick()
        assert body.actions == []
        assert body.node("sense").__str__() == "Senses[]"
        assert body.node("nose").__str__() == "Nose['coffee']"
        
        body.tick()
        assert body.__str__() == "Body[(1, 1), ['approach']]"
        assert body.node("sense").__str__() == "Senses['coffee']"
        assert body.node("nose").__str__() == "Nose['coffee']"
        
        body.tick()
        assert body.__str__() == "Body[(1, 1), []]"
        assert body.node("sense").__str__() == "Senses['coffee']"
        assert body.node("nose").__str__() == "Nose['coffee']"
        
        world.remove(world.get((1, 1)))
        
        body.tick()
        assert body.__str__() == "Body[(1, 1), []]"
        assert body.node("sense").__str__() == "Senses['coffee']"
        assert body.node("nose").__str__() == "Nose[]"
        
        body.tick()
        assert body.__str__() == "Body[(1, 1), []]"
        assert body.node("sense").__str__() == "Senses[]"
        assert body.node("nose").__str__() == "Nose[]"
        pass
    
    def xtestSmellNull(self):
        world = World()
        body = Body(world)
        body.build()
        
        body.tick()
        assert body.actions == []
        assert body.node("sense").__str__() == "Senses[]"
        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
    