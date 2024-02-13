use std::f64::consts::TAU;

use essay_tensor::Tensor;
use rand::RngCore;


pub fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

fn _random_uniform_tensor() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

#[inline]
pub fn random_uniform() -> f32 {
    let mut rng = rand::thread_rng();

    (rng.next_u64() as f64 / u64::MAX as f64) as f32
}

fn _random_normal_tensor() -> f32 {
    Tensor::random_normal([1], ())[0]
}

#[inline]
pub fn random_normal() -> f32 {
    let mut rng = rand::thread_rng();

    let rng_a = rng.next_u64() as f64 / u64::MAX as f64;
    let rng_b = rng.next_u64() as f64 / u64::MAX as f64;

    // TODO: save 2nd random variable to reduce next_u64 call

    // Box-Muller
    ((-2. * rng_a.ln()).sqrt() * (TAU * rng_b).cos()) as f32
}

pub fn random_pareto(low: f32, high: f32, alpha: f32) -> f32 {
    let x = random();

    let h_a = high.powf(alpha);
    let l_a = low.powf(alpha);

    (- (x * h_a - x * l_a - h_a) / (h_a * l_a)).powf(- 1. / alpha)
}
