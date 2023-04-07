use std::marker::PhantomData;

use super::row::{RowId, Row};
use super::row_meta::{RowTypeId, RowMetas, ColumnTypeId, ColumnType, RowType};

pub struct Table<'w> {
    row_meta: RowMetas,
    //entity_meta: EntityMeta,
    rows: Vec<Row<'w>>,

    type_rows: Vec<Vec<RowId>>,
}

impl<'t> Table<'t> {
    pub fn new() -> Self {
        let mut row_meta = RowMetas::new();
        //let entity_meta = EntityMeta::new();

        let column_type = row_meta.add_column::<()>();
        let mut col_vec = Vec::<ColumnTypeId>::new();
        col_vec.push(column_type.id());

        let row_type = row_meta.add_row(col_vec);
        row_meta.add_row_type::<()>(row_type);
        // let row_type = self.single_role_type::<()>();

        Self {
            row_meta: row_meta,
            //entity_meta: entity_meta,
            rows: Vec::new(),

            type_rows: Vec::new(),
        }
    }

    pub(crate) fn row_meta(&self) -> &RowMetas {
        &self.row_meta
    }

    pub(crate) fn row_meta_mut(&mut self) -> &mut RowMetas {
        &mut self.row_meta
    }

    pub fn column_type<T:'static>(&mut self) -> &ColumnType {
        self.row_meta.add_column::<T>()
    }

    pub(crate) fn get_column_type_id<T:'static>(&self) -> Option<ColumnTypeId> {
        self.row_meta.get_column_type_id::<T>()
    }

    pub fn get_row_type(&self, row_type_id: RowTypeId) -> &RowType {
        self.row_meta.get_row_id(row_type_id)
    }

    pub fn push<T:'static>(&mut self, value: T) -> RowRef<T> {
        let row_type_id = self.row_meta.single_row_type::<T>();
        let row_type = self.row_meta.get_row_id(row_type_id);
        let row_id = RowId::new(self.rows.len() as u32);

        unsafe { 
            let mut row = Row::new(row_id, row_type);
            row.push(value, row_type.column(0));
            self.rows.push(row);
        }

        while self.type_rows.len() <= row_type_id.index() {
            self.type_rows.push(Vec::new());
        }

        self.type_rows[row_type_id.index()].push(row_id);

        RowRef {
            type_id: row_type_id,
            row: row_id,
            marker: PhantomData,
        }
    }

    pub fn replace_push<T:'static>(&mut self, row_id: RowId, value: T) {
        let col_type_id = self.column_type::<T>().id();
        let row = self.rows.get(row_id.index()).unwrap();
        let old_type_id = row.type_id();
        let new_type_id = self.row_meta.push_row(old_type_id, col_type_id);
        let old_type = self.row_meta.get_row_id(old_type_id);
        let new_type = self.row_meta.get_row_id(new_type_id);

        let new_row = unsafe {
            row.replace_push(value, old_type, new_type, col_type_id)
        };

        let new_row_id = new_row.id();
        // let row = 0;

        self.rows[new_row_id.index()] = new_row;

        while self.type_rows.len() <= new_type_id.index() {
            self.type_rows.push(Vec::new());
        }

        self.type_rows[old_type_id.index()].retain(|row| *row != row_id);
        self.type_rows[new_type_id.index()].push(row_id);
    }

    pub fn set<T:'static>(&mut self, entity_ref: &RowRef<T>, value: T) {
        let type_id = entity_ref.type_id;
        let row_id = entity_ref.row;

        while self.rows.len() <= row_id.index() {
            let empty_type = self.row_meta.get_row_by_type::<()>().unwrap();
            let empty_row = RowId::new(self.rows.len() as u32);
            unsafe {
                self.rows.push(Row::new(empty_row, empty_type));
            }
        }

        let row_type = self.row_meta.get_row_id(type_id);

        let row = unsafe { Row::new(row_id, row_type) };

        self.rows[row_id.index()] = row;
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn get<T:'static>(&self, entity: &RowRef<T>) -> Option<&T> {
        match self.rows.get(entity.row_id().index()) {
            Some(row) => unsafe { row.get(entity.row_id(), 0) },
            None => None,
        }
    }

    pub fn get_mut<T:'static>(&mut self, entity: &RowRef<T>) -> Option<&mut T> {
        match self.rows.get_mut(entity.row_id().index()) {
            Some(row) => unsafe { 
                row.get_mut(entity.row_id(), 0) 
            },
            None => None,
        }
    }

    /*
    pub fn get_row(&self, row_id: RowId) -> &'t Row {
        self.rows.get(row_id.index())
    }

    pub fn get_mut_row(&self, row_id: RowId) -> &'t mut Row {
        self.rows.get_mut(row_id.index())
    }
    */

    pub(crate) unsafe fn get_fun<'a,F,R:'static>(
        &'a self, 
        row_id: RowId, 
        ptr_map: &Vec<usize>,
        mut fun: F
    ) -> &'a R
    where F: FnMut(&'a Row, &Vec<usize>) -> &'a R {
        let row = self.rows.get(row_id.index()).unwrap();

        fun(row, ptr_map)
    }

    pub fn get_row<T:'static>(
        &self, 
        row_id: RowId,
        cols: &Vec<usize>,
    ) -> Option<&T> {
        match self.rows.get(row_id.index()) {
            Some(row) => {
                unsafe {
                    row.get(row_id, *cols.get(0).unwrap())
                }
            },
            None => None,
        }
    }

    pub fn create_ref<T:'static>(&mut self, row_index: u32) -> RowRef<T> {
        let row_type = self.row_meta.single_row_type::<T>();

        RowRef {
            type_id: row_type,
            row: RowId::new(row_index),
            marker: PhantomData,
        }
    }

    pub fn iter_by_type<T:'static>(&self) -> EntityIterator<T> {
        match self.row_meta.get_row_by_type::<T>() {
            None => todo!(),
            Some(row_type) => {
                EntityIterator::new(&self, row_type.id())
            }
        }
    }

    pub fn iter_mut_by_type<T:'static>(&mut self) -> EntityMutIterator<T> {
        match self.row_meta.get_row_by_type::<T>() {
            None => todo!(),
            Some(row_type) => {
                EntityMutIterator::new(self, row_type.id())
            }
        }
    }

    pub(crate) fn push_row_type(
        &mut self, 
        row_id: RowTypeId, 
        column_id: ColumnTypeId
    ) -> RowTypeId {
        self.row_meta.push_row(row_id, column_id)
    }

    pub(crate) fn get_row_by_type_index(&self, row_type_id: RowTypeId, row_index: usize) -> Option<&'t Row> {
        match self.type_rows[row_type_id.index()].get(row_index) {
            Some(row_id) => self.rows.get(row_id.index()),
            None => None,
        }
    }
}

pub struct RowRef<T> {
    row: RowId,
    type_id: RowTypeId,
    marker: PhantomData<T>,
}

impl<T> RowRef<T> {
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

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn spawn() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push(TestA(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.iter_by_type()
            .map(|t: &TestA| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.push(TestB(10000));
        assert_eq!(table.len(), 2);

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

    #[test]
    fn eval() {
        let mut table = Table::new();
        let row_id = table.push(TestA(1)).row_id();



    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);
}