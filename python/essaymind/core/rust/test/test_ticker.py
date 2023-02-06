import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import TickerSystemBuilderRust

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class TickerTest(unittest.TestCase):

    def test_basic(self):
        log.info("test")

        system_builder = TickerSystemBuilderRust()
        log.info(system_builder)

        test_a = TestTicker("a")
        ticker_a = system_builder.ticker("a")
        ticker_a.on_tick(test_a.tick)

        test_b = TestTicker("b")
        ticker_b = system_builder.ticker()
        ticker_b.name("b")
        ticker_b.on_tick(test_b.tick)

        test_b1 = TestTicker("b1")
        ticker_b1 = system_builder.ticker()
        ticker_b1.name("b1")
        ticker_b1.on_tick(test_b1.tick)

        test_b_nil = TestTicker("b_nil")
        ticker_b_nil = system_builder.ticker()
        ticker_b_nil.on_tick(test_b_nil.tick)

        test_none = TestTicker("none")
        ticker_none = system_builder.ticker()

        system = system_builder.build()
        
        system.tick()
        system.tick()

        pass

class TestTicker:
    def __init__(self, name):
        self.name = name
        self.count = 0

    def tick(self, ticks):
        self.count += 2
        print(f"{self.name}-Ticker ticks={ticks} count={self.count}")

if __name__ == "__main__":
    unittest.main()
