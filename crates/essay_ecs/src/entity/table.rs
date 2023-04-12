use super::column::{Column, RowId};
use super::insert::{InsertBuilder, Insert, InsertPlan};
use super::query::{Query, QueryIterator, QueryBuilder, QueryPlan};
use super::meta::{RowTypeId, RowMetas, ColumnTypeId, EntityTypeId};

pub struct Table<'t> {
    meta: RowMetas,

    columns: Vec<Column<'t>>,

    type_rows: Vec<Vec<RowId>>,

    entities: Vec<EntityRow>,
    type_entities: Vec<Vec<EntityId>>,
}

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityId(usize);

pub struct EntityRow {
    id: EntityId,
    type_id: EntityTypeId,

    columns: Vec<RowId>,
}

pub struct RowRef {
    row: RowId,
    type_id: RowTypeId,
}

pub trait Component:'static {}

#[derive (Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComponentId(usize);

//
// implementation
//

impl<'t> Table<'t> {
    pub fn new() -> Self {
        let mut row_meta = RowMetas::new();

        Self {
            meta: row_meta,

            columns: Vec::new(),

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
    
    pub(crate) fn add_column<T:'static>(&mut self) -> ColumnTypeId {
        let column_id = self.meta.add_column::<T>();

        if column_id.index() < self.columns.len() {
            return column_id;
        }

        assert_eq!(column_id.index(), self.columns.len());

        self.columns.push(Column::new::<T>(&mut self.meta));
        
        column_id
    }

    pub(crate) fn get_column(&mut self, column_id: ColumnTypeId) -> &mut Column<'t> {
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

    pub(crate) fn add_entity_type(&mut self, cols: Vec<ColumnTypeId>) -> EntityTypeId {
        let entity_type_id = self.meta.add_entity_row(cols);

        while self.type_entities.len() <= entity_type_id.index() {
            self.type_entities.push(Vec::new());
        }
        
        entity_type_id
    }

    pub(crate) fn push_entity_row(
        &mut self, 
        entity_type_id: EntityTypeId, 
        rows: Vec<RowId>
    ) {
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

    pub(crate) unsafe fn deref<T:'static>(
        &self, 
        column_id: ColumnTypeId, 
        row_id: RowId
    ) -> Option<&'t T> {
        self.columns[column_id.index()].get(row_id)
    }

    pub(crate) unsafe fn deref_mut<T:'static>(
        &self, 
        column_id: ColumnTypeId, 
        row_id: RowId
    ) -> Option<&'t mut T> {
        self.columns[column_id.index()].get_mut(row_id)
    }

    pub(crate) fn get_row_by_type_index(
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

impl EntityId {
    fn index(&self) -> usize {
        self.0
    }
}

impl EntityRow {
    pub(crate) fn get_column(&self, index: usize) -> RowId {
        self.columns[index]
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
    use crate::{entity::{meta::{}, insert::InsertCursor}, prelude::Component};

    use super::{Table, Query, QueryIterator, InsertBuilder, Insert};

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

        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(2)");

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
    
    impl Component for TestA {}
    impl Component for TestB {}
    
    struct IsTest;
    struct IsTestC;

    impl Insert for TestC {
        fn build(builder: &mut InsertBuilder) {
            builder.add_column::<TestC>()
        }

        unsafe fn insert(cursor: &mut InsertCursor, value: Self) {
            cursor.insert(value);
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