import unittest

from graphics.retina import OffNeuron

class Test(unittest.TestCase):
    def round_value(self, value):
        for row in value:
            for i in range(len(row)):
                row[i] = round(row[i], 2)

    def testBase_1_1(self):
        retina_off = OffNeuron()
        value = [[0]]
        
        data = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0]]
        
        data = [[1, 1, 1], [1, 1, 1], [1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0]]
        
        data = [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0]]
        
        data = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[1]]
        
        data = [[0, 0, 0], [1, 0, 0], [0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.17]]
        
        data = [[1, 1, 1], [0, 0, 0], [1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.67]]
        
        data = [[1, 1, 1], [0, 0, 0], [0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.33]]
        
        data = [[0, 0, 0], [0, 0, 0], [1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.33]]

        pass
    
    def testBase_2_2(self):
        retina_off = OffNeuron()
        value = [[0, 0], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        # single
        data = [[1, 1, 1, 1], [1, 0, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[1, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 0, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 1], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 0, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [1, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 0, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 1]]
        
        # horizontal line
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [0, 0, 0, 0], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0.67, 0.67]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.33, 0.33], [0, 0]]
        
        # horizontal block
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0.33, 0.33]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.33, 0.33], [0, 0]]
        
        data = [[0, 0, 0, 0], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [0, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        # vertical block
        data = [[0, 0, 0, 1], [0, 0, 0, 1], [0, 0, 0, 1], [0, 0, 0, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.33], [0, 0.33]]
        
        data = [[0, 0, 1, 1], [0, 0, 1, 1], [0, 0, 1, 1], [0, 0, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.33, 0], [0.33, 0]]
        
        data = [[0, 1, 1, 1], [0, 1, 1, 1], [0, 1, 1, 1], [0, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 1, 0], [1, 1, 1, 0], [1, 1, 1, 0], [1, 1, 1, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.33], [0, 0.33]]
        
        data = [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.33, 0], [0.33, 0]]
        
        # triangles
        data = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.08, 0], [0, 0]]
        
        data = [[1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.42, 0.08], [0.08, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 0], [1, 1, 0, 0], [1, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0], [0, 0.42]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 0], [1, 1, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0.0], [0.0, 0.0]]
        
        data = [[0, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0.0], [0.0, 0.0]]
        
        data = [[0, 0, 1, 1], [0, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0.0], [0.0, 0.0]]

        data = [[0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.42, 0], [0, 0]]

        data = [[0, 0, 0, 0], [0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.08, 0.42], [0.42, 0]]

        # checkerboard
        data = [[0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.67, 0], [0, 0.67]]
        
        data = [[1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.67], [0.67, 0]]
        
        # vertical stripes
        data = [[1, 0, 1, 0], [1, 0, 1, 0], [1, 0, 1, 0], [1, 0, 1, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.67, 0], [0.67, 0]]
        
        data = [[0, 1, 0, 1], [0, 1, 0, 1], [0, 1, 0, 1], [0, 1, 0, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.67], [0, 0.67]]

        # horizontal stripes       
        data = [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [1, 1, 1, 1]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0.67, 0.67]]

        data = [[1, 1, 1, 1], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]
        retina_off.process(data, value)
        self.round_value(value)
        assert value == [[0.67, 0.67], [0, 0]]

        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    unittest.main()