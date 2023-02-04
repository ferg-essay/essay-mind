import unittest

import numpy as np
import graphics.ray as ray
from graphics.ray import RayTracing, RayScene

import logging

logging.basicConfig(level=logging.DEBUG,
                    format='%(levelname)-5s:%(name)-10s: %(message)s')

log = logging.getLogger("Test")

class Test(unittest.TestCase):

    def xtest_intersect_simple_triangle(self):
        ray_t = RayTracing(90, 3, 3)
        log.info(ray_t)
        
        orig = [0, 0, 0]
        ray =  [0, 0, -1]
        
        # basic triangle
        tri_0 = [[-1, 0, -1], [ 1, 0, -1], [ 0, 1, -1]]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) == 1.0
        # reversed triangle (face culling)
        assert ray_intersect(orig, ray, tri_0[0], tri_0[2], tri_0[1]) > 1e6
        
        # triangle far to right
        tri_2 = [[10, 0, -1], [12, 0, -1], [11, 1, -1]]
        assert ray_intersect(orig, ray, tri_2[0], tri_2[1], tri_2[2]) > 1e6
        
        # small triangle
        tri_3 = [[-0.01, -0.01, -0.01], [ 0.01, -0.01, -0.01], [ 0, 0.01, -0.01]]
        assert ray_intersect(orig, ray, tri_3[0], tri_3[1], tri_3[2]) == 0.01
        
        # triangle slightly above
        tri_4 = [[-1, 0.01, -1], [ 1, 0.01, -1], [ 0, 3, -1]]
        assert ray_intersect(orig, ray, tri_4[0], tri_4[1], tri_4[2]) > 1e6
        
        # triangle behind
        tri_5 = [[-1, 0, 1], [ 1, 0, 1], [ 0, 1, 1]]
        assert ray_intersect(orig, ray, tri_5[0], tri_5[1], tri_5[2]) > 1e6

    def xtest_intersect_ray_orig(self):
        orig = [0, 0, 0]
        ray =  [0, 0, -1]
        
        # basic triangle
        tri_0 = [[-1, -1, -1], [ 1, -1, -1], [ 0, 1, -1]]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) == 1.0
        
        # low origin
        orig = [0, -1, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) == 1.0
        
        orig= [0, -1.01, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) > 1e6
        
        orig = [-1, -1, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) == 1.0
        
        orig = [-1.01, -1, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) > 1e6
        
        orig = [1, -1, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) == 1.0
        
        orig = [1.01, -1, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) > 1e6
        
        orig = [0, 1, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) == 1.0
        
        orig = [0, 1.01, 0]
        assert ray_intersect(orig, ray, tri_0[0], tri_0[1], tri_0[2]) > 1e6

    def xtest_ray_90_2(self):
        ray_t = RayTracing(90, 2, 2)
        log.info(ray_t)
        
        rot = 0
        mat = ray_t.rot_mat(rot)
        log.info(mat)
        
        assert_vector(ray_t.get_ray(mat, 0, 0), [-0.7071, 0.5,   -0.5])
        assert_vector(ray_t.get_ray(mat, 1, 0), [ 0.7071, 0.5,   -0.5])
        assert_vector(ray_t.get_ray(mat, 0, 1), [-0.7071,-0.5,   -0.5])
        assert_vector(ray_t.get_ray(mat, 1, 1), [ 0.7071,-0.5,   -0.5])

    def xtest_ray_z_90_3(self):
        ray_t = RayTracing(90, 3, 3)
        log.info(ray_t)
        
        rot = 0
        mat = ray_t.rot_mat(rot)
         
        # center
        assert_vector(ray_t.get_ray(mat, 1, 1), [ 0.,     0.0,   -1.])
        # x/y axis
        assert_vector(ray_t.get_ray(mat, 0, 1), [-0.7071, 0.0,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 2, 1), [ 0.7071, 0.0,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 0), [ 0.,     0.7071,-0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 2), [ 0.,    -0.7071,-0.7071])

        # all rows
        assert_vector(ray_t.get_ray(mat, 0, 0), [-0.7071, 0.5,   -0.5])
        assert_vector(ray_t.get_ray(mat, 1, 0), [ 0.0,    0.7071,-0.7071])
        assert_vector(ray_t.get_ray(mat, 2, 0), [ 0.7071, 0.5,   -0.5])
        
        assert_vector(ray_t.get_ray(mat, 0, 1), [-0.7071, 0.,    -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 1), [ 0.,     0.,    -1.])
        assert_vector(ray_t.get_ray(mat, 2, 1), [ 0.7071, 0.,    -0.7071])
        
        assert_vector(ray_t.get_ray(mat, 0, 2), [-0.7071,-0.5,   -0.5])
        assert_vector(ray_t.get_ray(mat, 1, 2), [ 0.,    -0.7071,-0.7071])
        assert_vector(ray_t.get_ray(mat, 2, 2), [ 0.7071,-0.5,   -0.5])

    def xtest_ray_x_90(self):
        ray_t = RayTracing(90, 3, 3)
        log.info(ray_t)
        
        rot = 0.25
        mat = ray_t.rot_mat(rot) 
        log.info(mat)
        
        assert_vector(ray_t.get_ray(mat, 1, 1), [ 1.,     0.0,    0.])
        
        assert_vector(ray_t.get_ray(mat, 0, 0), [ 0.5,    0.5,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 0), [ 0.7071, 0.7071, 0.])
        assert_vector(ray_t.get_ray(mat, 2, 0), [ 0.5,    0.5,    0.7071])
        
        assert_vector(ray_t.get_ray(mat, 0, 1), [ 0.7071,  0.,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 1), [ 1.,     0.0,    0.])
        assert_vector(ray_t.get_ray(mat, 2, 1), [ 0.7071, 0.,     0.7071])
        
        assert_vector(ray_t.get_ray(mat, 0, 2), [ 0.5,   -0.5,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 2), [ 0.7071,-0.7071, 0.])
        assert_vector(ray_t.get_ray(mat, 2, 2), [ 0.5,   -0.5,    0.7071])

    def xtest_ray_90_4(self):
        ray_t = RayTracing(90, 4, 4)
        log.info(ray_t)
        
        rot = 0
        mat = ray_t.rot_mat(rot) 
        log.info(mat)
        
        # center
        assert_vector(ray_t.get_ray(mat, 1, 1), [-0.5, 0.433,  -0.75])
        assert_vector(ray_t.get_ray(mat, 2, 1), [ 0.,  0.5,    -0.866])
        assert_vector(ray_t.get_ray(mat, 1, 2), [-0.5, 0.,     -0.866])
        assert_vector(ray_t.get_ray(mat, 2, 2), [ 1.,     0.0,    0.])
        
        assert_vector(ray_t.get_ray(mat, 0, 0), [ 0.5,    0.5,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 0), [ 0.7071, 0.7071, 0.])
        assert_vector(ray_t.get_ray(mat, 2, 0), [ 0.5,    0.5,    0.7071])
        
        assert_vector(ray_t.get_ray(mat, 0, 1), [ 0.7071,  0.,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 1), [ 1.,     0.0,    0.])
        assert_vector(ray_t.get_ray(mat, 2, 1), [ 0.7071, 0.,     0.7071])
        
        assert_vector(ray_t.get_ray(mat, 0, 2), [ 0.5,   -0.5,   -0.7071])
        assert_vector(ray_t.get_ray(mat, 1, 2), [ 0.7071,-0.7071, 0.])
        assert_vector(ray_t.get_ray(mat, 2, 2), [ 0.5,   -0.5,    0.7071])

    def xtest_ray_z_120(self):
        ray_t = RayTracing(120, 5, 5)
        
        log.info(ray_t)
        rot = 0.
        mat = ray_t.rot_mat(rot) 
        
        assert_vector(ray_t.get_ray(mat, 2, 2), [ 0.0,    0.0,   -1.0])
        
        assert_vector(ray_t.get_ray(mat, 0, 0), [-0.866,  0.433, -0.25])
        assert_vector(ray_t.get_ray(mat, 1, 0), [-0.5,    0.75,  -0.433])
        assert_vector(ray_t.get_ray(mat, 2, 0), [ 0.0,    0.866, -0.5])
        assert_vector(ray_t.get_ray(mat, 3, 0), [ 0.5,    0.75,  -0.433])
        assert_vector(ray_t.get_ray(mat, 4, 0), [ 0.866,  0.433, -0.25])
        
        assert_vector(ray_t.get_ray(mat, 0, 1), [-0.866,  0.25,  -0.433])
        assert_vector(ray_t.get_ray(mat, 1, 1), [-0.5,    0.433, -0.75])
        assert_vector(ray_t.get_ray(mat, 2, 1), [ 0.0,    0.5,   -0.866])
        assert_vector(ray_t.get_ray(mat, 3, 1), [ 0.5,    0.433, -0.75])
        assert_vector(ray_t.get_ray(mat, 4, 1), [ 0.866,  0.25,  -0.433])
        
        assert_vector(ray_t.get_ray(mat, 0, 2), [-0.866,  0.0,   -0.5])
        assert_vector(ray_t.get_ray(mat, 1, 2), [-0.5,    0.0,   -0.866])
        assert_vector(ray_t.get_ray(mat, 2, 2), [ 0.0,    0.0,   -1.0])
        assert_vector(ray_t.get_ray(mat, 3, 2), [ 0.5,    0.0,   -0.866])
        assert_vector(ray_t.get_ray(mat, 4, 2), [ 0.866,  0.0,   -0.5])
        
        assert_vector(ray_t.get_ray(mat, 0, 3), [-0.866, -0.25,  -0.433])
        assert_vector(ray_t.get_ray(mat, 1, 3), [-0.5,   -0.433, -0.75])
        assert_vector(ray_t.get_ray(mat, 2, 3), [ 0.0,   -0.5,   -0.866])
        assert_vector(ray_t.get_ray(mat, 3, 3), [ 0.5,   -0.433, -0.75])
        assert_vector(ray_t.get_ray(mat, 4, 3), [ 0.866, -0.25,  -0.433])
        
        assert_vector(ray_t.get_ray(mat, 0, 4), [-0.866, -0.433, -0.25])
        assert_vector(ray_t.get_ray(mat, 1, 4), [-0.5,   -0.75,  -0.433])
        assert_vector(ray_t.get_ray(mat, 2, 4), [ 0.0,   -0.866, -0.5])
        assert_vector(ray_t.get_ray(mat, 3, 4), [ 0.5,   -0.75,  -0.433])
        assert_vector(ray_t.get_ray(mat, 4, 4), [ 0.866, -0.433, -0.25])

    def xtest_ray_90_depth_index(self):
        ray_t = RayTracing(90, 3, 3)
        log.info(ray_t)
        orig = [0, 0, 0]
        
        scene = RayScene()
        scene.add_box((-0.1, -0.1, -0.25), (0.1, 0.25, -0.5))
        
        mat = ray_t.rot_mat(0) 
        
        depth, index = ray_t.ray_trace_depth(orig, mat, scene)
                
        log.info(depth)
        log.info(index)
        
        assert_arrays(depth, [1e6, 0.3536, 1e6, 1e6, 0.25, 1e6, 1e6, 1e6, 1e6])
        assert_depth(depth, [0, 1, 0, 0, 1, 0, 0, 0, 0])
        
        scene = RayScene()
        scene.add_box((0.01, -0.1, -1), (1, 0.25, -1.1))
        
        depth, index = ray_t.ray_trace_depth(orig, mat, scene)
        assert_arrays(depth, [1e6, 1e6, 1e6, 1e6, 1e6, 1.414, 1e6, 1e6, 1e6])
        log.info(depth)

    def xtest_ray_90_ground(self):
        ray_t = RayTracing(90, 6, 6)
        log.info(ray_t)
        
        scene = RayScene()
        scene.add_ground((-1, 0, 1), (10, 0, -100))
        
        mat = ray_t.rot_mat(0) 
        
        greys = ray_t.ray_trace_grey([0, 0.1, 1], mat, scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 0, 0, 0, 0, 0, 
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0,
                            1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1])

    def test_ray_90_grey(self):
        ray_t = RayTracing(90, 6, 6)
        log.info(ray_t)
        
        scene = RayScene()
        scene.add_box((-1, 0, 0), (1, 1, -10))
        
        mat = ray_t.rot_mat(0) 
        
        greys = ray_t.ray_trace_grey([0, 0.1, 1], mat, scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 0, 0, 0, 0, 0, 
                            0, 1, 1, 1, 1, 0,
                            0, 1, 1, 1, 1, 0,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0])
        
        greys = ray_t.ray_trace_grey([-1, 0.1, 1], ray_t.rot_mat(0), scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 0, 0, 0, 0, 0, 
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0])
        
        greys = ray_t.ray_trace_grey([-1, 0.1, 0.4], ray_t.rot_mat(0), scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 0, 0, 1, 1, 1, 
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0])
        
        greys = ray_t.ray_trace_grey([-1, 0.1, 0.2], ray_t.rot_mat(0), scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 0, 0, 1, 1, 1, 
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0])
        
        greys = ray_t.ray_trace_grey([-1, 0.1, 0.1], ray_t.rot_mat(0), scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 0, 0, 1, 1, 1, 
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1,
                            0, 0, 0, 1, 1, 1])
        
        greys = ray_t.ray_trace_grey([-1, 0.1, 0.3], ray_t.rot_mat(0.125), scene)
        log.info(greys.reshape((6, 6)))
        assert_grey(greys, [0, 1, 1, 1, 0, 0, 
                            0, 1, 1, 1, 1, 0,
                            0, 1, 1, 1, 1, 0,
                            0, 1, 1, 1, 0, 0,
                            0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0])
       
def assert_vector(v1, v2):
    log.debug(f"{v1} {v2}")
    
    assert (abs(v1[0] - v2[0]) < 1e-4
            and abs(v1[1] - v2[1]) < 1e-4
            and abs(v1[2] - v2[2]) < 1e-4)
    
def assert_arrays(v1, v2):
    assert len(v1) == len(v2)
    
    for i in range(len(v1)):
        assert abs(v1[i] - v2[i]) < 1e-3
    
def assert_grey(v1, v2):
    assert len(v1) == len(v2)
    
    for i in range(len(v1)):
        if abs(v1[i] - v2[i]) >= 1e-3:
            log.info(f"{i}: {v1[i]:.3f} {v2[i]:.3f}")
            
        assert abs(v1[i] - v2[i]) < 1e-3
    
def assert_depth(v1, v2):
    assert len(v1) == len(v2)
    
    for i in range(len(v1)):
        assert (v1[i] < 1e6) == (v2[i] == 1)
    
def ray_intersect(orig_p, ray_p, v0_p, v1_p, v2_p):
    orig = np.array(orig_p, 'f')
    ray_dir = np.array(ray_p, 'f')
    v0 = np.array(v0_p, 'f')
    v1 = np.array(v1_p, 'f')
    v2 = np.array(v2_p, 'f')
        
    t = ray.ray_intersect(orig, ray_dir, v0, v1, v2)
    t = round(t, 4)
    log.info(f"{t:.3g} <= ray={ray_dir} v0={v0} v1={v1} v2={v2}")
        
    return t

if __name__ == "__main__":
    #import sys;sys.argv = ['', 'Test.testName']
    unittest.main()