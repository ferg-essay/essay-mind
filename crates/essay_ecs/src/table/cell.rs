use std::mem::{self, ManuallyDrop};
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use std::{marker::PhantomData, cmp};
use std::alloc::Layout;

use super::meta::{ColumnTypeId, ColumnType, RowMetas};

pub(crate) struct UnsafeCell<'c, T> {
    data: NonNull<u8>,

    marker: PhantomData<&'c T>,
}

impl<'c, T:'static> UnsafeCell<'c, T> {
    pub(crate) fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        let data = unsafe { std::alloc::alloc(layout) };
        let data = NonNull::new(data).unwrap();

        let mut cell = Self {
            data: data,

            marker: Default::default(),
        };

        unsafe {
            cell.write(value);
        }

        cell
    }
    
    pub fn deref(&self) -> &'c T {
        unsafe {
            &*self.data.as_ptr().cast::<T>()
        }
    }
    
    pub fn deref_mut(&self) -> &'c mut T {
        unsafe {
            &mut *self.data.as_ptr().cast::<T>()
        }
    }

    unsafe fn write(&mut self, value: T) {
        let mut value = ManuallyDrop::new(value);
        let source: NonNull<u8> = NonNull::from(&mut *value).cast();

        let src = source.as_ptr();
        let dst = self.data.as_ptr();

        let count = mem::size_of::<T>();

        std::ptr::copy_nonoverlapping::<u8>(src, dst, count);
    }
}

#[cfg(test)]
mod tests {
    use crate::table::cell::{UnsafeCell};

    #[test]
    fn cell_null() {
        let cell = UnsafeCell::<()>::new(());
        
        assert_eq!(cell.deref(), &());
    }

    #[test]
    fn cell_u8() {
        let mut cell = UnsafeCell::<u8>::new(1);

        assert_eq!(cell.deref(), &1);
        assert_eq!(cell.deref_mut(), &1);

        *cell.deref_mut() = 3;

        assert_eq!(cell.deref(), &3);
        assert_eq!(cell.deref_mut(), &3);
    }
}
