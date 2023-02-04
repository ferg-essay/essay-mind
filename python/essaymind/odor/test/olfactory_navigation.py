import logging
import unittest

from body.body import Body
#from symbol.Front import Front
from world.world import World, WorldObject
from odor.olfactory import Olfactory
from navigation.navigation import Triangulate, TriangulateApproach, NavigationMux
#from symbol.event.event_navigation import MemoryNavigation
#from symbol.z_old.event_mood_old import EventMood
from event.event_complex import EventComplex
from navigation.locomotion import Locomotion

logging.basicConfig(level=logging.INFO,
                    format='%(levelname)s:%(name)-10s: %(message)s')

class OlfactoryNavigationTest(unittest.TestCase):
    def test_chemotaxis_navigation_mood(self):
        world = World()
        body = Body(world)
        #olf = body.add_node("olf", Olfactory(['f', 'z']))
        olf = Olfactory(body, ['f', 'z'])
        # body.add_node("olf.triangulate", TriangulateApproach(olf))
        # body.add_node("olf.triangulate.node.f", TriangulateNode('olf-f', olf.get_node('f')))
        # body.add_node("olf.triangulate.node.z", TriangulateNode('olf-z', olf.get_node('z')))
        
        #loco = body.add_node("locomotion", Locomotion(body))
        loco = Locomotion(body)
        
        #amy = body.add_node("amygdala", Amygdala(body))
        #amy.ensure_target(0)
        #amy['f'] = 'food'
        #amy['z'] = 'ignore'
        
        # nav = body.add_node("navigation", NavigationColumn())
        #mux = body.add_node("olf.triangulate.mux", NavigationMux('olf.tri'))
        #mux = body.add_node("olf.triangulate.mux", NavigationMux('olf.tri'))
        
        memory = EventComplex(body)
        
        #nav = MemoryNavigation(body)
        #nav.ensure_target(0)
        
        #mood = EventMood(body)
        #mood['f'] = 'food'
        #mood['z'] = 'ignore'
        
        memory['f'] = 'food'
        memory['z'] = 'ignore'

        olf.add_target(memory)
        # olf.add_target(mood)
        
        #front = body.add_node("front", Front())
        #front.add_input(olf)
        
        body.build()
        body.speed_request = 0.5
        
        world.add_object(WorldObject((5, 1), {"scent": 30, 'key': 'f'}))
        for i in range(5):
            body.tick()
            
        print(body)
        # assert body.__str__() == "Body(0.088,0.21)[]"
        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
    