from motion.motion_vote import MotionVote
from mood.mood import MoodFreeze

import logging

log = logging.getLogger("Freeze")

class ActionFreeze(MoodFreeze):
    def __init__(self, area, name='freeze'):
        super().__init__(area, name)
        
        self.on_freeze = area.top.nexus(name)
        
    def build(self):
        super().build()
        
        self.motion = MotionVote.from_body(self.area.top)

        #PainNexus.add_from_body(body, self.selector)
        
    def action(self):
        log.info("freeze")
        self.motion.freeze(1)
        self.on_freeze.send()
