use std::marker::PhantomData;

use super::column::Column;
use super::prelude::{ViewTypeId, QueryCursor2};
use super::row::{RowId, Row};
use super::meta::{RowTypeId, RowMetas, ColumnTypeId};

pub struct Table<'t> {
    meta: RowMetas,

    columns: Vec<Column<'t>>,

    rows: Vec<Row<'t>>,

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
            meta: row_meta,

            columns: Vec::new(),

            rows: Vec::new(),

            type_rows: Vec::new(),
        }
    }

    pub(crate) fn meta(&self) -> &RowMetas {
        &self.meta
    }

    pub(crate) fn meta_mut(&mut self) -> &mut RowMetas {
        &mut self.meta
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

    pub fn push_column<T:'static>(&mut self, value: T) -> RowRef {
        let mut builder = InsertBuilder::new(self.meta_mut());

        builder.add_column::<T>();

        let plan = builder.build();

        unsafe {
            let row = self.push_empty_row(plan.row_type());

            plan.cursor(row).insert(value);

            RowRef {
                type_id: plan.row_type(),
                row: row.id(),
            }
        }
    }

    pub fn push<T:Insert>(&mut self, value: T) -> RowRef {
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
        let row_type = self.meta.get_row_id(row_type_id);
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

    //
    // query
    //

    pub fn query<'a,T:Query>(&mut self) -> QueryIterator<'_,'t,T> {
        let plan = self.get_query_plan::<T>();
        
        unsafe { self.query_with_plan(plan) }
    }

    pub(crate) fn get_query_plan<T:Query>(&mut self) -> QueryPlan {
        let mut builder = QueryBuilder::new(self.meta_mut());

        T::build(&mut builder);

        builder.build()
    }

    pub(crate) unsafe fn query_with_plan<T:Query>(
        &self, 
        plan: QueryPlan
    ) -> QueryIterator<'_,'t,T> {
        QueryIterator::new(self, plan)
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

pub trait Insert:'static {
    fn build(builder: &mut InsertBuilder);

    unsafe fn insert(cursor: &mut InsertCursor, value: Self);
}

pub struct InsertBuilder<'a> {
    meta: &'a mut RowMetas,
    columns: Vec<ColumnTypeId>,
}

pub struct InsertPlan {
    row_type: RowTypeId,
    row_cols: Vec<usize>,
}

pub struct InsertCursor<'a, 't> {
    row: &'a mut Row<'t>,
    map: &'a InsertPlan,
    index: usize,
}

impl<'a> InsertBuilder<'a> {
    pub(crate) fn new(meta: &'a mut RowMetas) -> Self {
        Self {
            meta: meta,
            columns: Vec::new(),
        }
    }

    pub(crate) fn add_column<T:'static>(&mut self) {
        let id = self.meta.add_column::<T>();
        
        self.columns.push(id);
    }

    pub(crate) fn build(self) -> InsertPlan {
        let row_id = self.meta.add_row(self.columns.clone());
        let row = self.meta.get_row_id(row_id);

        let mut row_cols = Vec::<usize>::new();

        for col_id in &self.columns {
            row_cols.push(row.column_position(*col_id).unwrap());
        }

        InsertPlan {
            row_type: row.id(),
            row_cols: row_cols,
        }
    }
}

impl InsertPlan {
    pub fn index(&self, index: usize) -> usize {
        self.row_cols[index]
    }

    pub(crate) fn row_type(&self) -> RowTypeId {
        self.row_type
    }

    pub(crate) fn cursor<'a, 't>(&'a self, row: &'a mut Row<'t>) -> InsertCursor<'a, 't> {
        InsertCursor {
            map: &self,
            row: row,
            index: 0, 
        }
    }
}

impl<'a, 't> InsertCursor<'a, 't> {
    pub unsafe fn insert<T:'static>(&mut self, value: T) {
        let index = self.index;
        self.index += 1;

        self.row.write::<T>(self.map.row_cols[index], value);
    }
}

pub trait Query {
    type Item<'a>;

    fn build(query: &mut QueryBuilder);

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t>;
}

pub struct QueryCursor<'a,'t> {
    row: &'a Row<'t>,
    cols: &'a Vec<usize>,
    index: usize,
}

pub struct QueryBuilder<'a> {
    meta: &'a mut RowMetas, 
    cols: Vec<ColumnTypeId>,
}

pub(crate) struct QueryPlan {
    view: ViewTypeId,
    cols: Vec<usize>,
}

impl QueryPlan {
    pub(crate) fn new_cursor<'a,'t>(
        &'a self, 
        row: &'a Row<'t>
    ) -> QueryCursor<'a,'t> {
        QueryCursor {
            row: row,
            cols: &self.cols,
            index: 0,
        }
    }

    pub(crate) fn view(&self) -> ViewTypeId {
        self.view
    }
}

impl<'a,'t> QueryCursor<'a,'t> {
    pub unsafe fn deref<T:'static>(&mut self) -> &'t T {
        let index = self.index;
        self.index += 1;

        self.row.deref(self.cols[index])
    }

    pub unsafe fn deref_mut<T:'static>(&mut self) -> &'t mut T {
        let index = self.index;
        self.index += 1;

        self.row.deref_mut(self.cols[index])
    }
}

impl<'a> QueryBuilder<'a> {
    pub(crate) fn new(meta: &'a mut RowMetas) -> Self {
        Self {
            meta: meta,
            cols: Vec::new(),
        }
    }

    pub fn add_ref<T:'static>(&mut self) {
        let col_id = self.meta.add_column::<T>();

        self.cols.push(col_id);
    }

    pub fn add_mut<T:'static>(&mut self) {
        let col_id = self.meta.add_column::<T>();

        self.cols.push(col_id);
    }

    pub(crate) fn build(self) -> QueryPlan {
        let view_id = self.meta.add_view(self.cols.clone());
        let view = self.meta.get_view(view_id);

        let cols = self.cols.iter()
            .map(|col_id| view.column_position(*col_id).unwrap())
            .collect();

        QueryPlan {
            view: view_id,
            cols: cols,
        }
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
    type Item = T::Item<'t>;

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
                    return unsafe { Some(T::query(&mut self.query.new_cursor(row))) };
                }
                None => {},
            };

            self.view_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

//
// insert composed of tuples
//

macro_rules! impl_insert_tuple {
    ($($part:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($part:Insert),*> Insert for ($($part,)*)
        {
            fn build(builder: &mut InsertBuilder) {
                $(
                    $part::build(builder);
                )*
            }

            unsafe fn insert<'a>(cursor: &mut InsertCursor, value: Self) {
                let ($($part,)*) = value;
                $(
                    $part::insert(cursor, $part);
                )*
            }
        }
    }
}

//impl_query_tuple!();
impl_insert_tuple!(P1,P2);
//impl_query_tuple!(P1,P2,P3);
//impl_query_tuple!(P1,P2,P3,P4);
//impl_query_tuple!(P1,P2,P3,P4,P5);

//
// View query composed of tuples
//

macro_rules! impl_query_tuple {
    ($($part:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($part:Query,)*> Query for ($($part,)*)
        {
            type Item<'t> = ($(
                <$part as Query>::Item<'t>, // <'a>,
            )*);

            fn build(builder: &mut QueryBuilder) {
                $(
                    $part::build(builder);
                )*
            }

            unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { // <'a> {
                ($(
                    $part::query(cursor),
                )*)
            }
        }
    }
}

//impl_query_tuple!();
impl_query_tuple!(P1,P2);
impl_query_tuple!(P1,P2,P3);
//impl_query_tuple!(P1,P2,P3,P4);
//impl_query_tuple!(P1,P2,P3,P4,P5);

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
    use crate::store::{prelude::{Row}, meta::{}};

    use super::{Table, Query, QueryIterator, QueryCursor, InsertCursor, InsertBuilder, Insert};

    #[test]
    fn spawn() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push_column(TestA(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.query::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.push_column(TestB(10000));
        assert_eq!(table.len(), 2);

        values = table.query::<&TestA>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.query::<&TestB>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000)");

        table.push_column(TestB(100));
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
    fn push_type() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push::<TestA>(TestA(1));
        //table.push(TestC(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.query::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");
    }

    #[test]
    fn push_tuple() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.push((TestA(1),TestB(2)));
        //table.push(TestC(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.query::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");

//        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        //assert_eq!(values.join(","), "TestB(2)");

        //values = table.query::<(&TestA,&TestB)>().map(|(a:, b)| format!("({:?},{:?})", a, b)).collect();
        //assert_eq!(values.join(","), "(TestA(1),TestB(2))");
    }

    #[test]
    fn eval() {
        let mut table = Table::new();
        let row_id = table.push_column(TestA(1)).row_id();



    }
    #[test]
    fn test_table() {
        let mut table = TestTable::new();
        table.push(TestA(1));
        table.push((TestA(2),TestB(3)));

        let mut values : Vec<String> = table.query::<&TestA>()
            .map(|t: &TestA| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1),TestA(2)");

        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(2)");
    }

    #[derive(Debug)]
    struct TestA(u32);

    #[derive(Debug)]
    struct TestB(u16);

    #[derive(Debug)]
    struct TestC(u32);

    trait TestComponent:'static {}
    
    impl TestComponent for TestA {}
    impl TestComponent for TestB {}
    
    struct IsTest;
    struct IsTestC;

    impl<T:TestComponent> Insert for T {
        fn build(builder: &mut InsertBuilder) {
            builder.add_column::<T>()
        }

        unsafe fn insert(cursor: &mut InsertCursor, value: Self) {
            cursor.insert(value);
        }
    }

    impl Insert for TestC {
        fn build(builder: &mut InsertBuilder) {
            builder.add_column::<TestC>()
        }

        unsafe fn insert(cursor: &mut InsertCursor, value: Self) {
            cursor.insert(value);
        }
    }

    impl<T:TestComponent> Query for &T {
        type Item<'t> = &'t T;

        fn build(query: &mut super::QueryBuilder) {
            query.add_ref::<T>()
        }
    
        unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { // <'a> {
            cursor.deref::<T>()
        }
    }

    impl<T:TestComponent> Query for &mut T {
        type Item<'t> = &'t mut T;

        fn build(query: &mut super::QueryBuilder) {
            query.add_ref::<T>()
        }
    
        unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { // <'a> {
            cursor.deref_mut::<T>()
        }
    }

    struct TestTable<'t> {
        table: Table<'t>,
    }

    impl<'t> TestTable<'t> {
        fn new() -> Self {
            Self {
                table: Table::new(),
            }
        }

        fn push<T:Insert>(&mut self, value: T)
        {
             self.table.push::<T>(value);
        }

        fn query<'a,T>(&mut self) -> QueryIterator<T>
        //where T:Query<IsTest,Item<'a>=T>
        where T:Query<Item<'t>=T> // <'a>=T>
        {
            self.table.query()
        }
    }
}