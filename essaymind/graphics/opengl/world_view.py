class WorldView(object):
    def __init__(self, world_map):
        self.world_map = world_map
        self.ground_color = (0.8, 0.6, 0.3, 1)
        self.wall_color = (0.8, 0.7, 0.4, 1)
        self.sky_color = (0.5, 0.7, 0.8, 1)
        
    def build(self, scene):
        thin = 0.1
        length = self.world_map.length
        width = self.world_map.width
        t2 = 2 * thin
        
        scene.color(self.sky_color)
        scene.ambient(1)
        scene.add_cube((-t2, -1, -length - t2), ((width + 2 * t2), 20, length + 2 * t2)) 
 
        scene.color(self.ground_color)
        scene.add_cube((-thin, -thin, -length - thin),
                       (width + 2 * thin, thin, length + 2 * thin))
        
       
        scene.color(self.wall_color)
        scene.add_cube((-thin, 0, -length), (thin, 1, length))
        scene.add_cube((width, 0, -length), (thin, 1, length))
        scene.add_cube((-thin, 0, 0), ((width + 2 * thin), 1, thin))
        scene.add_cube((-thin, 0, -length - thin), ((width + 2 * thin), 1, thin))
        
        for j in range(width):
            for i in range(length):
                value = self.world_map[(i, j)]
                
                if value > 0:
                    scene.add_cube((i, 0, -j - 1), (1, value, 1))