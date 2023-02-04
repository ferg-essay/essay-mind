import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import FiberBuilder, MindNode as RustMindNode

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class RustNodeTest(unittest.TestCase):

    def test_basic(self):
        log.info("test")

        builder = FiberBuilder()
        log.info(builder)
        log.info(builder.fiber_id())
        log.info(builder.fiber_id())
        fiber = builder.fiber_key("a")
        log.info(fiber)
        log.info(builder.fiber_key("b"))

        fiber("zorp", 1.3, 0.5)
        node = RustMindNode("my-node")
        log.info(node)

        # node_life()
        pass

if __name__ == "__main__":
    unittest.main()
