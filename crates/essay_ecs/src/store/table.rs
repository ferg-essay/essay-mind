use std::marker::PhantomData;
use std::{mem, ptr::NonNull};

use super::ptr::PtrOwn;
use super::row::{RowId, Row, RowMeta};
use super::row_meta::{RowTypeId, RowMetas, ColumnTypeId, ColumnType, RowType};
use super::type_meta::{TypeIndex, TypeMetas};

pub struct Table<'w> {
    row_meta: RowMetas,
    rows: Vec<Row<'w>>,
}

impl<'t> Table<'t> {
    pub fn new() -> Self {
        let mut row_meta = RowMetas::new();

        let column_type = row_meta.add_column::<()>();
        let mut col_vec = Vec::<ColumnTypeId>::new();
        col_vec.push(column_type.id());

        let row_type = row_meta.add_row::<()>(col_vec);
        // let row_type = self.single_role_type::<()>();

        Self {
            row_meta: row_meta,
            rows: Vec::new(),
        }
    }

    pub fn push<T:'static>(&mut self, value: T) -> EntityRef<T> {
        let row_type = self.row_meta.single_row_type::<T>();
        let row_id = RowId::new(self.rows.len() as u32);


        unsafe { 
            let mut row = Row::new(row_id, row_type);
            row.push(value, row_type.column(0));
            self.rows.push(row);
         }

        EntityRef {
            type_id: row_type.id(),
            row: row_id,
            marker: PhantomData,
        }
    }

    pub fn set<T:'static>(&mut self, entity_ref: &EntityRef<T>, value: T) {
        let type_id = entity_ref.type_id;
        let row_id = entity_ref.row;

        while self.rows.len() <= row_id.index() {
            let empty_type = self.row_meta.get_row::<()>().expect("");
            let empty_row = RowId::new(self.rows.len() as u32);
            unsafe {
                self.rows.push(Row::new(empty_row, empty_type));
            }
        }

        let row_type = self.row_meta.get_row_id(type_id).expect("type_id");

        let mut row = unsafe { Row::new(row_id, row_type) };

        self.rows[row_id.index()] = row;
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn get<T:'static>(&self, entity: &EntityRef<T>) -> Option<&T> {
        match self.rows.get(entity.row_id().index()) {
            Some(row) => unsafe { row.get(entity.row_id(), 0) },
            None => None,
        }
    }

    pub fn get_mut<T:'static>(&mut self, entity: &EntityRef<T>) -> Option<&mut T> {
        match self.rows.get_mut(entity.row_id().index()) {
            Some(row) => unsafe { 
                row.get_mut(entity.row_id(), 0) 
            },
            None => None,
        }
    }

    pub fn create_ref<T:'static>(&mut self, row_index: u32) -> EntityRef<T> {
        let row_type = self.row_meta.single_row_type::<T>();

        EntityRef {
            type_id: row_type.id(),
            row: RowId::new(row_index),
            marker: PhantomData,
        }
    }

    pub fn iter_by_type<T:'static>(&self) -> EntityIterator<T> {
        match self.row_meta.get_row::<T>() {
            None => todo!(),
            Some(row_type) => {
                EntityIterator::new(&self, row_type.id())
            }
        }
    }

    pub fn iter_mut_by_type<T:'static>(&mut self) -> EntityMutIterator<T> {
        match self.row_meta.get_row::<T>() {
            None => todo!(),
            Some(row_type) => {
                EntityMutIterator::new(self, row_type.id())
            }
        }
    }
}

pub struct EntityRef<T> {
    row: RowId,
    type_id: RowTypeId,
    marker: PhantomData<T>,
}

impl<T> EntityRef<T> {
    pub fn row_id(&self) -> RowId {
        self.row
    }

    pub fn row_type_id(&self) -> RowTypeId {
        self.type_id
    }
}



struct RowCursor<'a, 't> {
    table: &'a Table<'t>,
    index: usize,
}

impl<'a, 't> RowCursor<'a, 't> {
    fn next(&mut self, row_type: RowTypeId) -> Option<&Row<'t>> {
        while self.index < self.table.len() {
           let index = self.index;
            self.index = index + 1;

            if let Some(row) = self.table.rows.get(index) {
                if row.type_id() == row_type {
                    return Some(row)
                }
            }
        }

        None
    }
}

pub struct EntityIterator<'a, 't, T> {
    cursor: RowCursor<'a, 't>,
    type_id: RowTypeId,
    marker: PhantomData<T>,
}

impl<'a, 't, T> EntityIterator<'a, 't, T> {
    pub fn new(table: &'a Table<'t>, type_id: RowTypeId) -> Self {
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
                    return unsafe { Some(row.ptr(0).deref()) };
            }
        }
    }
}

pub struct EntityMutIterator<'a, 't, T> {
    cursor: RowCursor<'a, 't>,
    type_id: RowTypeId,
    marker: PhantomData<T>,
}

impl<'a, 't, T> EntityMutIterator<'a, 't, T> {
    pub fn new(table: &'a mut Table<'t>, type_id: RowTypeId) -> Self {
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
                return unsafe { Some(row.ptr(0).deref_mut()) };
            }
        }
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
        values = table.iter_by_type().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.push(TestB(10000));
        assert_eq!(table.len(), 2);

        let mut values = Vec::<String>::new();
        values = table.iter_by_type().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.iter_by_type().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000)");

        table.push(TestB(100));
        assert_eq!(table.len(), 3);

        values = table.iter_by_type().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.iter_by_type().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        for entity in table.iter_mut_by_type::<TestB>() {
            entity.0 += 1;
        }
        //table.iter_by_type().map(|t: &TestB| values.push(format!("{:?}", t)));
        values = table.iter_by_type().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);
}