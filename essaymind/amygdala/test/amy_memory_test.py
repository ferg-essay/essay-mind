import unittest

from world.world import World
from body.body import Body
from odor.body_nose import BodyNose
from odor.world_odor import Odor
from mood.mood import Moods, MoodNode
from amygdala.amy_memory import AmyMemory

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class AmyMemoryTest(unittest.TestCase):
    def test_amy_memory(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        
        amy = AmyMemory(body)

        mood = Moods(body)
        mood_food = MoodFood(mood)
        mood_ignore = MoodIgnore(mood)
        
        amy_food = amy.add_node('food', mood_food)
        amy_ignore = amy.add_node('ignore', mood_ignore)
        
        amy.add_source(nose)
        
        amy_food['f'] = 'key-food'
        
        body.build()
        
        world.add_object(Odor((5, 0), 'f'))

        for i in range(4):
            body.tick()
            log.info(mood_food)
            log.info(mood_ignore)
            assert str(mood_food) == 'food-Mood[0.25]'
            assert str(mood_ignore) == 'ignore-Mood[0.00]'
            
        print(body)
        # assert body.__str__() == "Body(0.088,0.21)[]"
        pass

class MoodFood(MoodNode):
    def __init__(self, mood):
        super().__init__(mood, 'food')

class MoodIgnore(MoodNode):
    def __init__(self, mood):
        super().__init__(mood, 'ignore')

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
    