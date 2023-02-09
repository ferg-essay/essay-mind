import unittest

from world.world import World
from body.body import Body
from body.body_move import BodyMove
#from mood.mood import Mood, MoodSuppressMax
from mood.mood_suppress import Mood, MoodSuppressMax
from locomotion.locomotion_vote import LocomotionVote
from odor.body_nose import BodyNose
from odor.world_odor import Odor
from odor.mood_odor import OdorMoodApproach
from odor.mood_odor import OdorMoodAvoid

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class MoodOdorTest(unittest.TestCase):

    def xtest_mood_null(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        move = BodyMove(body)
        hab = LocomotionVote(body)
        mood = Mood(body)
        odor_to = MoodToTest(mood)
        odor_away = MoodAwayTest(mood)
        
        body.build()

        for i in range(5):        
            body.tick()
            log.debug(body)
            assert str(body) == "Body(0.00,0.00,d=0,s=0)[idle][]"
            log.debug(hab)
            assert str(hab) == "LocomotionVote[None,turn=0.00,to=0.00,away=0.00]"
        pass

    def xtest_mood_to(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        move = BodyMove(body)
        hab = LocomotionVote(body)
        mood = Mood(body)
        odor_to = MoodToTest(mood)
        odor_away = MoodAwayTest(mood)
        
        body.build()
        
        world.add_object(Odor((5, 0), "to"))

        for d in [0, 0, 0.05, 0.1, 0.15,
                  0.2, 0.24, 0.27, 0.28, 0.29,
                  0.28, 0.27, 0.26, 0.25, 0.24]:
            body.tick()
            log.debug(body)
            assert str(body) == f"Body(0.00,0.00,d={d},s=0)[idle][]"

    def xtest_mood_away(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        move = BodyMove(body)
        hab = LocomotionVote(body)
        mood = Mood(body)
        odor_to = MoodToTest(mood)
        odor_away = MoodAwayTest(mood)
        
        body.build()
        
        world.add_object(Odor((5, 0), "away"))

        for d in [0, 0, 0.95, 0.9, 0.85,
                  0.8, 0.76, 0.73, 0.72, 0.71,
                  0.72, 0.73, 0.74, 0.75, 0.76]:
            body.tick()
            log.debug(f"{d} {body}")
            assert str(body) == f"Body(0.00,0.00,d={d},s=0)[idle][]"

    def xtest_mood_to_and_away(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        move = BodyMove(body)
        hab = LocomotionVote(body)
        mood = Mood(body)
        odor_to = MoodToTest(mood)
        odor_away = MoodAwayTest(mood)
        
        body.build()
        
        world.add_object(Odor((-5, 0), "to"))
        world.add_object(Odor((5, 0), "away"))

        for d in [0, 0, 0.95, 0.9, 0.85,
                  0.8, 0.76, 0.73, 0.73, 0.73,
                  0.74, 0.74, 0.75, 0.75, 0.75]:
            body.tick()
            log.debug(f"{d} {body}")
            assert str(body) == f"Body(0.00,0.00,d={d},s=0)[idle][]"

    def xtest_mood_to_and_away_colocated(self):
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        move = BodyMove(body)
        hab = LocomotionVote(body)
        mood = Mood(body)
        odor_to = MoodToTest(mood)
        odor_away = MoodAwayTest(mood)
        
        body.build()
        
        world.add_object(Odor((5, 0), "to"))
        world.add_object(Odor((5, 0), "away"))

        for d in [0, 0, 0.95, 0.9, 0.85,
                  0.8, 0.76, 0.73, 0.72, 0.71,
                  0.72, 0.73, 0.74, 0.75, 0.76]:
            body.tick()
            log.debug(f"{d} {body}")
            assert str(body) == f"Body(0.00,0.00,d={d},s=0)[idle][]"

    def test_mood_to_and_away_ordered(self):
        '''
        'away' is further from 'to', but mood/hab doesn't have priority. 
        '''
        world = World()
        body = Body(world)
        nose = BodyNose(body)
        move = BodyMove(body)
        hab = LocomotionVote(body)
        mood = MoodSuppressMax(body)
        mood.global_suppress_max = 0.5
        odor_to = MoodToTest(mood)
        odor_away = MoodAwayTest(mood)
        
        body.build()
        
        world.add_object(Odor((5, 0), "to"))
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood[]"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.to']"
        
        world.add_object(Odor((5, 0), "away"))
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.to']"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.to']"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.to']"
        
        world.remove_local_object((5, 0))
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.to']"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood[]"
        
        world.add_object(Odor((5, 0), "away"))
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood[]"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.away']"
        
        world.add_object(Odor((5, 0), "to"))
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.away']"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.away']"
        
        body.tick()
        log.debug(mood)
        assert str(mood) == "Mood['odor.away']"

class MoodToTest(OdorMoodApproach):
    def __init__(self, mood):
        super().__init__(mood, 'to')

class MoodAwayTest(OdorMoodAvoid):
    def __init__(self, mood):
        super().__init__(mood, 'away')

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()