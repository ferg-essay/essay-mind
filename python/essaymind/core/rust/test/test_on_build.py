import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import TickerSystemBuilderRust

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class TickerOnBuild(unittest.TestCase):

    def xtest_empty(self):

        system_builder = TickerSystemBuilderRust()

        system = system_builder.build();
        log.info(system)

        system.tick()

        pass

    def xtest_on_build_one(self):
        test_a = TestTicker("a")
        system_builder = TickerSystemBuilderRust()
        ticker_a = system_builder.ticker("a")
        ticker_a.on_build(test_a.build)
        ticker_a.on_tick(test_a.tick)

        log.info(test_a)
        assert test_a.take() == "a[]"

        system = system_builder.build()

        log.info(test_a)
        assert test_a.take() == "a['a-build']"

        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(1)']"

        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(2)']"

        system.tick()
        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(3)', 'a-tick(4)']"

        pass

    def xtest_on_build_without_on_build(self):
        test_a = TestTicker("a")
        system_builder = TickerSystemBuilderRust()
        ticker_a = system_builder.ticker("a")
        ticker_a.on_tick(test_a.tick)

        log.info(test_a)
        assert test_a.take() == "a[]"

        system = system_builder.build()

        log.info(test_a)
        assert test_a.take() == "a[]"

        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(1)']"

        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(2)']"

        system.tick()
        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(3)', 'a-tick(4)']"

        pass

    def test_two_build(self):
        test_a = TestTicker("a")
        system_builder = TickerSystemBuilderRust()
        ticker_a = system_builder.ticker("a")
        ticker_a.on_build(test_a.build)
        ticker_a.on_tick(test_a.tick)

        ticker_b = system_builder.ticker("b")
        ticker_b.on_build(test_a.build2)

        log.info(test_a)
        assert test_a.take() == "a[]"

        system = system_builder.build()

        log.info(test_a)
        assert test_a.take() == "a['a-build', 'a-build2']"

        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(1)']"

        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(2)']"

        system.tick()
        system.tick()
        log.info(test_a)
        assert test_a.take() == "a['a-tick(3)', 'a-tick(4)']"

        pass

class TestTicker:
    def __init__(self, name):
        self.name = name
        self.values = []

    def build(self):
        self.values.append(f"{self.name}-build")

    def build2(self):
        self.values.append(f"{self.name}-build2")

    def tick(self, ticks):
        self.values.append(f"{self.name}-tick({ticks})")

    def take(self):
        value = str(self)
        self.values = []
        return value

    def __str__(self):
        return f"{self.name}{self.values}"

if __name__ == "__main__":
    unittest.main()
