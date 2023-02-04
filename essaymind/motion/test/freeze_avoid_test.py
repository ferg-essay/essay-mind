import unittest

from world.world import World
from body.body import Body
from body.body_move import BodyMove
from mood.mood import Moods
from motion.explore import MoodExplore, ActionExplore
from motion.freeze import ActionFreeze
from motion.motion_vote import MotionVote
import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class FreezeAvoidTest(unittest.TestCase):


    def testName(self):
        world = World()
        body = Body(world)
        BodyMove(body)
        
        MotionVote(body)

        area_mood = Moods(body, 'mood')
        area_action = Moods(body, 'action')
        
        mood_explore = MoodExplore(area_mood)
        action_explore = ActionExplore(area_action)
        
        action_freeze = ActionFreeze(area_action)
        
        nexus_pain = body.nexus('pain')
        nexus_freeze = body.nexus('freeze')
        
        nexus_freeze.add(mood_explore.freeze)
        
        selector_freeze = area_action.selector('freeze')
        selector_freeze.add(action_freeze.send)
        
        nexus_pain.add(selector_freeze.select)
        #nexus_pain.add(mood_explore.pain)
                
        body.build()
        
        body.tick()
        log.info(area_mood)
        assert str(area_mood) == "mood-Moods['explore']"
        log.info(area_action)
        assert str(area_action) == "action-Moods[]"
        
        body.tick()
        log.info(area_mood)
        assert str(area_mood) == "mood-Moods['explore']"
        log.info(area_action)
        assert str(area_action) == "action-Moods[]"
        
        body.tick()
        log.info(area_mood)
        assert str(area_mood) == "mood-Moods['explore']"
        log.info(area_action)
        assert str(area_action) == "action-Moods[]"

        nexus_pain.send()
        body.tick()
        log.info(area_mood)
        assert str(area_mood) == "mood-Moods['explore']"
        log.info(area_action)
        assert str(area_action) == "action-Moods['freeze']"

        nexus_pain.send()
        body.tick()
        log.info(area_mood)
        assert str(area_mood) == "mood-Moods['explore']"
        log.info(area_action)
        assert str(area_action) == "action-Moods['freeze']"
        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()