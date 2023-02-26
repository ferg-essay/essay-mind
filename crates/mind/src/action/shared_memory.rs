use std::sync::{Arc, RwLock, RwLockReadGuard, PoisonError, RwLockWriteGuard};
use core::fmt::Debug;

pub struct SharedWriter<T,const N:usize=2>([Arc<RwLock<T>>; N]);
pub struct SharedReader<T,const N:usize=2>([Arc<RwLock<T>>; N]);

impl<T:Default + Debug,const N:usize> SharedWriter<T,N> {
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

    pub fn write(&self, ticks: u64) -> 
        Result<RwLockWriteGuard<T>,PoisonError<RwLockWriteGuard<T>>>
    {
        self.0[(ticks % N as u64) as usize].write()
    }

    pub fn reader(&self) -> SharedReader<T,N> {
        SharedReader(self.0.clone())
    }
    /*
    pub fn read(&self, tick: usize) -> 
        Result<RwLockReadGuard<T>,PoisonError<RwLockReadGuard<T>>>
    {
        self.0[(tick + 1) % N].read()
    }
     */
}

impl<T,const N:usize> Clone for SharedWriter<T,N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T,const N:usize> SharedReader<T,N> {
    pub fn read(&self, ticks: u64) -> 
        Result<RwLockReadGuard<T>,PoisonError<RwLockReadGuard<T>>>
    {
        self.0[((ticks + 1) % N as u64) as usize].read()
    }
}

impl<T,const N:usize> Clone for SharedReader<T,N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}


