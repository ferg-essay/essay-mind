use std::{ptr::NonNull};

use super::{ptr::PtrOwn, meta::{RowTypeId, ColumnTypeId}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RowId(u32);

/*
pub struct Row<'t> {
    row_id: RowId,
    type_id: RowTypeId,
    data: Vec<u8>,
    ptrs: Vec<PtrOwn<'t>>,
}
*/

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
