import unittest

from body.body import Body
from tectum.tectum import Tectum

import logging
from body.fiber import FiberAngle

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class TestTectum(unittest.TestCase):

    def test_toward(self):
        body = Body()
        body.random_max(True)
        tectum = Tectum(body)
        
        toward = FiberAngle()
        tectum.when_toward("test", toward)
        tectum.move_ticks(2)
        
        body.build()
        
        log.info(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        toward("test", 0)
        
        body.tick()
        assert str(body) == "Body(0.00,0.00,d=0,s=0)['tectum.actions.to.0']"
        
        body.tick()
        assert str(body) == "Body(0.00,0.10,d=0,s=0.1)[]"
        
        body.tick()
        assert str(body) == 'Body(0.00,0.20,d=0,s=0.1)[]'
        
        body.tick()
        assert str(body) == 'Body(0.00,0.20,d=0,s=0)[]'
        
        body.tick()
        assert str(body) == 'Body(0.00,0.20,d=0,s=0)[]'
        
        toward("test", 0.24)
        
        body.tick()
        log.info(body)
        assert str(body) == "Body(0.00,0.20,d=0,s=0)['tectum.actions.to.17']"
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.05,0.29,d=0.083,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.14,0.34,d=0.17,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.14,0.34,d=0.17,s=0)[]'
        
        toward("test", 0.76)
        
        body.tick()
        log.info(body)
        assert str(body) == "Body(0.14,0.34,d=0.17,s=0)['tectum.actions.to.83']"
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.19,0.42,d=0.083,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.19,0.52,d=1,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.19,0.52,d=1,s=0)[]'
        
        toward("test", 1.0 / 12 - 0.01)
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.19,0.52,d=1,s=0)['tectum.actions.to.0']"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.19,0.62,d=1,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.19,0.72,d=1,s=0.1)[]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.19,0.72,d=1,s=0)[]'
        
        toward("test", 11.0 / 12 + 0.01)
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.19,0.72,d=1,s=0)['tectum.actions.to.0']"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.19,0.82,d=1,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.19,0.92,d=1,s=0.1)[]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.19,0.92,d=1,s=0)[]'

    
    def test_away(self):
        body = Body()
        body.random_max(True)
        tectum = Tectum(body)
        
        away = FiberAngle()
        tectum.when_away("test", away)
        tectum.move_ticks(2)
        
        body.build()
        
        log.info(body)
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        body.tick()
        assert str(body) == 'Body(0.00,0.00,d=0,s=0)[]'
        
        away("test", 0)
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(0.00,0.00,d=0,s=0)['tectum.actions.away.75']"
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(-0.07,0.07,d=0.88,s=0.1)[]"
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(-0.17,0.07,d=0.75,s=0.1)[]'
        
        body.tick()
        assert str(body) == 'Body(-0.17,0.07,d=0.75,s=0)[]'
        
        body.tick()
        assert str(body) == 'Body(-0.17,0.07,d=0.75,s=0)[]'
        
        away("test", 0.24)
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(-0.17,0.07,d=0.75,s=0)['tectum.actions.away.75']"
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(-0.24,0.00,d=0.62,s=0.1)[]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(-0.24,-0.10,d=0.5,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.24,-0.10,d=0.5,s=0)[]'
        
        away("test", 0.01)
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(-0.24,-0.10,d=0.5,s=0)['tectum.actions.away.75']"
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(-0.17,-0.17,d=0.38,s=0.1)[]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(-0.07,-0.17,d=0.25,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(-0.07,-0.17,d=0.25,s=0)[]'
        
        away("test", 0.99)
        
        body.tick()
        log.debug(body)
        assert str(body) == "Body(-0.07,-0.17,d=0.25,s=0)['tectum.actions.away.25']"
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,-0.24,d=0.38,s=0.1)[]'
        
        body.tick()
        log.debug(body)
        assert str(body) == 'Body(0.00,-0.34,d=0.5,s=0.1)[]'
        
        body.tick()
        log.info(body)
        assert str(body) == 'Body(0.00,-0.34,d=0.5,s=0)[]'

        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()