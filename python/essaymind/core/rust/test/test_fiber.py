import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import TickerSystemBuilderRust

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class RustNodeTest(unittest.TestCase):

    def test_basic(self):
        log.info("test")

        system_builder = TickerSystemBuilderRust()
        log.info(system_builder)

        ticker_a = system_builder.ticker("a")
        #fiber = builder.fiber_key("a")
        fiber_builder = ticker_a.fiber("a")
        ticker_a.on_fiber(fiber_builder, test_fn)

        #log.info(builder.fiber_key("b"))

        ticker_a.on_fiber(fiber_builder, test_fn)
        test = Test()
        ticker_a.on_fiber(fiber_builder, test.my_call)
        system_builder.build()
        fiber = fiber_builder.fiber()
        log.info(fiber)

        my_call = test.my_call
        log.info(my_call)

        fiber("zorp", 1.3, 0.5)

        # node_life()
        pass

class Test:
    def my_call(self, id, key, value, p):
        log.info(f"call: {id} {key}")

def test_fn(id, key, value, p):
    print(f"test {id} {key} {value} {p}")

if __name__ == "__main__":
    unittest.main()
