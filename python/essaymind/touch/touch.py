from body.node import BodyNode
from body.fiber import Fiber, FiberKeyValue
import logging
from motion.motion_vote import MotionVote

log = logging.getLogger("Touch")

class BodyTouch(BodyNode):
    def __init__(self, parent, name='touch'):
        super().__init__(parent, name)
        
        #PainFreeze(body)
        self.pain_tick_max = self.top.config.get('pain.ticks', 1)
        self.pain_left_tick_end = 0
        self.pain_right_tick_end = 0
        self._ticks = 0
        
        self.pain_nexus = FiberKeyValue()
        self.pain_left_nexus = FiberKeyValue()
        self.pain_right_nexus = FiberKeyValue()
        self.moveto_left_nexus = FiberKeyValue()
        self.moveto_right_nexus = FiberKeyValue()
        
    def on_left(self, target):
        self.pain_left_nexus.to(target)
        
        return self
        
    def on_right(self, target):
        self.pain_right_nexus.to(target)
        
        return self
        
        
    def ticks(self, ticks):
        assert ticks > 0
        self.pain_tick_max = ticks
        return self
        
    def build(self):
        self.moveto_left_nexus.to(self.moveto_left)
        self.moveto_right_nexus.to(self.moveto_right)
        
    def moveto_left(self):
        log.debug("left-pain body-touch")
        self.pain_left_tick_end = self.ticks + self.pain_tick_max
        
    def moveto_right(self):
        log.debug("right-pain body-touch")
        self.pain_right_tick_end = self.ticks + self.pain_tick_max
        
    def tick(self):
        ticks = self._ticks
        self._ticks = ticks + 1
        
        if ticks < self.pain_left_tick_end:
            log.info("pain-left")
            self.pain_left_nexus("pain-left", 1, 1)
            self.pain_nexus.send()
        
        if ticks < self.pain_right_tick_end:
            log.info("pain-right")
            self.pain_right_nexus("pain-right", 1, 1)
            self.pain_right_nexus.send()
            self.pain_nexus.send()
            
class PainFreeze(BodyNode):
    def __init__(self, parent, name='pain.freeze'):
        super().__init__(parent, name)
        
    def build(self, body):
        #PainNexus.add_from_body(body, lambda: self.pain())
        PainNexus.add_from_body(body, self.pain)
        #pain.add_listener(self)
        
        self.motion = MotionVote.from_body(body)
        
        self.is_pain = False
        
    def pain(self):
        log.info("pain")
        assert False
        self.is_pain = True
        
    def tick(self, body):
        is_pain = self.is_pain
        self.is_pain = False
        
        if is_pain:
            log.info("pain-freeze")
            self.locomotion.freeze(1)

        