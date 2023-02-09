import numpy as np
import math
import logging

from world.world import World
from world.world_actor import WorldActor
from graphics.ray import RayTracing, RayScene

log = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)s:%(name)-10s: %(message)s')
    
class EyePipeline:
    def __init__(self, fov, w, world, actor, ego_angle=0, ego_height=0.2):
        self.eye_ray = EyeRay(fov, w, world, actor, ego_angle, ego_height)
        self.eye_blur = Eye6Blur(self.eye_ray.data)
        self.eye_onoff = Eye6OnOff(self.eye_blur.data)
        self.eye_border = Eye6Border(self.eye_onoff.on_data, self.eye_onoff.off_data)
        
    def tick(self):
        self.eye_ray.tick()
        
        self.eye_blur.tick()
        
        self.eye_onoff.tick()
        eye_update_gain(self.eye_onoff.on_data, self.eye_onoff.width, 3)
        eye_update_gain(self.eye_onoff.off_data, self.eye_onoff.width, 3)
        
        self.eye_border.tick()
        eye_update_gain(self.eye_border.data, self.eye_border.width, 3)

class EyeRay:
    def __init__(self, fov, w, world, actor, ego_angle, ego_height):
        assert w > 0
        assert 0 <= ego_angle <= 1
        assert isinstance(world, World)
        assert isinstance(actor, WorldActor)
        
        self.angle = ego_angle
        self.height = ego_height
        
        self.world = world
        self.actor = actor
        world_map = world.world_map
        self.world_map = world_map
        
        self.rects = []
        
        #self.x0 = self.border
        #self.y0 = self.border
        
        self.scene = self.build_scene()
        self.ray_t = RayTracing(fov, w, w)
        
        self.data = np.zeros(w * w, 'f')
        
    def build_scene(self):
        world_map = self.world.world_map
        width = world_map.width
        length = world_map.length
        
        scene = RayScene()
        
        scene.set_light0(0.7)
        scene.set_light1(0.3)
        
        scene.add_wall((0, 0, - length), (width, 1, - length))
        scene.add_wall((0, 0, 0), (0, 1, - length))
        scene.add_wall((width, 0, -length), (width, 1, 0))
        scene.add_wall((width, 0, 0), (0, 1, 0))
        
        for j in range(length):
            for i in range(width):
                if world_map[(i, j)]:
                    scene.add_box((i, 0, -j), (i + 1, 1, -j - 1))
        
        scene.add_ground((-0.1, 0, 0.1), (width + 0.1, 0, -length - 0.1), (0.5, 0.5, 0.5))
        
        scene.build()
        
        return scene
        
    def tick(self):
        actor = self.actor
        
        orig = (actor.x, self.height, -actor.y)
        mat = self.ray_t.rot_mat((actor.dir_ego + self.angle) % 1.0)
        
        self.ray_t.ray_trace_grey(orig, mat, self.scene, self.data)
    
class Eye6Blur:
    def __init__(self, sample):
        length = len(sample)
        
        width = (int) (math.sqrt(length))
        
        assert width * width == length 
        
        self.width = width
        self.w_blur = width // 2 - 1
        
        self.source = sample
        
        self.blur_size = self.w_blur * self.w_blur
        
        self.data = np.zeros(self.blur_size)


    def tick(self):
        s = self.source
        blur = self.data
        width = self.width
        w_blur = self.w_blur
        
        factor = 0.25
                
        for j in range(w_blur):
            s_y0 = 2 * j * width + width + j % 2
            y0 = j * w_blur
             
            for i in range(w_blur):
                s_x0 = s_y0 + 2 * i
                x0 = y0 + i
                
                v = s[s_x0] + s[s_x0 + 1] + s[s_x0 + width] + s[s_x0 + width + 1]
            
                blur[x0] = factor * v
    
class Eye6OnOff:
    def __init__(self, source):
        self.source = source
        
        length = len(source)
        
        w_source = (int) (math.sqrt(length))
        
        assert w_source * w_source == length 
        
        self.width = w_source
                
        self.on_data = np.zeros(self.width * self.width)
        self.off_data = np.zeros(self.width * self.width)

    def tick_compress(self):
        source = self.source
        
        on_data = self.on_data
        off_data = self.off_data
        
        w = self.width
        sw = w + 2

        for j in range(1, w - 1):
            off = j % 2
            s0 = (j + 1) * sw + 1 + off
            x0 = j * w
            
            for i in range(1, w - 1):
                s = s0 + i
                x = x0 + i
                
                sm1 = s - sw - off
                sp1 = s + sw - off
                
                v = source[s] - (1.0/6.0) * (source[sm1] + source[sm1 + 1]
                                             + source[s - 1] + source[s + 1]
                                             + source[sp1] + source[sp1 + 1])
                                             
                on_data[x] = max(0, v)
                off_data[x] = max(0, -v)
                

    def tick(self):
        source = self.source
        
        on_data = self.on_data
        off_data = self.off_data
        
        w = self.width
        sw = w

        for j in range(1, w - 1):
            off = j % 2
            s0 = j * w
            x0 = j * w
            
            for i in range(1, w - 1):
                s = s0 + i
                x = x0 + i
                
                sm1 = s - sw - off
                sp1 = s + sw - off
                
                v = source[s] - (1.0/6.0) * (source[sm1] + source[sm1 + 1]
                                             + source[s - 1] + source[s + 1]
                                             + source[sp1] + source[sp1 + 1])
                                             
                on_data[x] = max(0, v)
                off_data[x] = max(0, -v)
                
        self.tick_border()
        
    def tick_border(self):
        s = self.source
        
        on_data = self.on_data
        off_data = self.off_data
        
        w = self.width
        sw = w
        
        xn = w * (w - 1)

        for i in range(1, w - 1):
            v_top = s[i] - 0.25 * (s[i - 1] + s[i + 1] + s[i + sw - 1] + s[i + sw])
            on_data[i] =  max(0, v_top)
            off_data[i] =  max(0, -v_top)
            
            v_bot = s[xn + i] - 0.25 * (s[xn + i - 1] + s[xn + i + 1] 
                                        + s[xn - sw + i - 1] + s[xn - sw + i])
            on_data[xn + i] =  max(0, v_bot)
            off_data[xn + i] =  max(0, -v_bot)
            
            y0 = i * w
            v_l = s[y0] - (1/3) * (s[y0 + 1] + s[y0 - sw] + s[y0 + sw])
            on_data[y0] =  max(0, v_l)
            off_data[y0] =  max(0, -v_l)
            
            yn = y0 + w - 1
            v_r = s[yn] - (1/3) * (s[yn - 1] + s[yn - w] + s[yn + w - 1])
            on_data[yn] =  max(0, v_r)
            off_data[yn] =  max(0, -v_r)
            
        xt = xn + w - 1
        
        v_ul = s[0] - (1/2) * (s[1] + s[w])
        on_data[0] = max(0, v_ul)
        off_data[0] = max(0, -v_ul)
            
        x0t = w - 1
        v_ur = s[x0t] - (1/3) * (s[x0t - 1] + s[x0t + w - 1] + s[x0t + w])
        on_data[x0t] = max(0, v_ur)
        off_data[x0t] = max(0, -v_ur)
            
        v_ll = s[xn] - (1/3) * (s[xn - w] + s[xn - w + 1] + s[xn + 1])
        on_data[xn] = max(0, v_ll)
        off_data[xn] = max(0, -v_ll)
            
        v_lr = s[xt] - (1/2) * (s[xt - w] + s[xt - 1])
        on_data[xt] = max(0, v_lr)
        off_data[xt] = max(0, -v_lr)
    
class Eye6Border:
    def __init__(self, s_on, s_off):
        self.s_on = s_on
        self.s_off = s_off
        
        length = len(s_on)
        
        w_source = (int) (math.sqrt(length))
        
        assert w_source * w_source == len(s_on)
        assert w_source * w_source == len(s_off)
        
        self.width = w_source
                
        self.data = np.zeros(self.width * self.width)

    def tick(self):
        s_on = self.s_on
        s_off = self.s_off
        
        data = self.data

        for j in range(len(self.data)):
            data[j] = min(1, s_on[j] + s_off[j])

    def tick_old(self):
        s_on = self.s_on
        s_off = self.s_off
        
        data = self.data
        w = self.width

        for j in range(1, w):
            s0 = j * w
            x0 = j * w
            off = j % 2
            
            for i in range(w):
                s = s0 + i
                x = x0 + i
                
                if off == 0:
                    if off + i > 0:
                        v = (s_on[s] * 0.5 * (s_off[s - w - 1] + s_off[s - w])
                             + s_off[s] * 0.5 * (s_on[s - w - 1] + s_on[s - w]))
                    else:
                        v = s_on[s] * s_off[s - w] + s_off[s] * s_on[s - w]
                else:
                    if off + i < w:
                        v = (s_on[s] * 0.5 * (s_off[s - w] + s_off[s - w + 1])
                             + s_off[s] * 0.5 * (s_on[s - w] + s_on[s - w + 1]))
                    else:
                        v = s_on[s] * s_off[s - w] + s_off[s] * s_on[s - w]
                                             
                data[x] = v
                
def eye_update_gain(data, w, n):
    assert n > 1
    dw = w // n
    
    for j in range(n):
        y0 = j * dw
        
        dy = dw if j < n - 1 else w - y0
        
        for i in range(n):
            x0 = i * dw
            
            dx = dw if i < n - 1 else w - x0
            
            factor = max_area(data, x0, y0, w, dx, dy)
            
            if factor < 1e-4:
                # skip
                factor = 0
            elif factor < 1e-2:
                update_area(data, x0, y0, w, dx, dy, 0.1/factor)
            elif factor < 1e-1:
                update_area(data, x0, y0, w, dx, dy, 0.5/factor)
            else:
                update_area(data, x0, y0, w, dx, dy, 1.0/factor)
                
def max_area(data, x0, y0, w, dx, dy):
    v = 0
    for j in range(dy):
        y = y0 + j
        x1 = y * w + x0
        for i in range(dx):
            v = max(v, data[x1 + i])
            
    return v
                
def update_area(data, x0, y0, w, dx, dy, factor):
    for j in range(dy):
        y = y0 + j
        x1 = y * w + x0
        for i in range(dx):
            data[x1 + i] *= factor
            assert 0 <= data[x1 + i] and data[x1 + i] <= 1

