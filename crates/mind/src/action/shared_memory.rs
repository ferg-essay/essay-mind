use std::sync::{Arc, RwLock, RwLockReadGuard, PoisonError, RwLockWriteGuard};
use core::fmt::Debug;

pub struct SharedMemory<T,const N:usize>([Arc<RwLock<T>>; N]);

impl<T:Default + Debug,const N:usize> SharedMemory<T,N> {
    pub fn new()-> Self {
        /*
        let data: [Arc<RwLock<T>>; N] = data
            .drain(..)
            .map(|item| Arc::new(RwLock::new(item)))
            .collect::<Vec<Arc<RwLock<T>>>>()
            .try_into()
            .unwrap();
         */
        let data: [Arc<RwLock<T>>; N] = (0..N)
            .map(|_| Arc::new(RwLock::new(Default::default())))
            .collect::<Vec<Arc<RwLock<T>>>>()
            .try_into()
            .unwrap();

        Self(data)
    }

    pub fn write(&self, tick: usize) -> 
        Result<RwLockWriteGuard<T>,PoisonError<RwLockWriteGuard<T>>>
    {
        self.0[tick % N].write()
    }

    pub fn read(&self, tick: usize) -> 
        Result<RwLockReadGuard<T>,PoisonError<RwLockReadGuard<T>>>
    {
        self.0[(tick + 1) % N].read()
    }
}

impl<T,const N:usize> Clone for SharedMemory<T,N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

