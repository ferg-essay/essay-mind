class Config(object):
    def __init__(self):
        self.config_map = dict()
        
    def __setitem__(self, name, value):
        self.config_map[name] = value
        
    def get(self, name, default):
        while True:
            value = self.config_map.get(name)
            
            if value != None:
                return value
            
            p = name.rfind('.')
            
            if p >= 0:
                name = name[0:p]
            else:
                return default
        