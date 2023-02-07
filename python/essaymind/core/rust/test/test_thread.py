import logging
import unittest

from essaymind import FiberKeyValue
from essaymind._essaymind import test_thread_py;

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class ThreadRustTest(unittest.TestCase):

    def test_basic(self):
        log.info("test")

        test_thread_py()
        # node_life()
        pass

class Test:
    def my_call(self, id, key, value, p):
        log.info(f"call: {id} {key}")

def test_fn(id, key, value, p):
    print(f"test {id} {key} {value} {p}")

if __name__ == "__main__":
    unittest.main()
