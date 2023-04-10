use std::marker::PhantomData;

use super::prelude::ViewTypeId;
use super::row::{RowId, Row};
use super::row_meta::{RowTypeId, RowMetas, ColumnTypeId, ColumnType, RowType, InsertMapBuilder};

pub struct Table<'w> {
    row_meta: RowMetas,
    //entity_meta: EntityMeta,
    rows: Vec<Row<'w>>,

    type_rows: Vec<Vec<RowId>>,
}

//
// query tuples of components
//

pub trait ViewQuery<'t> {
    type Item;

    fn build(query: &mut ViewQueryBuilder);

    unsafe fn query<'a>(row: &'a Row<'t>, cursor: &mut ViewQueryCursor) -> Self::Item;
}

enum AccessType {
    AccessRef,
    AccessMut
}

pub struct ViewQueryBuilder<'a> {
    meta: &'a mut RowMetas, 
    cols: Vec<ColumnTypeId>,
}

pub struct ViewQueryMap {
    view: ViewTypeId,
    cols: Vec<usize>,
}

impl ViewQueryMap {
    fn new_cursor(&self) -> ViewQueryCursor {
        ViewQueryCursor {
            cols: &self.cols,
            index: 0,
        }
    }
}

pub struct ViewQueryCursor<'a> {
    cols: &'a Vec<usize>,
    index: usize,
}

impl<'a> ViewQueryCursor<'a> {
    pub fn next(&mut self) -> usize {
        let index = self.index;
        self.index += 1;

        self.cols[index]
    }
}

impl<'a> ViewQueryBuilder<'a> {
    fn new(meta: &'a mut RowMetas) -> Self {
        Self {
            meta: meta,
            cols: Vec::new(),
        }
    }

    fn add_ref<T:'static>(&mut self) {
        let col_type = self.meta.add_column::<T>();

        self.cols.push(col_type.id());
    }

    fn add_mut<T:'static>(&mut self) {
        let col_type = self.meta.add_column::<T>();

        self.cols.push(col_type.id());
    }

    fn build(self) -> ViewQueryMap {
        let view_id = self.meta.add_view(self.cols.clone());
        let view = self.meta.get_view(view_id);

        let cols = self.cols.iter()
            .map(|col_id| view.column_position(*col_id).unwrap())
            .collect();

        ViewQueryMap {
            view: view_id,
            cols: cols,
        }
    }
}

//
// implementation
//

impl<'t> Table<'t> {
    pub fn new() -> Self {
        let mut row_meta = RowMetas::new();
        //let entity_meta = EntityMeta::new();

        let column_type = row_meta.add_column::<()>();
        let mut col_vec = Vec::<ColumnTypeId>::new();
        col_vec.push(column_type.id());

        let row_type = row_meta.add_row(col_vec);
        // row_meta.add_row_type::<()>(row_type);
        // let row_type = self.single_role_type::<()>();

        Self {
            row_meta: row_meta,
            //entity_meta: entity_meta,
            rows: Vec::new(),

            type_rows: Vec::new(),
        }
    }

    pub(crate) fn meta(&self) -> &RowMetas {
        &self.row_meta
    }

    pub(crate) fn meta_mut(&mut self) -> &mut RowMetas {
        &mut self.row_meta
    }

    /*
    pub fn column_type<T:'static>(&mut self) -> &ColumnType {
        self.row_meta.add_column::<T>()
    }
    */

    /*
    pub(crate) fn get_column_type_id<T:'static>(&self) -> Option<ColumnTypeId> {
        self.row_meta.get_column_type_id::<T>()
    }
    */

    pub fn get_row_type(&self, row_type_id: RowTypeId) -> &RowType {
        self.row_meta.get_row_id(row_type_id)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn get<T:'static>(&self, entity: &RowRef<T>) -> Option<&T> {
        match self.rows.get(entity.row_id().index()) {
            Some(row) => unsafe { Some(row.get(0)) },
            None => None,
        }
    }

    pub fn get_mut<T:'static>(&mut self, entity: &RowRef<T>) -> Option<&mut T> {
        match self.rows.get_mut(entity.row_id().index()) {
            Some(row) => unsafe { 
                Some(row.get_mut(0))
            },
            None => None,
        }
    }

    pub(crate) fn get_row(&self, row_id: RowId) -> Option<&Row> {
        self.rows.get(row_id.index())
    }

    pub(crate) fn get_mut_row(&mut self, row_id: RowId) -> Option<&mut Row<'t>> {
        self.rows.get_mut(row_id.index())
    }

    pub fn push<T:'static>(&mut self, value: T) -> RowRef<T> {
        let mut builder = InsertMapBuilder::new();
        builder.push(self.row_meta.add_column::<T>().id());

        let row_type_id = self.row_meta.add_row(builder.columns().clone());
        let row_type = self.row_meta.get_row_id(row_type_id);
        let cols = builder.build_insert(row_type); // self.row_meta.get_row_id(row_type));

        let row_id = unsafe {
            let row = self.push_empty_row(row_type_id);

            row.insert(&cols, 0, value);

            row.id()
        };

        /*
        let row_type = self.row_meta.get_row_id(row_type_id);
        let row_id = RowId::new(self.rows.len() as u32);

        unsafe { 
            let mut row = Row::new(row_id, row_type);
            row.push(value, row_type.column(0));
            self.rows.push(row);
        }
        */

        RowRef {
            type_id: row_type_id,
            row: row_id,
            marker: PhantomData,
        }
    }

    pub(crate) unsafe fn push_empty_row(&mut self, row_type_id: RowTypeId) -> &mut Row<'t> {
        let row_type = self.row_meta.get_row_id(row_type_id);
        let row_id = RowId::new(self.rows.len() as u32);

        while self.type_rows.len() <= row_type_id.index() {
            self.type_rows.push(Vec::new());
        }

        self.type_rows[row_type_id.index()].push(row_id);

        unsafe {
            self.rows.push(Row::new(row_id, row_type));

            self.rows.get_mut(row_id.index()).unwrap()
        }
    }

    pub fn replace_push<T:'static>(&mut self, row_id: RowId, value: T) {
        let col_type_id = self.meta_mut().add_column::<T>().id();
        let row = self.rows.get(row_id.index()).unwrap();
        let old_type_id = row.type_id();
        let new_type_id = self.row_meta.push_row(old_type_id, col_type_id);
        let old_type = self.row_meta.get_row_id(old_type_id);
        let new_type = self.row_meta.get_row_id(new_type_id);

        let new_row = unsafe {
            row.expand(value, old_type, new_type, col_type_id)
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
            let empty_type = self.row_meta.get_single_view_type::<()>().unwrap();
            let empty_row = RowId::new(self.rows.len() as u32);
            unsafe {
                todo!();
            //       self.rows.push(Row::new(empty_row, empty_type));
            }
        }

        let row_type = self.row_meta.get_row_id(type_id);

        let row = unsafe { Row::new(row_id, row_type) };

        self.rows[row_id.index()] = row;
    }

    /*
    pub fn get_row(&self, row_id: RowId) -> &'t Row {
        self.rows.get(row_id.index())
    }

    pub fn get_mut_row(&self, row_id: RowId) -> &'t mut Row {
        self.rows.get_mut(row_id.index())
    }
    */

    /*
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
    */

    pub fn get_row_value<T:'static>(
        &self, 
        row_id: RowId,
        cols: &Vec<usize>,
    ) -> Option<&T> {
        match self.rows.get(row_id.index()) {
            Some(row) => {
                unsafe {
                    Some(row.get(*cols.get(0).unwrap()))
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

    pub fn iter_by_type<T:ViewQuery<'t>+'static>(&mut self) -> ViewIterator2<'_,'t,T> {
        let mut builder = ViewQueryBuilder::new(self.meta_mut());

        T::build(&mut builder);

        let view_query = builder.build();
        
        unsafe { self.iter_by_view(view_query) }
    }

    unsafe fn iter_by_view<T:ViewQuery<'t>+'static>(
        &self, 
        view_query: ViewQueryMap
    ) -> ViewIterator2<'_,'t,T> {
        ViewIterator2::new(self, view_query)
    }

    pub fn iter_mut_by_type<T:'static>(&mut self) -> EntityMutIterator<T> {
        match self.row_meta.get_single_view_type::<T>() {
            None => todo!(),
            Some(view_type) => {
                EntityMutIterator::new(self, view_type)
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
    fn next(&mut self, view_type: ViewTypeId) -> Option<&Row<'t>> {
        while self.index < self.table.len() {
           let index = self.index;
            self.index = index + 1;

            if let Some(row) = self.table.rows.get(index) {
                todo!()
                /*
                if row.type_id() == view_type {
                    return Some(row)
                }
                */
            }
        }

        None
    }
}

pub struct EntityIterator<'a, 't, T> {
    cursor: RowCursor<'a, 't>,
    type_id: ViewTypeId,
    marker: PhantomData<T>,
}

impl<'a, 't, T> EntityIterator<'a, 't, T> {
    pub fn new(table: &'a Table<'t>, type_id: ViewTypeId) -> Self {
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
                todo!()
                //return unsafe { Some(row.get(0)) };
            }
        }
    }
}

pub struct EntityMutIterator<'a, 't, T> {
    cursor: RowCursor<'a, 't>,
    type_id: ViewTypeId,
    marker: PhantomData<T>,
}

impl<'a, 't, T> EntityMutIterator<'a, 't, T> {
    pub fn new(table: &'a mut Table<'t>, type_id: ViewTypeId) -> Self {
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
                todo!()
                //return unsafe { Some(row.get_mut(0)) };
            }
        }
    }
}

pub struct ViewIterator<'a, 't, T:ViewQuery<'t>> {
    cursor: ViewCursor<'a, 't>,
    view_id: ViewTypeId,
    view_query: ViewQueryMap,
    marker: PhantomData<T>,
}

impl<'a, 't, T:ViewQuery<'t>> ViewIterator<'a, 't, T> {
    pub fn new(table: &'a Table<'t>, view_query: ViewQueryMap) -> Self {
        Self {
            cursor: ViewCursor::new(table, view_query.view),
            view_id: view_query.view,
            view_query: view_query,
            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:ViewQuery<'t>+'static> Iterator for ViewIterator<'a, 't, T> {
    type Item=T::Item;

    fn next(&mut self) -> Option<T::Item> {
        todo!()
        /*
        match self.cursor.next() {
            None => { return None },
            Some(row) => {
                unsafe { Some(T::query(row, &mut self.view_query.new_cursor())) }
            }
        }
        */
    }
}

pub struct ViewCursor<'a, 't> {
    table: &'a Table<'t>,
    view_type: ViewTypeId,
    view_type_index: usize,
    row_index: usize,
}

impl<'a, 't> ViewCursor<'a, 't> {
    fn new(table: &'a Table<'t>, view_type: ViewTypeId) -> Self {
        Self {
            table: table,
            view_type,
            view_type_index: 0,
            row_index: 0,
        }
    }

    fn next(&mut self) -> Option<&Row<'a>> {
        let view = self.table.meta().get_view(self.view_type);

        while self.view_type_index < view.rows().len() {
            let row_type_id = view.rows()[self.view_type_index];
            let row_index = self.row_index;
            self.row_index += 1;

            match self.table.rows.get(row_index) {
                Some(row) => return Some(row),
                None => {},
            };

            self.view_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

pub struct ViewIterator2<'a, 't, T:ViewQuery<'t>> {
    table: &'a Table<'t>,

    view_id: ViewTypeId,
    query: ViewQueryMap,

    view_type_index: usize,

    row_index: usize,

    marker: PhantomData<T>,
}

impl<'a, 't, T:ViewQuery<'t>> ViewIterator2<'a, 't, T> {
    fn new(
        table: &'a Table<'t>, 
        query: ViewQueryMap,
    ) -> Self {
        Self {
            table: table,

            view_id: query.view,
            query,

            view_type_index: 0,
            row_index: 0,

            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:ViewQuery<'t>> Iterator for ViewIterator2<'a, 't, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let view = self.table.meta().get_view(self.view_id);

        while self.view_type_index < view.rows().len() {
            let view_row_id = view.rows()[self.view_type_index];
            let view_row = self.table.meta().get_view_row(view_row_id);
            let row_type_id = view_row.row_type_id();
            let row_index = self.row_index;
            self.row_index += 1;

            match self.table.type_rows[row_type_id.index()].get(row_index) {
                Some(row_id) => {
                    let row = &self.table.rows[row_id.index()];
                    return unsafe { Some(T::query(row, &mut self.query.new_cursor())) };
                }
                None => {},
            };

            self.view_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::store::prelude::Row;

    use super::{Table, ViewQuery, ViewQueryCursor};

    #[test]
    fn spawn() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push(TestA(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.iter_by_type::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.push(TestB(10000));
        assert_eq!(table.len(), 2);

        values = table.iter_by_type::<&TestA>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.iter_by_type::<&TestB>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000)");

        table.push(TestB(100));
        assert_eq!(table.len(), 3);

        values = table.iter_by_type::<&TestA>().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.iter_by_type::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        for entity in table.iter_by_type::<&mut TestB>() {
            entity.0 += 1;
        }
        //table.iter_by_type().map(|t: &TestB| values.push(format!("{:?}", t)));
        values = table.iter_by_type::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
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

    trait TestComponent:'static {}
    
    impl TestComponent for TestA {}
    impl TestComponent for TestB {}

    impl<'a,'t,T:TestComponent> ViewQuery<'t> for &'a T
    where 't: 'a
    {
        type Item = &'a T;

        fn build(query: &mut super::ViewQueryBuilder) {
            query.add_ref::<T>()
        }
    
        unsafe fn query<'b>(
            row: &'b Row<'t>, 
            cursor: &mut ViewQueryCursor
        ) -> Self::Item {
            row.get::<T>(cursor.next())
        }
    }

    impl<'a,'t,T:TestComponent> ViewQuery<'t> for &'a mut T
    where 't: 'a
    {
        type Item = &'a mut T;

        fn build(query: &mut super::ViewQueryBuilder) {
            query.add_ref::<T>()
        }
    
        unsafe fn query<'b>(
            row: &'b Row<'t>, 
            cursor: &mut ViewQueryCursor
        ) -> Self::Item {
            row.get_mut::<T>(cursor.next())
        }
    }
}