use std::{mem, ptr::NonNull};

use super::ptr::PtrOwn;

pub struct Table<'w> {
    rows: Vec<Row<'w>>,
}

impl<'e> Table<'e> {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
        }
    }

    fn push(&mut self, value: Row<'e>) {
        self.rows.push(value);
    }
}

struct Row<'e> {
    data: Vec<u8>,
    ptr: PtrOwn<'e>,
}

// TODO: alignment, drop
impl<'e> Row<'e> {
    fn new<T>(value: T) -> Self {
        let len = mem::size_of::<T>();
        let mut data = Vec::<u8>::new();
        data.resize(len, 0); // TODO: ignoring alignment

        let mut storage = unsafe { NonNull::new_unchecked(data.as_mut_ptr()) };

        let ptr = unsafe { PtrOwn::make_into(value, &mut storage) };

        Self {
            data: data,
            ptr: ptr,
        }
    }
}
