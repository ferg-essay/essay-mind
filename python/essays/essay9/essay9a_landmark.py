import unittest


import logging

from essaymind.body.body import Body
from essaymind.body.action_probability import ActionProbabilityGroup
from essaymind.motion.move_probability_actions import MoveNode
from essaymind.odor.world_odor import Odor
from essaymind.odor.body_nose import BodyNose
from essaymind.tectum.tectum import Tectum

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')
logging.getLogger().setLevel(logging.DEBUG)

log = logging.getLogger("Test")
        

class Essay9aLandmark(unittest.TestCase):


    def test_nose_tectum(self):
        # direct nose to tectum connection

        body = Body()
        world = body.world
        body.seed(8)
        body.random_max(True)
        body.moveto(5, 5)
        
        nose = BodyNose(body)
        
        tectum = Tectum(body)
        tectum.move_ticks(2)
        tectum.when_toward("odor", nose.on_odor())
        
        body.build()
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.00,5.00,d=0,s=0)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.00,5.00,d=0,s=0)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.00,5.00,d=0,s=0)[]"
        
        world.add_object(Odor((9, 5), "coffee"))
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.00,5.00,d=0,s=0)['tectum.actions.to.33']"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.09,5.05,d=0.17,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.17,5.00,d=0.33,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.17,5.00,d=0.33,s=0)['tectum.actions.to.17']"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.22,4.91,d=0.42,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.22,4.81,d=0.5,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.22,4.81,d=0.5,s=0)['tectum.actions.to.83']"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.27,4.73,d=0.42,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(5.36,4.68,d=0.33,s=0.1)[]"

        pass
        
if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()