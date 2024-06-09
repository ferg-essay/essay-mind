use std::{cell::RefCell, f64::consts::TAU, rc::Rc, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}};

use once_cell::sync::Lazy;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;


#[inline]
pub fn random() -> f32 {
    random_uniform()
}

#[inline]
pub fn random_uniform() -> f32 {
    next_u32() as f32 / u32::MAX as f32
}

#[inline]
pub fn random_normal() -> f32 {
    // let mut rng = rand::thread_rng();

    let rng_a = next_u64() as f64 / u64::MAX as f64;
    let rng_b = next_u64() as f64 / u64::MAX as f64;

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

pub fn random_test() {
    IS_TEST.store(true, Ordering::Relaxed);
}

fn next_u64() -> u64 {
    LOCAL_RNG.with(|x| { x.borrow_mut().next_u64() })
}

#[inline]
fn next_u32() -> u32 {
    LOCAL_RNG.with(|x| { x.borrow_mut().next_u32() })
}

struct TestRandom {
    rng: Arc<Mutex<ChaChaRng>>,
}

impl TestRandom {
    fn new() -> Self {
        let rng = ChaChaRng::seed_from_u64(42);
        // let rng = ChaChaRng::from_seed(seed); // StdRng::seed_from_u64(42);


        TestRandom {
            rng: Arc::new(Mutex::new(rng)),
        }
    }
}

impl RngCore for TestRandom {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.rng.lock().unwrap().next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.rng.lock().unwrap().next_u64()
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.lock().unwrap().fill_bytes(dest);
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.lock().unwrap().try_fill_bytes(dest)
    }
}

impl Clone for TestRandom {
    fn clone(&self) -> Self {
        Self { rng: self.rng.clone() }
    }
}

static TEST_RNG: Lazy<TestRandom> = Lazy::new(|| TestRandom::new());
thread_local! {
    static LOCAL_RNG: Rc<RefCell<Box<dyn RngCore>>> = {
        let rng : Box<dyn RngCore> = if IS_TEST.load(Ordering::Relaxed) {
            Box::new(rand::thread_rng())
        } else {
            Box::new(TEST_RNG.clone())
        };

        Rc::new(RefCell::new(rng))
    }
}
static IS_TEST: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
mod test {
    use crate::random::{random_normal, random_uniform};

    #[test]
    fn test_uniform() {
        assert_eq!(0.8426099, random_uniform());
        assert_eq!(0.5140493, random_uniform());
        assert_eq!(0.63707566, random_uniform());
        assert_eq!(0.4101988, random_uniform());
        assert_eq!(0.001730432, random_uniform());

        assert_eq!(0.09781387, random_uniform());
        assert_eq!(0.78603035, random_uniform());
        assert_eq!(0.16741663, random_uniform());
        assert_eq!(0.66239846, random_uniform());
        assert_eq!(0.16315256, random_uniform());
    }

    #[test]
    fn test_normal() {
        assert_eq!(-0.9748171, random_normal());
        assert_eq!(1.0693096, random_normal());
        assert_eq!(-1.0732915, random_normal());
        assert_eq!(-0.5822864, random_normal());
        assert_eq!(-1.652344, random_normal());

        assert_eq!(-0.39343083, random_normal());
        assert_eq!(-0.25734413, random_normal());
        assert_eq!(0.727897, random_normal());
        assert_eq!(-1.0519918, random_normal());
        assert_eq!(-1.442093, random_normal());
    }
}