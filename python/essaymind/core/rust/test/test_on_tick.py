import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import TickerSystemBuilderRust, ticks

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class TickerTest(unittest.TestCase):

    def test_one(self):
        system_builder = TickerSystemBuilderRust()

        test_a = TestTicker("a")
        ticker_a = system_builder.ticker("a")
        ticker_a.on_tick(test_a.ticks)

        log.info(ticks())
        assert ticks() == 0

        log.info(str(test_a))
        assert test_a.take() == "a[]"

        system = system_builder.build()
        assert ticks() == 0
        assert test_a.take() == "a[]"

        system.tick()
        assert ticks() == 1
        log.info(str(test_a))
        assert test_a.take() == "a['a-ticks(1,1)']"

        system.tick()
        system.tick()
        system.tick()
        assert ticks() == 4
        log.info(str(test_a))
        assert test_a.take() == "a['a-ticks(2,2)', 'a-ticks(3,3)', 'a-ticks(4,4)']"
        pass

    def xtest_multi(self):
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

        assert test_a.take() == None
        assert test_b.take() == None
        assert test_b1.take() == None
        assert test_b_nil.take() == None
        
        system.tick()

        log.debug(test_a.value)
        assert test_a.take() == "a-Ticker ticks=1 count=2"
        log.debug(test_b.value)
        assert test_b.take() == "b-Ticker ticks=1 count=2"
        log.debug(test_b1.value)
        assert test_b1.take() == "b1-Ticker ticks=1 count=2"
        log.debug(test_b_nil.value)
        assert test_b_nil.take() == "b_nil-Ticker ticks=1 count=2"

        assert test_a.take() == None
        assert test_b.take() == None
        assert test_b1.take() == None
        assert test_b_nil.take() == None
        
        system.tick()

        log.debug(test_a.value)
        assert test_a.take() == "a-Ticker ticks=2 count=4"
        log.debug(test_b.value)
        assert test_b.take() == "b-Ticker ticks=2 count=4"
        log.debug(test_b1.value)
        assert test_b1.take() == "b1-Ticker ticks=2 count=4"
        log.debug(test_b_nil.value)
        assert test_b_nil.take() == "b_nil-Ticker ticks=2 count=4"

        pass

class TestTicker:
    def __init__(self, name):
        self.name = name
        self.values = []

    def build(self):
        self.values.append(f"{self.name}-build")

    def tick(self, ticks):
        self.values.append(f"{self.name}-tick({ticks})")

    def tick2(self, ticks):
        self.values.append(f"{self.name}-tick2({ticks})")

    def ticks(self, n_ticks):
        self.values.append(f"{self.name}-ticks({n_ticks},{ticks()})")

    def take(self):
        value = str(self)
        self.values = []
        return value

    def __str__(self):
        return f"{self.name}{self.values}"

if __name__ == "__main__":
    unittest.main()
