use std::{ptr::NonNull, mem, marker::PhantomData};

use super::{prelude::TypeIndex, ptr::PtrOwn, row_meta::{RowType, RowTypeId, ColumnType}};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RowId(u32);

pub(crate) struct Row<'t> {
    row_id: RowId,
    type_id: RowTypeId,
    data: Vec<u8>,
    ptrs: Vec<PtrOwn<'t>>,
}

pub struct RowBuilder<'t> {
    id: RowId,
    row_type: &'t RowType,
    ptrs: Vec<PtrOwn<'t>>,
    offset: usize,
}

// TODO: alignment, drop, columns, non-vec backing
impl<'t> Row<'t> {
    pub(crate) unsafe fn new(
        row_id: RowId, 
        row_type: &RowType,
    ) -> Self {
        let len = row_type.length();

        let mut data = Vec::<u8>::new();
        data.resize(len, 0); // TODO: ignoring alignment

        /*
        let mut storage = unsafe { NonNull::new_unchecked(data.as_mut_ptr()) };

        let ptr = PtrOwn::make_into(value, &mut storage);
        */

        Self {
            row_id: row_id,
            type_id: row_type.id(),
            data: data,
            ptrs: Vec::new(),
        }
    }

    pub(crate) unsafe fn push<T>(&mut self, value: T, col_type: &ColumnType) {
        let offset = col_type.offset();

        let mut storage = unsafe { 
            NonNull::new_unchecked(self.data.as_mut_ptr().add(offset))
        };

        let ptr = PtrOwn::make_into(value, &mut storage);

        self.ptrs.push(ptr);
    }

    /*
    pub unsafe fn new_old<T>(value: T, row_id: RowId, row_type: RowTypeId) -> Self {
        let len = mem::size_of::<T>();

        let mut data = Vec::<u8>::new();
        data.resize(len, 0); // TODO: ignoring alignment

        let mut storage = unsafe { NonNull::new_unchecked(data.as_mut_ptr()) };

        let ptr = PtrOwn::make_into(value, &mut storage);

        Self {
            row_id: row_id,
            type_id: row_type,
            _data: data,
            ptr: ptr,
        }
    }
    */

    pub fn type_id(&self) -> RowTypeId {
        self.type_id
    }

    pub fn ptr(&self, index: usize) -> &PtrOwn<'t> {
        self.ptrs.get(index).expect("unavailable index")
    }

    pub unsafe fn get<T:'static>(&self, row_id: RowId, index: usize) -> Option<&T> {
        if row_id == self.row_id {
            Some(self.ptrs.get(index).expect("ptr unassigned").deref())
        } else {
            None
        }
    }

    pub unsafe fn get_mut<T:'static>(&mut self, row_id: RowId, index: usize) -> Option<&mut T> {
        if row_id == self.row_id {
            Some(self.ptrs.get(index).expect("ptr unassigned").deref_mut())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowMeta {
    row_id: RowId,
    type_id: TypeIndex,
}

impl RowMeta {
    pub fn new(row_id: RowId, type_id: TypeIndex) -> Self {
        Self {
            row_id,
            type_id,
        }
    }

    pub(crate) fn id(&self) -> RowId {
        self.row_id
    }
}

impl RowId {
    pub fn new(id: u32) -> RowId {
        RowId(id)
    }

    #[inline]
    pub const fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for RowId {
    fn from(value: u32) -> Self {
        RowId(value)
    }
}
