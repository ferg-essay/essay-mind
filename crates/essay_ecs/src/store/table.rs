use std::marker::PhantomData;
use std::{mem, ptr::NonNull};

use super::ptr::PtrOwn;
use super::type_info::{TypeIndex, TypeMetas};

pub struct Table<'w> {
    types: TypeMetas,
    row_metas: Vec<RowMeta>,
    rows: Vec<Row<'w>>,
}

impl<'t> Table<'t> {
    pub fn new() -> Self {
        Self {
            types: TypeMetas::new(),
            row_metas: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn push<T:'static>(&mut self, value: T) {
        let type_id = self.types.add_type::<T>();
        let row_id = RowId { index: self.rows.len() as u32 };



        unsafe { self.rows.push(Row::new(value, row_id, type_id)); }
        self.row_metas.push(RowMeta::new(row_id, type_id));
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    unsafe fn get_item<T:'static>(&self, row_meta: RowMeta) -> Option<&T> {
        match self.rows.get(row_meta.row_id.index()) {
            Some(row) => row.get_item(row_meta),
            None => None,
        }
    }

    fn get_mut_item<T:'static>(&mut self, entity: EntityRef<T>) -> Option<&mut T> {
        match self.rows.get_mut(entity.row.row_id.index()) {
            Some(row) => unsafe { row.get_mut_item(entity.row) },
            None => None,
        }
    }

    pub fn eval<T:'static,F>(&mut self, fun: &mut F)
        where F: FnMut(&mut T)
    {
        let type_id = self.types.add_type::<T>();

        for value in self.iter_mut_by_type::<T>() {
            fun(value);
        }
    }

    pub fn iter_by_type<T:'static>(&self) -> EntityIterator<T> {
        match self.types.get_id::<T>() {
            None => todo!(),
            Some(type_id) => {
                EntityIterator::new(&self, type_id)
            }
        }
    }

    pub fn iter_mut_by_type<T:'static>(&mut self) -> EntityMutIterator<T> {
        match self.types.get_id::<T>() {
            None => todo!(),
            Some(type_id) => {
                EntityMutIterator::new(self, type_id)
            }
        }
    }
}

pub struct EntityRef<T> {
    row: RowMeta,
    type_id: TypeIndex,
    marker: PhantomData<T>,
}

struct RowCursor<'a, 't> {
    table: &'a Table<'t>,
    index: usize,
}

impl<'a, 't> RowCursor<'a, 't> {
    fn next(&mut self, type_id: TypeIndex) -> Option<&Row<'t>> {
        while self.index < self.table.len() {
           let index = self.index;
            self.index = index + 1;

            if let Some(row) = self.table.rows.get(index) {
                if row.meta.type_id == type_id {
                    return Some(row)
                }
            }
        }

        None
    }
}

pub struct EntityIterator<'a, 't, T> {
    cursor: RowCursor<'a, 't>,
    type_id: TypeIndex,
    marker: PhantomData<T>,
}

impl<'a, 't, T> EntityIterator<'a, 't, T> {
    pub fn new(table: &'a Table<'t>, type_id: TypeIndex) -> Self {
        Self {
            cursor: RowCursor { table: table, index: 0 },
            type_id: type_id,
            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:'static> Iterator for EntityIterator<'a, 't, T> {
    type Item=&'a T;

    fn next(&mut self) -> Option<&'t T> {
        match self.cursor.next(self.type_id) {
            None => { return None },
            Some(row) => {
                    return unsafe { Some(row.ptr.deref()) };
            }
        }
    }
}

pub struct EntityMutIterator<'a, 't, T> {
    cursor: RowCursor<'a, 't>,
    type_id: TypeIndex,
    marker: PhantomData<T>,
}

impl<'a, 't, T> EntityMutIterator<'a, 't, T> {
    pub fn new(table: &'a mut Table<'t>, type_id: TypeIndex) -> Self {
        Self {
            cursor: RowCursor { table: table, index: 0 },
            type_id: type_id,
            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:'static> Iterator for EntityMutIterator<'a, 't, T> {
    type Item=&'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor.next(self.type_id) {
            None => { return None },
            Some(row) => {
                return unsafe { Some(row.ptr.deref_mut()) };
            }
        }
    }
}

pub struct Row<'t> {
    meta: RowMeta,
    _data: Vec<u8>,
    ptr: PtrOwn<'t>,
}

// TODO: alignment, drop
impl<'t> Row<'t> {
    unsafe fn new<T>(value: T, row_id: RowId, type_id: TypeIndex) -> Self {
        let len = mem::size_of::<T>();

        let mut data = Vec::<u8>::new();
        data.resize(len, 0); // TODO: ignoring alignment

        let mut storage = unsafe { NonNull::new_unchecked(data.as_mut_ptr()) };

        let ptr = PtrOwn::make_into(value, &mut storage);

        Self {
            meta: RowMeta::new(row_id, type_id),
            _data: data,
            ptr: ptr,
        }
    }

    unsafe fn get_item<T:'static>(&self, row_meta: RowMeta) -> Option<&T> {
        if row_meta.row_id == self.meta.row_id {
            Some(self.ptr.as_ref().deref())
        } else {
            None
        }
    }

    unsafe fn get_mut_item<T:'static>(&mut self, row_meta: RowMeta) -> Option<&mut T> {
        if row_meta.row_id == self.meta.row_id {
            Some(self.ptr.as_mut().deref_mut())
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
    fn new(row_id: RowId, type_id: TypeIndex) -> Self {
        Self {
            row_id,
            type_id,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RowId {
    index: u32,
}

impl RowId {
    #[inline]
    pub const fn index(&self) -> usize {
        self.index as usize
    }
}

pub struct RowRef<'w> {
    table: &'w Table<'w>,
    row_id: RowId,
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn spawn() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push(TestA(1));
        assert_eq!(table.len(), 1);

        let mut values = Vec::<String>::new();
        table.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        table.push(TestB(10000));
        assert_eq!(table.len(), 2);

        let mut values = Vec::<String>::new();
        table.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        table.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000)");

        table.push(TestB(100));
        assert_eq!(table.len(), 3);

        let mut values = Vec::<String>::new();
        table.eval(&mut|t: &mut TestA| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestA(1)");

        let mut values = Vec::<String>::new();
        table.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        let mut values = Vec::<String>::new();
        table.eval(&mut|t: &mut TestB| t.0 += 1);
        table.eval(&mut|t: &mut TestB| (&mut values).push(format!("{:?}", t)));
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);
}