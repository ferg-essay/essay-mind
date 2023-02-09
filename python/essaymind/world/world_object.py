class WorldObject:
    def __init__(self, loc, attrs = {}):
        assert isinstance(attrs, dict)

        self.loc = loc
        self.attrs = attrs
        
    def attr(self, name):
        return self.attrs.get(name)
    
    def tick(self, body):
        return
    
    def __str__(self):
        return f'WorldObject{self.loc}{self.attrs}'