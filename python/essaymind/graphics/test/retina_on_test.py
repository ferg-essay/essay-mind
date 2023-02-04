import unittest

from graphics.retina import OnNeuron

class Test(unittest.TestCase):
    def round_value(self, value):
        for row in value:
            for i in range(len(row)):
                row[i] = round(row[i], 2)

    def testBase_1_2(self):
        retina_on = OnNeuron()
        value = [[0, 0], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        # x=0 special case
        data = [[0, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[1, 0], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [1, 0]]
        
        # x=1 center dot
        data = [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 1], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 1]]
        
        # x=1 center dot + UL
        data = [[0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0.83], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0.83], [0, 0.83]]
        
        # x=1 center dot + UR
        data = [[0, 0, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0.83], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0, 0.83]]
        
        # x=1 center dot + L
        data = [[0, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0.83, 0.83], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0.83, 0.83]]
        
        # x=1 center dot + R
        data = [[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0.83], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 1], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0, 0.83]]
        
        # x=1 center dot + LL
        data = [[0, 0, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0.83], [0.83, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 1, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0, 0.83]]

        pass
    
    def testBase_2_2(self):
        retina_on = OnNeuron()
        value = [[0, 0], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        # single
        data = [[0, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[1, 0], [0, 0]]
        
        # single
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [1, 0]]
        
        data = [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 1], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [1, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 1]]
        
        # horizontal line
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0.67, 0.67]]
        
        # horizontal block
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0.33, 0.33]]
        
        data = [[0, 0, 0, 0], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0.42, 0.33], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0.33, 0.33]]
        
        # vertical block
        data = [[0, 0, 0, 1], [0, 0, 0, 1], [0, 0, 0, 1], [0, 0, 0, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[0, 0, 1, 1], [0, 0, 1, 1], [0, 0, 1, 1], [0, 0, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0, 0.5]]
        
        data = [[0, 1, 1, 1], [0, 1, 1, 1], [0, 1, 1, 1], [0, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0.17], [0.5, 0]]
        
        data = [[1, 1, 1, 0], [1, 1, 1, 0], [1, 1, 1, 0], [1, 1, 1, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        print(value)
        assert value == [[0, 0], [0, 0.17]]
        
        data = [[1, 1, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.33, 0], [0.33, 0]]
        
        data = [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        # triangles
        data = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        # triangles
        data = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 0], [1, 1, 0, 0], [1, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.08, 0.42], [0.42, 0]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 0], [1, 1, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0.08], [0.08, 0.42]]
        
        data = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.0, 0.0], [0.0, 0.08]]
        
        data = [[0, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.08, 0.0], [0.0, 0.0]]
        
        data = [[0, 0, 1, 1], [0, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.42, 0.08], [0.08, 0.0]]

        data = [[0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.42], [0.42, 0.08]]

        data = [[0, 0, 0, 0], [0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0, 0.42]]

        # checkerboard
        data = [[0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.67], [0.67, 0]]
        
        data = [[1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.67, 0], [0, 0.67]]
        
        # vertical stripes
        data = [[1, 0, 1, 0], [1, 0, 1, 0], [1, 0, 1, 0], [1, 0, 1, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0.67], [0, 0.67]]
        
        data = [[0, 1, 0, 1], [0, 1, 0, 1], [0, 1, 0, 1], [0, 1, 0, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.67, 0], [0.67, 0]]

        # horizontal stripes       
        data = [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [1, 1, 1, 1]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0.67, 0.67], [0, 0]]

        data = [[1, 1, 1, 1], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]
        retina_on.process(data, value)
        self.round_value(value)
        assert value == [[0, 0], [0.67, 0.67]]

        pass


if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testBase']
    unittest.main()