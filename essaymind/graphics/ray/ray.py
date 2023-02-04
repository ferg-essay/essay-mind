import numpy as np
import math
import logging
#import numba
#from numba import jit, njit

log = logging.getLogger("Ray")

k_epsilon = 1e-5
d_max = 1e10

class RayTracing(object):
    def __init__(self, angle, w, h):
        assert w > 1 # and w % 2 == 1
        assert h > 1 # and h % 2 == 1
        self.angle = angle
        self.w = w
        self.h = h
        self.triangles = []
        
        #self.init_rays_angles(angle, w, h)
        self.init_rays_rect(angle, w, h)
                
        # self.rays = self.init_rays(angle, w, h)
        self.rays = self.flatten_rays()
        
        self.sky_grey = 0.9
        
    def init_rays_rect(self, angle, w, h):
        theta = np.radians(angle)
        
        w2 = np.sin(theta / 2)
        h2 = np.sin(theta / 2)
        z = np.cos(theta / 2)
        
        dw = 2 * w2 / (w - 1)
        dh = 2 * h2 / (h - 1)
        
        x0 = -w2
        y0 = -h2
        
        self.rays = []
        for j in range(self.h):
            row = []
            self.rays.append(row)
            y = y0 + (h - j - 1) * dh
            for i in range(self.w):
                x = x0 + i * dw 
                
                row.append(normalize([x, y, -z]))
        # self.rays = self.init_rays(angle, w, h)
        
    def init_rays_angles(self, angle, w, h):
        rays = np.zeros((h, w, 3), 'f')
        
        theta = np.radians(angle)
         
        w0 = - theta / 2
        h0 = w0
        
        dw = theta / (w - 1)
        dh = theta / (h - 1)
        
        self.rays = []
        for j in range(self.h):
            row = []
            self.rays.append(row)
            for i in range(self.w):
                row.append(self.init_ray(w0 + i * dw, h0 + j * dh))
        
        return rays

    def init_ray(self, theta_w, theta_h):
        c_w, s_w = np.cos(theta_w), np.sin(theta_w)
        c_h, s_h = np.cos(theta_h), -np.sin(theta_h)
        
        #mat = np.zeros((3, 3))
        
        '''
        mat[0][0] = c_w
        mat[0][1] = 0
        mat[0][2] = - s_w
        
        mat[1][0] = -s_h * s_w
        mat[1][1] = c_h
        mat[1][2] = -s_h * c_w
        
        mat[2][0] = c_h * s_w
        mat[2][1] = s_h
        mat[2][2] = c_h * c_w
        #mat[3][3] = 1
        '''
        
        #return mat 
        
        #ray[0] = s_w
        #ray[1] = 0
        #ray[2] = -c_w
        
        #ray[0] = 0
        #ray[1] = s_h
        #ray[2] = -c_h
        
        #ray[0] = s_w
        #ray[1] = s_h * c_w
        #ray[2] = - c_h * c_w
        
        return np.array([s_w, s_h * c_w, - c_h * c_w]) 
        
    def rot_mat(self, angle_y):
        assert 0 <= angle_y and angle_y <= 1
        mat = np.zeros((3, 3))
        theta = angle_y * 2 * np.pi
        c, s = np.cos(theta), np.sin(theta)
        
        mat[0][0] = c
        mat[0][2] = -s
        mat[1][1] = 1
        mat[2][0] = s
        mat[2][2] = c
        
        return mat
    
    def ray_trace_depth(self, orig, mat, scene):
        rays = self.fill_rays(mat)
        triangles = scene.get_triangles()
        #depth_all = np.zeros(len(rays) * len(triangles), 'f')
        index_rays = np.zeros(len(rays), 'i')
        depth_rays = np.zeros(len(rays), 'f')
        
        self.ray_trace_impl(self, orig, mat, triangles, index_rays, depth_rays)
        '''
        for j in range(len(rays)):
            best_dist = 1e6
            best_index = -1
            ray = rays[j]
            #log.info(f"{ray}")
            
            for i in range(len(triangles)):
                tri = triangles[i]
                dist = ray_intersect(orig, ray, tri.v0, tri.v1, tri.v2)
                #log.info(f"{dist} {tri}")
                if dist < best_dist:
                    best_dist = dist
                    best_index = i
            
            index_rays[j] = best_index
            depth_rays[j] = best_dist
        '''    
        return (depth_rays, index_rays)
    
    def ray_trace_grey(self, orig, mat, scene, greys):
        #rays = self.fill_rays(mat)
        rays = self.rays
        triangles = scene.get_triangles()
        triangle_array = scene.get_triangle_array()
        
        #depth_all = np.zeros(len(rays) * len(triangles), 'f')
        index_rays = np.zeros(len(rays), 'i')
        depth_rays = np.zeros(len(rays), 'f')
        
        orig = np.asarray(orig, 'f')
        mat = np.asarray(mat, 'f')
        
        #self.ray_trace_impl(orig, rays, triangles, index_rays, depth_rays)
        ray_trace_impl(orig, mat, rays, triangle_array, index_rays, depth_rays)
        
        #greys = np.zeros(len(rays), 'f')
        
        for i in range(len(rays)):
            depth = depth_rays[i]
            index = index_rays[i]
            
            if index >= 0:
                greys[i] = triangles[index].grey
            else:
                greys[i] = self.sky_grey
        
        return greys
            
        
    def fill_rays(self, mat):
        rays = []
        for j in range(self.h):
            for i in range(self.w):
                rays.append(self.get_ray(mat, i, j))
                
        return rays
    
    def flatten_rays(self):
        rays = []
        for j in range(self.h):
            for i in range(self.w):
                rays.append(np.asarray(self.rays[j][i], 'f'))
                
        return rays
    
    def get_ray_2(self, ray_0, i, j):
        return np.matmul(self.rays[j][i], ray_0)
    
    def get_ray(self, mat, i, j):
        return np.asarray(np.matmul(mat, self.rays[j][i]), 'f')

    def add_tri(self, v0, v1, v2, color=(1,1,1)):
        self.triangles.append(Triangle(v0, v1, v2))
        
    def __str__(self):
        return f"RayTracing[{self.angle},{self.w},{self.h}]"

box_vertices = (
    (0, 0, 0),
    (0, 1, 0),
    (1, 1, 0),
    (1, 0, 0),
    
    (0, 0, 1),
    (0, 1, 1),
    (1, 1, 1),
    (1, 0, 1),
    )

box_surfaces = (
    (2, 3, 7, 6), # right
    (0, 1, 5, 4), # left
    (1, 2, 6, 5), # top
    (0, 4, 7, 3), # bottom
    (0, 3, 2, 1), # back
    (4, 5, 6, 7), # front
    )
    
class RayScene:
    def __init__(self):
        self.triangles = []
        self.light0_loc = normalize((-1, 3, 1))
        self.light0 = 0
        self.light1_loc = normalize((1, 3, -1))
        self.light1 = 0
        
    def build(self):
        self.build_triangle_array()
        
    def build_triangle_array(self):
        triangles = self.triangles
        
        triangle_array = np.zeros((len(triangles), 3, 3), 'f')
        
        for i in range(len(triangles)):
            triangle = triangles[i]
            
            triangle_array[i][0] = triangle.v0
            triangle_array[i][1] = triangle.v1
            triangle_array[i][2] = triangle.v2
            
        self.triangle_array = triangle_array
        
    def set_light0(self, value):
        assert 0 <= value and value <= 1
        
        self.light0 = value
        
    def set_light1(self, value):
        assert 0 <= value and value <= 1
        
        self.light1 = value
        
    def add_box(self, v0, v1, color=(1,1,1), is_outside=True):
        v = [
            np.array([v0[0], v0[1], v0[2]], 'f'),
            np.array([v0[0], v1[1], v0[2]], 'f'),
            np.array([v1[0], v1[1], v0[2]], 'f'),
            np.array([v1[0], v0[1], v0[2]], 'f'),
            
            np.array([v0[0], v0[1], v1[2]], 'f'),
            np.array([v0[0], v1[1], v1[2]], 'f'),
            np.array([v1[0], v1[1], v1[2]], 'f'),
            np.array([v1[0], v0[1], v1[2]], 'f'),
            ]

        for s in box_surfaces:
            if is_outside:
                self.add_square(v[s[0]], v[s[1]], v[s[2]], v[s[3]], color)
            else:
                self.add_square(v[s[0]], v[s[3]], v[s[2]], v[s[1]], color)
            
    def add_ground(self, v0_s, v1_s, color=(1, 1, 1)):
        v0 = v0_s
        v1 = (v1_s[0], v0_s[1], v0_s[2])
        v2 = (v1_s[0], v0_s[1], v1_s[2])
        v3 = (v0_s[0], v0_s[1], v1_s[2])
        
        self.add_triangle(v0, v1, v2, color)
        self.add_triangle(v2, v3, v0, color)
            
    def add_wall(self, v0_s, v1_s, color=(1, 1, 1)):
        v0 = v0_s
        v1 = (v1_s[0], v0_s[1], v1_s[2])
        v2 = (v1_s[0], v1_s[1], v1_s[2])
        v3 = (v0_s[0], v1_s[1], v0_s[2])
        
        self.add_triangle(v0, v1, v2, color)
        self.add_triangle(v2, v3, v0, color)
            
    def add_square(self, v0, v1, v2, v3, color):
        self.add_triangle(v0, v1, v2, color)
        self.add_triangle(v2, v3, v0, color)
        
    def add_triangle(self, v0, v1, v2, color):
        light = self.calculate_light(v0, v1, v2)
        color = (color[0] * light, color[1] * light, color[2] * light)
        self.triangles.append(Triangle(v0, v1, v2, color))
        
    def calculate_light(self, v0, v1, v2):
        v0v1 = np.subtract(v1, v0)
        v0v2 = np.subtract(v2, v0)
        
        norm = normalize(np.cross(v0v1, v0v2))

        light0_dot = max(0, np.dot(norm, self.light0_loc))
        light1_dot = max(0, np.dot(norm, self.light1_loc))
        
        light0 = self.light0
        light1 = self.light1
        
        ambient = min(1 - light0, 1 - light1)
        
        assert ambient >= 0
        
        return light0_dot * light0 + light1_dot * light1 + ambient
        
    def get_triangles(self):
        return self.triangles
        
    def get_triangle_array(self):
        return self.triangle_array
    
def normalize(v):
    v = np.asarray(v, 'f')
    d = np.sqrt(np.dot(v, v))
        
    vn = v * (1/d)
        
    return vn
        
class Triangle:
    def __init__(self, v0, v1, v2, color):
        self.v0 = v0
        self.v1 = v1
        self.v2 = v2
        self.color = color
        self.grey = math.sqrt((color[0] * color[0] + color[1] * color[1] + color[2] * color[2]) / 3)
        
    def __str__(self):
        return f"Tri{self.v0}{self.v1}{self.v2}"
        
class Box:
    def __init(self, v0, v1, color):
        self.v0 = v0
        self.v1 = v1
        self.color = color
        
#@jit        
def ray_trace_impl(orig, mat, rays, triangle_array, index_rays, depth_rays):
    ray = np.zeros(3, 'f')
    
    for j in range(len(rays)):
        best_dist = 1e6
        best_index = -1
        np.matmul(mat, rays[j], ray)
            
        for i in range(len(triangle_array)):
            tri = triangle_array[i]
            dist = ray_intersect(orig, ray, tri[0], tri[1], tri[2])
                #log.info(f"{dist} {tri}")
            if dist < best_dist:
                best_dist = dist
                best_index = i
            
            index_rays[j] = best_index
            depth_rays[j] = best_dist
        
#@njit        
def ray_intersect(orig, ray, v0, v1, v2):
    v0v1 = np.subtract(v1, v0)
    v0v2 = np.subtract(v2, v0)
    pvec = np.cross(ray, v0v2)
    det = np.dot(v0v1, pvec)
    #log.info(f"det {det} v0v1={v0v1} v0v2={v0v2} pvec={pvec}")
    if det < k_epsilon:
        return d_max
    if abs(det) < k_epsilon:
        return d_max
        
    inv_det = 1 / det
        
    tvec = np.subtract(orig, v0)
    u = np.dot(tvec, pvec) * inv_det
    if u < 0 or u > 1:
        return d_max
        
    qvec = np.cross(tvec, v0v1)
    v = np.dot(ray, qvec) * inv_det
        
    if v < 0 or u + v > 1:
        return d_max
        
    t = np.dot(v0v2, qvec) * inv_det
        
    return t if t >= 0 else d_max
        
class Triangle_Old:
    def __init__(self, v0, v1, v2):
        self.bounds = np.array(9, v0[0], v0[1], v0[2],
                                  v1[0], v1[1], v1[2],                                  
                                  v2[0], v2[1], v2[2])