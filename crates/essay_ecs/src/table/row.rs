use std::{ptr::NonNull};

use super::{ptr::PtrOwn, row_meta::{RowType, RowTypeId, ColumnTypeId, ColumnItem}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RowId(u32);

pub struct Row<'t> {
    row_id: RowId,
    type_id: RowTypeId,
    data: Vec<u8>,
    ptrs: Vec<PtrOwn<'t>>,
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

        Self {
            row_id: row_id,
            type_id: row_type.id(),
            data: data,
            ptrs: Vec::new(),
        }
    }

    pub fn id(&self) -> RowId {
        self.row_id
    }

    pub fn type_id(&self) -> RowTypeId {
        self.type_id
    }

    pub fn ptr(&self, index: usize) -> &PtrOwn<'t> {
        self.ptrs.get(index).unwrap()
    }

    pub(crate) unsafe fn push<T>(&mut self, value: T, col_type: &ColumnItem) {
        let offset = col_type.offset();

        let mut storage = unsafe { 
            NonNull::new_unchecked(self.data.as_mut_ptr().add(offset))
        };

        let ptr = PtrOwn::make_into(value, &mut storage);

        self.ptrs.push(ptr);
    }

    pub(crate) unsafe fn replace_push<'a,T>(
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
        println!("replace_push {:?} {:?}", new_col.id(), new_col.offset());
        for new_col in new_type.columns() {
            println!("  col {:?} {:?}", new_col.id(), new_col_id);
            if new_col.id() == new_col_id {
                new_row.ptrs.push(new_ptr);
            } else {
                let old_col = old_type.column_find(new_col.id()).unwrap();

                new_row.copy(new_col, self, old_col);
                println!("  copy {:?}:{:?} {:?}:{:?}", 
                    new_col.id(), new_col.offset(),
                    old_col.id(), old_col.offset());
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
        //self.data.copy_from_slice(&old_row.data[old_offset..old_offset + length]);

        let mut storage = unsafe { 
            NonNull::new_unchecked(self.data.as_mut_ptr().add(new_offset))
        };

        let ptr = PtrOwn::new(storage);

        self.ptrs.push(ptr);
    }

    pub(crate) unsafe fn get_fun<'a,F,R:'static>(
        &'a self, 
        row_id: RowId, 
        ptr_map: &Vec<usize>,
        mut fun: F
    ) -> &'a R
    where F: FnMut(&'a Row, &Vec<usize>) -> &'a R {
        assert_eq!(row_id, self.row_id);

        fun(self, ptr_map)
    }

    pub(crate) unsafe fn get<T:'static>(&self, row_id: RowId, index: usize) -> Option<&T> {
        if row_id == self.row_id {
            Some(self.ptrs.get(index).unwrap().deref())
        } else {
            None
        }
    }

    pub unsafe fn get_mut<T:'static>(&mut self, row_id: RowId, index: usize) -> Option<&mut T> {
        if row_id == self.row_id {
            Some(self.ptrs.get(index).unwrap().deref_mut())
        } else {
            None
        }
    }

    pub(crate) fn insert<T:'static>(&self, index: usize, this: T) {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowMeta {
    row_id: RowId,
    type_id: RowTypeId,
}

impl RowMeta {
    pub fn new(row_id: RowId, type_id: RowTypeId) -> Self {
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
