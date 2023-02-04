import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import FiberBuilder, FiberKey, MindNode as RustMindNode

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class RustNodeTest(unittest.TestCase):

    def test_basic(self):
        log.info("test")

        builder = FiberBuilder()
        log.info(builder)

        fiber = builder.fiber_key("a")
        log.info(fiber)
        log.info(builder.fiber_key("b"))

        fiber.to(test_fn)

        fiber("zorp", 1.3, 0.5)
        node = RustMindNode("my-node")
        log.info(node)

        # node_life()
        pass

def test_fn(id, key, value, p):
    print(f"test {id} {key} {value} {p}")

if __name__ == "__main__":
    unittest.main()
