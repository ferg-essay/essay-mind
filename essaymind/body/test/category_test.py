import unittest

from body.body import Body, BodyNode
from body.category_area import CategoryArea, Category, CategoryNode
from world.world import World

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")


class CategoryTest(unittest.TestCase):

    def test_select(self):
        world = World()
        body = Body(world)
        
        send_area = CategoryArea(body, 'sender_area')  
        target_area = CategoryArea(body, 'target_area')
        
        sender = TestSender(send_area)  
        target = TestTarget(target_area)
        
        body.build()
        body.tick()
        
        log.info(sender)
        assert str(sender) == 'sender[False,sender-category[1]]'  
        log.info(target)  
        assert str(target) == 'target[False,target-category[2]]'
        
        target.send()
        body.tick()
        log.info(sender)
        assert str(sender) == 'sender[False,sender-category[1]]'  
        log.info(target)  
        assert str(target) == 'target[True,target-category[2]]'
        
        
        body.tick()
        log.info(sender)
        assert str(sender) == 'sender[False,sender-category[1]]'  
        log.info(target)  
        assert str(target) == 'target[False,target-category[2]]'

        sender.send()
        body.tick()
        log.info(sender)
        assert str(sender) == 'sender[True,sender-category[1]]'  
        log.info(target)  
        assert str(target) == 'target[True,target-category[2]]'
        
        body.tick()
        log.info(sender)
        assert str(sender) == 'sender[False,sender-category[1]]'  
        log.info(target)  
        assert str(target) == 'target[False,target-category[2]]'
        
        pass

SENDER = Category('sender-category', 1)
TARGET = Category('target-category', 2)

class TestSender(CategoryNode):
    def __init__(self, area, name='sender', category=SENDER):
        super().__init__(area, name, category)
        
    def build(self):
        super().build()
        
        self.target_area = self.area.side.area('target_area')
        self.target = self.target_area['target']
        
    def tick(self):
        super().tick()
        
        if self.value:
            self.target.send()
            log.info(f"action {self} {self.target}")

class TestTarget(CategoryNode):
    def __init__(self, area, name='target', category=TARGET):
        super().__init__(area, name, category)
        
    def tick(self):
        super().tick()
        
        if self.value:
            log.info(f"action {self}")

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()