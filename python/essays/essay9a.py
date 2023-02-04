import unittest

import logging
from essaymind import _essaymind

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class Essay9(unittest.TestCase):

    def test_basic(self):
        log.info("test")
        log.info(_essaymind.sum_as_string(1, 3))
        pass
        
if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()