use std::mem::{self, ManuallyDrop};
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use std::{marker::PhantomData, cmp};
use std::alloc::Layout;

use super::meta::{ColumnTypeId, ColumnType, RowMetas};

pub struct Column<'c, T> {
    meta: ColumnType,
    pad_size: usize,

    data: NonNull<u8>,
    
    len: usize,
    capacity: usize,

    marker: PhantomData<&'c T>,
}

impl<'c, T:'static> Column<'c, T> {
    const INC : usize = if mem::size_of::<T>() < 8 {
        8
    } else if mem::size_of::<T>() < 64 {
        4
    } else {
        1
    };

    pub(crate) fn new(metas: &mut RowMetas) -> Self {
        let id = metas.add_column::<T>();
        let meta = metas.get_column(id);

        let pad_size = meta.layout_padded().size();
        
        // zero-length items are pre-allocated
        let length = if pad_size == 0 { 1 } else { 0 };
        let capacity = length;

        let data = dangling_data(meta.layout_padded().align());
        
        Self {
            meta: meta.clone(),

            data: data,
            
            pad_size: pad_size,
            len: length,
            capacity: capacity,

            marker: Default::default(),
        }
    }
    
    pub fn id(&self) -> ColumnTypeId {
        self.meta.id()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    pub fn get(&self, index: usize) -> Option<&'c T> {
        if index < self.len {
            let offset = self.offset(index);

            unsafe {
                Some(&*self.data.as_ptr().add(offset).cast::<T>())
            }
        } else {
            None
        }
    }
    
    pub fn get_mut(&self, index: usize) -> Option<&'c mut T> {
        if index < self.len {
            let offset = self.offset(index);

            unsafe {
                Some(&mut *self.data.as_ptr().add(offset).cast::<T>())
            }
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        self.reserve(1);

        let index = self.len;

        unsafe {
            self.write(index, value);
        }
        
        self.len += 1;

        index
    }

    unsafe fn write(&mut self, index: usize, value: T) {
        assert!(index < self.capacity);

        let mut value = ManuallyDrop::new(value);
        let source: NonNull<u8> = NonNull::from(&mut *value).cast();

        let src = source.as_ptr();

        let offset = self.offset(index);
        let dst = self.data.as_ptr().add(offset);

        let count = mem::size_of::<T>();

        std::ptr::copy_nonoverlapping::<u8>(src, dst, count);
    }
    
    #[inline]
    fn offset(&self, index: usize) -> usize {
        self.pad_size * index
    }

    pub(crate) fn reserve(&mut self, len: usize) {
        let avail = self.capacity - self.len;

        if avail < len {
            let delta = cmp::max(Self::INC, len - avail);

            self.extend(self.len + delta);
        }
    }

    pub(crate) fn reserve_exact(&mut self, len: usize) {
        let avail = self.capacity - self.len;

        if len < avail {
            let delta = len - avail;

            self.extend(self.len + delta);
        }
    }

    fn extend(&mut self, new_capacity: usize) {
        assert!(self.pad_size > 0, "zero sized column items can't be pushed");
        assert!(self.capacity < new_capacity);

        let layout = self.array_layout(new_capacity);

        let data = if self.capacity == 0 {
            unsafe {
                std::alloc::alloc(layout)
            }
        } else {
            unsafe {
                std::alloc::realloc(
                    self.data.as_ptr(),
                    layout,
                    layout.size(),
                 )
            }
        };

        self.data = NonNull::new(data).unwrap();
        self.capacity = new_capacity;
    }
    
    fn array_layout(&mut self, n: usize) -> Layout {
        unsafe {
            let size = n * self.meta.size_padded();
            Layout::from_size_align_unchecked(size, self.meta.layout_padded().align())
        }
    }
}

fn dangling_data(align: usize) -> NonNull<u8> {
    if align > 0 {
        assert!(align.is_power_of_two());

        unsafe { NonNull::new_unchecked(align as *mut u8) }
    } else {
        unsafe { NonNull::new_unchecked(8 as *mut u8) }
    }
}

#[cfg(test)]
mod tests {
    use crate::store::meta::RowMetas;

    use super::Column;

    #[test]
    fn col_null() {
        let mut metas = RowMetas::new();
        let col = Column::<()>::new(&mut metas);

        assert_eq!(col.capacity(), 1);
        assert_eq!(col.len(), 1);
        
        //assert_eq!(col.push(()), 0);
        assert_eq!(col.get(0), Some(&()));
        assert_eq!(col.get(1), None);
    }

    #[test]
    fn col_u8() {
        let mut metas = RowMetas::new();
        let mut col = Column::<u8>::new(&mut metas);

        assert_eq!(col.capacity(), 0);
        assert_eq!(col.len(), 0);
        
        assert_eq!(col.get(0), None);

        assert_eq!(col.push(1), 0);

        assert_eq!(col.capacity(), 8);
        assert_eq!(col.len(), 1);

        assert_eq!(col.get(0), Some(&1));
        assert_eq!(col.get(1), None);

        assert_eq!(col.push(2), 1);

        assert_eq!(col.capacity(), 8);
        assert_eq!(col.len(), 2);

        assert_eq!(col.get(0), Some(&1));
        assert_eq!(col.get(1), Some(&2));
        assert_eq!(col.get(2), None);
    }

    #[test]
    fn col_u16() {
        let mut metas = RowMetas::new();
        let mut col = Column::<TestA>::new(&mut metas);

        assert_eq!(col.capacity(), 0);
        assert_eq!(col.len(), 0);
        
        assert_eq!(col.get(0), None);

        assert_eq!(col.push(TestA(1)), 0);

        assert_eq!(col.capacity(), 8);
        assert_eq!(col.len(), 1);

        assert_eq!(col.get(0), Some(&TestA(1)));
        assert_eq!(col.get(1), None);

        assert_eq!(col.push(TestA(1002)), 1);

        assert_eq!(col.capacity(), 8);
        assert_eq!(col.len(), 2);

        assert_eq!(col.get(0), Some(&TestA(1)));
        assert_eq!(col.get(1), Some(&TestA(1002)));
        assert_eq!(col.get(2), None);
    }

    #[derive(Debug, PartialEq)]
    struct TestA(u16);
}
