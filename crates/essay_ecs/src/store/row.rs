use std::{ptr::NonNull};

use super::{ptr::PtrOwn, meta::{RowType, RowTypeId, ColumnTypeId, ColumnItem, RowMetas, InsertPlan}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RowId(u32);

pub struct Row<'t> {
    row_id: RowId,
    type_id: RowTypeId,
    data: Vec<u8>,
    ptrs: Vec<PtrOwn<'t>>,
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

// TODO: alignment, drop, columns, non-vec backing
impl<'t> Row<'t> {
    pub(crate) unsafe fn new(
        row_id: RowId, 
        row_type: &RowType,
    ) -> Self {
        let len = row_type.length();

        let mut data = Vec::<u8>::new();
        data.resize(len, 0); // TODO: ignoring alignment

        let mut row = Self {
            row_id: row_id,
            type_id: row_type.id(),
            data: data,
            ptrs: Vec::new(),
        };

        for col in row_type.columns() {
            let data = unsafe { 
                NonNull::new_unchecked(row.data.as_mut_ptr().add(col.offset()))
            };

            let ptr = PtrOwn::new(data);

            row.ptrs.push(ptr);
        }

        row
    }

    pub fn id(&self) -> RowId {
        self.row_id
    }

    pub fn type_id(&self) -> RowTypeId {
        self.type_id
    }

    pub(crate) unsafe fn deref<T:'static>(&self, index: usize) -> &'t T {
        self.ptrs.get(index).unwrap().deref()
    }

    pub(crate) unsafe fn deref_mut<T:'static>(&self, index: usize) -> &'t mut T {
        self.ptrs.get(index).unwrap().deref_mut()
    }

    pub(crate) unsafe fn write<T:'static>(&mut self, index: usize, value: T) {
        self.ptrs[index].write(value); // TODO: drop for replace(?)
    }

    pub(crate) unsafe fn expand<'a,T>(
        &self, 
        value: T, 
        old_type: &RowType,
        new_type: &RowType,
        new_col_id: ColumnTypeId,
    ) -> Row<'a> {
        let mut new_row = Row::new(self.row_id, new_type);

        let new_col = new_type.column_find(new_col_id).unwrap();

        let mut storage = unsafe { 
            NonNull::new_unchecked(new_row.data.as_mut_ptr().add(new_col.offset()))
        };

        let new_ptr = PtrOwn::make_into(value, &mut storage);

        for new_col in new_type.columns() {
            if new_col.id() == new_col_id {
                new_row.ptrs.push(new_ptr);
            } else {
                let old_col = old_type.column_find(new_col.id()).unwrap();

                new_row.copy(new_col, self, old_col);
            }
        }

        new_row
    }

    unsafe fn copy(&mut self, new_col: &ColumnItem, old_row: &Row, old_col: &ColumnItem) {
        assert_eq!(new_col.id(), old_col.id());
        assert_eq!(new_col.length(), old_col.length());

        let new_offset = new_col.offset();
        let old_offset = old_col.offset();
        let length = new_col.length();

        for i in 0..length {
            self.data[i + new_offset] = old_row.data[i + old_offset];
        }

        let mut storage = unsafe { 
            NonNull::new_unchecked(self.data.as_mut_ptr().add(new_offset))
        };

        let ptr = PtrOwn::new(storage);

        self.ptrs.push(ptr);
    }
}
