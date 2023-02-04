class Retina:
    def __init__(self):
        self.v = 0
        
class OnNeuron:
    def process(self, data_in, data_out):
        for j in range(0, len(data_out), 2):
            row_in0 = data_in[j]
            row_in1 = data_in[j + 1]
            row_in2 = data_in[j + 2]
            row_in3 = data_in[j + 3]
            
            row_out0 = data_out[j]
            row_out1 = data_out[j + 1]
            
            value_neg = row_in0[0] + row_in0[1]
            value_neg += (row_in0[0] + row_in2[0]) / 2 + row_in1[1]
            value_neg += row_in2[0] + row_in2[1]
            value_pos = row_in1[0]
            value = max(0, value_pos - value_neg / 6)
            row_out0[0] = value
                
            value_neg = row_in1[0] + row_in1[1]
            value_neg += row_in2[0] + row_in2[2]
            value_neg += row_in3[0] + row_in3[1]
            value_pos = row_in2[1]
            value = max(0, value_pos - value_neg / 6)
            row_out1[0] = value
            
            for i in range(1, len(row_out0)):
                value_neg = row_in0[i] + row_in0[i + 1]
                value_neg += row_in1[i - 1] + row_in1[i + 1]
                value_neg += row_in2[i] + row_in2[i + 1]
                value_pos = row_in1[i]
                value = max(0, value_pos - value_neg / 6)
                row_out0[i] = value
                
                value_neg = row_in1[i] + row_in1[i + 1]
                value_neg += row_in2[i] + row_in2[i + 2]
                value_neg += row_in3[i] + row_in3[i + 1]
                value_pos = row_in2[i + 1]
                value = max(0, value_pos - value_neg / 6)
                row_out1[i] = value
        
class OffNeuron:
    def process(self, data_in, data_out):
        for j in range(len(data_out)):
            row_in0 = data_in[j]
            row_in1 = data_in[j + 1]
            row_in2 = data_in[j + 2]
            row_out = data_out[j]
            
            for i in range(len(row_out)):
                value_neg = row_in0[i] + 2 * row_in0[i + 1] + row_in0[i + 2]
                value_neg += 2 * (row_in1[i] + row_in1[i + 2])
                value_neg += row_in2[i] + 2 * row_in2[i + 1] + row_in2[i + 2]
                value_pos = row_in1[i + 1]
                value = max(0, value_neg / 12 - value_pos)
                row_out[i] = value
