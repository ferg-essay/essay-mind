use std::marker::PhantomData;

use super::column::Column;
use super::prelude::{ViewTypeId, QueryCursor2};
use super::row::{RowId, Row};
use super::meta::{RowTypeId, RowMetas, ColumnTypeId, EntityTypeId, EntityGroup};

pub struct Table<'t> {
    meta: RowMetas,

    columns: Vec<Column<'t>>,

    rows: Vec<Row<'t>>,

    type_rows: Vec<Vec<RowId>>,

    entities: Vec<EntityRow>,
    type_entities: Vec<Vec<EntityId>>,
}

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityId(usize);

impl EntityId {
    fn index(&self) -> usize {
        self.0
    }
}

pub struct EntityRow {
    id: EntityId,
    type_id: EntityTypeId,

    columns: Vec<RowId>,
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

        Self {
            meta: row_meta,

            columns: Vec::new(),

            rows: Vec::new(),

            type_rows: Vec::new(),
            
            entities: Vec::new(),
            type_entities: Vec::new(),
        }
    }

    pub(crate) fn meta(&self) -> &RowMetas {
        &self.meta
    }

    pub(crate) fn meta_mut(&mut self) -> &mut RowMetas {
        &mut self.meta
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
    
    fn add_column<T:'static>(&mut self) -> ColumnTypeId {
        let column_id = self.meta.add_column::<T>();

        if column_id.index() < self.columns.len() {
            return column_id;
        }

        assert_eq!(column_id.index(), self.columns.len());

        self.columns.push(Column::new::<T>(&mut self.meta));
        
        column_id
    }

    fn get_column(&mut self, column_id: ColumnTypeId) -> &mut Column<'t> {
        &mut self.columns[column_id.index()]
    }

    pub fn push_column<T:'static>(&mut self, value: T) {
        let mut builder = InsertBuilder::new(self);

        builder.add_column::<T>();

        let plan = builder.build();

        let mut cursor = plan.cursor(self);
        unsafe {
            cursor.insert(value);
        }
        cursor.complete();
    }

    pub fn push<T:Insert>(&mut self, value: T) {
        let plan = self.insert_plan::<T>();

        let mut cursor = plan.cursor(self);
        unsafe {
            T::insert(&mut cursor, value);
        }
        cursor.complete();
    }

    fn add_entity_type(&mut self, cols: Vec<ColumnTypeId>) -> EntityTypeId {
        let entity_type_id = self.meta.add_entity_row(cols);

        while self.type_entities.len() <= entity_type_id.index() {
            self.type_entities.push(Vec::new());
        }
        
        entity_type_id
    }

    fn push_entity_row(&mut self, entity_type_id: EntityTypeId, rows: Vec<RowId>) {
        let entity_id = EntityId(self.entities.len());

        let entity_row = EntityRow {
            id: entity_id,
            type_id: entity_type_id,
            columns: rows,
        };

        self.entities.push(entity_row);
        
        self.type_entities[entity_type_id.index()].push(entity_id);
    }

    fn insert_plan<T:Insert>(&mut self) -> InsertPlan {
        let mut builder = InsertBuilder::new(self);

        T::build(&mut builder);

        builder.build()
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

    unsafe fn deref<T:'static>(
        &self, 
        column_id: ColumnTypeId, 
        row_id: RowId
    ) -> Option<&'t T> {
        self.columns[column_id.index()].get(row_id)
    }

    unsafe fn deref_mut<T:'static>(
        &self, 
        column_id: ColumnTypeId, 
        row_id: RowId
    ) -> Option<&'t mut T> {
        self.columns[column_id.index()].get_mut(row_id)
    }

    fn get_row_by_type_index(
        &self, 
        row_type_id: EntityTypeId, 
        row_index: usize
    ) -> Option<&EntityRow> {
        match self.type_entities[row_type_id.index()].get(row_index) {
            Some(row_id) => self.entities.get(row_id.index()),
            None => None,
        }
    }
}

pub trait Insert:'static {
    fn build(builder: &mut InsertBuilder);

    unsafe fn insert(cursor: &mut InsertCursor, value: Self);
}

pub struct InsertBuilder<'a,'t> {
    table: &'a mut Table<'t>,
    columns: Vec<ColumnTypeId>,
}

pub struct InsertPlan {
    entity_type: EntityTypeId,
    columns: Vec<ColumnTypeId>,
}

pub struct InsertCursor<'a, 't> {
    table: &'a mut Table<'t>,
    plan: &'a InsertPlan,
    index: usize,
    rows: Vec<RowId>,
}

impl<'a,'t> InsertBuilder<'a,'t> {
    pub(crate) fn new(table: &'a mut Table<'t>) -> Self {
        Self {
            table: table,
            columns: Vec::new(),
        }
    }

    pub(crate) fn add_column<T:'static>(&mut self) {
        let id = self.table.add_column::<T>();
        
        self.columns.push(id);
    }

    pub(crate) fn build(self) -> InsertPlan {
        let entity_id = self.table.add_entity_type(self.columns.clone());

        InsertPlan {
            entity_type: entity_id,
            columns: self.columns.clone(),
        }
    }
}

impl InsertPlan {
    pub(crate) fn insert<T:'static>(
        &self,
        table: &mut Table, 
        index: usize, 
        value: T
    ) -> RowId {
        unsafe {
            let column_id = self.columns[index];
            table.get_column(column_id).push(value)
        }
    }

    pub(crate) fn cursor<'a, 't>(&'a self, table: &'a mut Table<'t>) -> InsertCursor<'a, 't> {
        InsertCursor {
            plan: &self,
            table: table,
            index: 0, 
            rows: Vec::new(),
        }
    }
}

impl<'a, 't> InsertCursor<'a, 't> {
    pub unsafe fn insert<T:'static>(&mut self, value: T) {
        let index = self.index;
        self.index += 1;

        let row_id = self.plan.insert(self.table, index, value);

        self.rows.push(row_id);
    }

    fn complete(self) {
        self.table.push_entity_row(self.plan.entity_type, self.rows)
    }
}

pub trait Query {
    type Item<'a>;

    fn build(query: &mut QueryBuilder);

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t>;
}

pub struct QueryCursor<'a,'t> {
    table: &'a Table<'t>,
    entity_group: &'a EntityGroup,
    row: &'a EntityRow,
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
        table: &'a Table<'t>,
        group: &'a EntityGroup,
        row: &'a EntityRow
    ) -> QueryCursor<'a,'t> {
        QueryCursor {
            table: table,
            entity_group: group,
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
        let index = self.cols[self.index];
        self.index += 1;

        let column_id = self.entity_group.columns()[index];
        let row_id = self.row.columns[index];

        self.table.deref::<T>(column_id, row_id).unwrap()
    }

    pub unsafe fn deref_mut<T:'static>(&mut self) -> &'t mut T {
        let index = self.cols[self.index];
        self.index += 1;

        let column_id = self.entity_group.columns()[index];
        let row_id = self.row.columns[index];

        self.table.deref_mut(column_id, row_id).unwrap()
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

    entity_index: usize,

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
            entity_index: 0,

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
            let entity_type = self.table.meta().get_entity_type(row_type_id);
            let entity_index = self.entity_index;
            self.entity_index += 1;

            match self.table.get_row_by_type_index(row_type_id, entity_index) {
                Some(row) => {
                    return unsafe { 
                        let mut cursor = self.query.new_cursor(
                            self.table,
                            entity_type, 
                            row
                        );
                        
                        Some(T::query(&mut cursor))
                    }
                }
                None => {},
            };

            self.view_type_index += 1;
            self.entity_index = 0;
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

        values = table.query::<&TestB>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000)");

        values = table.query::<&TestA>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.push_column(TestB(100));
        assert_eq!(table.len(), 3);

        values = table.query::<&TestA>().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        for entity in table.query::<&mut TestB>() {
            entity.0 += 1;
        }
        
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

        //values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        //assert_eq!(values.join(","), "TestB(2)");

        values = table.query::<(&TestA,&TestB)>().map(|v| format!("{:?}", v)).collect();
        assert_eq!(values.join(","), "(TestA(1), TestB(2))");
    }

    #[test]
    fn eval() {
        let mut table = Table::new();
        //let row_id = table.push_column(TestA(1)).row_id();

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
        assert_eq!(values.join(","), "TestB(3)");
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