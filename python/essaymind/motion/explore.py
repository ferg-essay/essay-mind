from body.node import BodyNode
import logging
from motion.motion_vote import MotionVote
#from body.selector import Selector
#from touch.touch import PainNexus
from mood.mood import MoodToward, MOOD_TOWARD
from body.category_area import CategoryNode

log = logging.getLogger("Explore")

class MoodExplore(MoodToward):
    def __init__(self, area, name='explore'):
        super().__init__(area, name)

        self.is_turn = False
        
        area.selector(name).add(self.send)
        
        self.area_move = area.side.area('action')
        #self.explore = self.area_move.action('explore')
        #self.freeze = self.area_move.action('freeze')
        #self.away = self.area_move.action('away')
        
        #PainNexus.add_from_body(body, self.pain)

    def build(self):
        super().build()
        
        action_area = self.area.side.area('action')
        self.explore = action_area.selector('explore')
        self.freeze = self.area_move.selector('freeze')
        self.motion = MotionVote.from_body(self.area.top)
        self.is_pain_left = False
        self.is_pain_right = False
        self.is_freeze = True
        
    def pain(self):
        self.is_pain_left = True
        
    def pain_left(self):
        self.is_pain_left = True
        
    def pain_right(self):
        self.is_pain_right = True
        
    def cortex_pain_left(self, p):
        #log.info(f"cortex-pain {p}")
        self.is_pain_left = True
        
    def cortex_pain_right(self, p):
        #log.info(f"cortex-pain {p}")
        self.is_pain_right = True
        
    def freeze(self):
        self.is_freeze = True
        
    def tick(self):
        self.send()
        
        super().tick()
        
    def action(self):
        is_pain_left = self.is_pain_left
        is_pain_right = self.is_pain_right
        self.is_pain_left = False
        self.is_pain_right = False
        
        if is_pain_left:
            log.info(f"right-turn explore pain-left:{is_pain_left}")
            #self.freeze.suppress()
            self.motion.explore(0.01, 0, 0.1)
        elif is_pain_right:
            log.info(f"left-turn explore pain-right:{is_pain_right}")
            #self.freeze.suppress()
            self.motion.explore(0.01, 0.1, 0)
        else:
            self.motion.explore(0.1, 0.0, 0)
        #self.explore.select()

class ActionExplore(MoodToward):
    def __init__(self, area, name='explore'):
        super().__init__(area, name)
        self.body = area
        #area[name] = self

        self.is_turn = False
        
        area.selector(name).add(self.send)
        #selector_freeze = Selector.target_from_body(body, "locomotion", "freeze")
        #PainNexus.add_from_body(body, selector_freeze.select)

    def build(self):
        super().build()
        
        self.motion = MotionVote.from_body(self.body)
        assert self.motion
        
    def turn_request(self):
        self.is_turn = True
        
    def action(self):
        is_turn = self.is_turn
        self.is_turn = False

        log.info(f"explore-action {is_turn}")
        
        if is_turn:        
            self.motion.explore(0.01, 0.05, 0)
        else:
            self.motion.explore(0.1, 0, 0)
