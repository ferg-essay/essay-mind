use std::marker::PhantomData;

use super::prelude::{ViewTypeId, QueryCursor};
use super::row::{RowId, Row};
use super::row_meta::{RowTypeId, RowMetas, ColumnTypeId, ColumnType, RowType, InsertBuilder, Query, QueryBuilder, QueryPlan, Insert, InsertPlan, InsertCursor};

pub struct Table<'w> {
    row_meta: RowMetas,
    rows: Vec<Row<'w>>,

    type_rows: Vec<Vec<RowId>>,
}

pub struct RowRef {
    row: RowId,
    type_id: RowTypeId,
}

//
// implementation
//

impl<'t> Table<'t> {
    pub fn new() -> Self {
        let mut row_meta = RowMetas::new();

        let column_type = row_meta.add_column::<()>();
        let mut columns = Vec::<ColumnTypeId>::new();
        columns.push(column_type);

        row_meta.add_row(columns.clone());
        row_meta.add_view(columns);

        Self {
            row_meta: row_meta,
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

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn get_row(&self, row_id: RowId) -> Option<&Row> {
        self.rows.get(row_id.index())
    }

    pub(crate) fn get_mut_row(&mut self, row_id: RowId) -> Option<&mut Row<'t>> {
        self.rows.get_mut(row_id.index())
    }

    pub fn push_single<T:'static>(&mut self, value: T) -> RowRef {
        self.push::<Single<T>>(value)
    }

    pub fn push<T:Insert>(&mut self, value: T::Item) -> RowRef {
        let plan = self.insert_plan::<T>();

        unsafe {
            let row = self.push_empty_row(plan.row_type());

            T::insert(&mut plan.cursor(row), value);

            RowRef {
                type_id: plan.row_type(),
                row: row.id(),
            }
        }
    }

    fn insert_plan<T:Insert>(&mut self) -> InsertPlan {
        let mut builder = InsertBuilder::new(self.meta_mut());

        T::build(&mut builder);

        builder.build()
    }

    unsafe fn push_empty_row(&mut self, row_type_id: RowTypeId) -> &mut Row<'t> {
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

    pub fn replace_extend<T:'static>(&mut self, row_id: RowId, value: T) {
        let col_type_id = self.meta_mut().add_column::<T>();
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

    pub fn get_row_value<T:'static>(
        &self, 
        row_id: RowId,
        cols: &Vec<usize>,
    ) -> Option<&T> {
        match self.rows.get(row_id.index()) {
            Some(row) => {
                unsafe {
                    Some(row.deref(*cols.get(0).unwrap()))
                }
            },
            None => None,
        }
    }

    //
    // query
    //

    pub fn query<'a,T:Query>(&mut self) -> QueryIterator<'_,'t,T> {
        let plan = self.get_query_plan::<T>();
        
        unsafe { self.query_with_plan(plan) }
    }

    pub(crate) fn get_query_plan<'a,T:Query>(&mut self) -> QueryPlan {
        let mut builder = QueryBuilder::new(self.meta_mut());

        T::build(&mut builder);

        builder.build()
    }

    pub(crate) unsafe fn query_with_plan<'a,T:Query>(
        &self, 
        plan: QueryPlan
    ) -> QueryIterator<'_,'t,T> {
        QueryIterator::new(self, plan)
    }

    //
    // row ref
    //

    pub fn get<T:'static>(&self, entity: &RowRef) -> Option<&T> {
        match self.rows.get(entity.row_id().index()) {
            Some(row) => unsafe { Some(row.deref(0)) },
            None => None,
        }
    }

    pub fn get_mut<T:'static>(&mut self, entity: &RowRef) -> Option<&mut T> {
        match self.rows.get_mut(entity.row_id().index()) {
            Some(row) => unsafe { 
                Some(row.deref_mut(0))
            },
            None => None,
        }
    }

    pub fn create_ref<T:'static>(&mut self, row_index: u32) -> RowRef {
        let row_type = self.row_meta.single_row_type::<T>();

        RowRef {
            type_id: row_type,
            row: RowId::new(row_index),
        }
    }

    pub fn replace<T:'static>(&mut self, entity_ref: &RowRef, value: T) {
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

    pub(crate) fn push_row_type(
        &mut self, 
        row_id: RowTypeId, 
        column_id: ColumnTypeId
    ) -> RowTypeId {
        self.row_meta.push_row(row_id, column_id)
    }

    fn get_row_by_type_index(
        &self, 
        row_type_id: RowTypeId, 
        row_index: usize
    ) -> Option<&Row<'t>> {
        match self.type_rows[row_type_id.index()].get(row_index) {
            Some(row_id) => self.rows.get(row_id.index()),
            None => None,
        }
    }
}

pub struct Single<T> {
    marker: PhantomData<T>,
}

impl<T:'static> Insert for Single<T> {
    type Item = T;

    fn build(builder: &mut InsertBuilder) {
        builder.add_column::<T>()
    }

    unsafe fn insert(cursor: &mut InsertCursor, value: Self::Item) {
        cursor.insert(value)
    }
}

pub struct QueryIterator<'a, 't, T:Query> {
    table: &'a Table<'t>,

    view_id: ViewTypeId,
    query: QueryPlan,

    view_type_index: usize,

    row_index: usize,

    marker: PhantomData<T>,
}

impl<'a, 't, T:Query> QueryIterator<'a, 't, T> {
    fn new(
        table: &'a Table<'t>, 
        query: QueryPlan,
    ) -> Self {
        Self {
            table: table,

            view_id: query.view(),
            query,

            view_type_index: 0,
            row_index: 0,

            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:Query> Iterator for QueryIterator<'a, 't, T>
{
    type Item = T::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let view = self.table.meta().get_view(self.view_id);

        while self.view_type_index < view.rows().len() {
            let view_row_id = view.rows()[self.view_type_index];
            let view_row = self.table.meta().get_view_row(view_row_id);
            let row_type_id = view_row.row_type_id();
            let row_index = self.row_index;
            self.row_index += 1;

            match self.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
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

impl RowRef {
    pub fn row_id(&self) -> RowId {
        self.row
    }

    pub fn row_type_id(&self) -> RowTypeId {
        self.type_id
    }
}

#[cfg(test)]
mod tests {
    use crate::store::prelude::{Row, QueryCursor};

    use super::{Table, Query};

    #[test]
    fn spawn() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push_single(TestA(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.query::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.push_single(TestB(10000));
        assert_eq!(table.len(), 2);

        values = table.query::<&TestA>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.query::<&TestB>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000)");

        table.push_single(TestB(100));
        assert_eq!(table.len(), 3);

        values = table.query::<&TestA>().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        for entity in table.query::<&mut TestB>() {
            entity.0 += 1;
        }
        //table.iter_by_type().map(|t: &TestB| values.push(format!("{:?}", t)));
        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[test]
    fn eval() {
        let mut table = Table::new();
        let row_id = table.push_single(TestA(1)).row_id();



    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);

    trait TestComponent:'static {}
    
    impl TestComponent for TestA {}
    impl TestComponent for TestB {}

    impl<T:TestComponent> Query for &T
    {
        type Item<'a> = &'a T;

        fn build(query: &mut super::QueryBuilder) {
            query.add_ref::<T>()
        }
    
        unsafe fn query<'a>(row: &'a Row, cursor: &mut QueryCursor) -> Self::Item<'a> {
            cursor.deref::<T>(row)
        }
    }

    impl<T:TestComponent> Query for &mut T
    {
        type Item<'a> = &'a mut T;

        fn build(query: &mut super::QueryBuilder) {
            query.add_ref::<T>()
        }
    
        unsafe fn query<'a>(row: &'a Row, cursor: &mut QueryCursor) -> Self::Item<'a> {
            cursor.deref_mut::<T>(row)
        }
    }
}