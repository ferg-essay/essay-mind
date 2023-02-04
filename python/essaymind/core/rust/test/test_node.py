import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import Node, node_life

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class RustNodeTest(unittest.TestCase):

    def test_basic(self):
        log.info("test")
        log.info(Node())
        # node_life()
        pass

if __name__ == "__main__":
    unittest.main()
