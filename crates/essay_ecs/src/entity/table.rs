use super::column::{Column, RowId};
use super::insert::{InsertBuilder, Insert, InsertPlan};
use super::prelude::ViewId;
use super::view::{View, ViewIterator, ViewBuilder, ViewPlan};
use super::meta::{TableMeta, ColumnId, RowTypeId, ViewType};

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityId(usize);

pub struct Table {
    meta: TableMeta,

    columns: Vec<Column>,

    rows: Vec<Row>,
    rows_by_type: Vec<Vec<EntityId>>,
}

pub struct Row {
    id: EntityId,
    row_type: RowTypeId,

    columns: Vec<RowId>,
}

pub trait Component:'static {}

#[derive (Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComponentId(usize);

//
// implementation
//

impl Table {
    pub fn new() -> Self {
        let mut row_meta = TableMeta::new();

        Self {
            meta: row_meta,

            columns: Vec::new(),
            
            rows: Vec::new(),
            rows_by_type: Vec::new(),
        }
    }

    pub(crate) fn meta(&self) -> &TableMeta {
        &self.meta
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    //
    // Column
    //

    pub(crate) fn column_mut(&mut self, column_id: ColumnId) -> &mut Column {
        &mut self.columns[column_id.index()]
    }
    
    pub(crate) fn add_column<T:'static>(&mut self) -> ColumnId {
        let column_id = self.meta.add_column::<T>();

        if column_id.index() < self.columns.len() {
            return column_id;
        }

        assert_eq!(column_id.index(), self.columns.len());

        self.columns.push(Column::new::<T>(&mut self.meta));
        
        column_id
    }

    //
    // row (entity)
    //

    pub fn spawn<T:Insert>(&mut self, value: T) -> EntityId {
        let plan = self.insert_plan::<T>();

        self.spawn_with_plan(plan, value)
    }

    pub fn get<T:'static>(&mut self, entity: EntityId) -> Option<&T> {
        let column_id = self.meta().get_column::<T>()?;
        let row = self.rows.get(entity.index())?;
        let row_type = self.meta().row(row.row_type);

        let index = row_type.position(column_id)?;

        unsafe {
            self.get_by_id(column_id, row.columns[index])
        }
    }

    pub fn get_mut<T:'static>(&mut self, entity: EntityId) -> Option<&mut T> {
        let column_id = self.meta().get_column::<T>()?;
        let row = self.rows.get(entity.index())?;
        let row_type = self.meta().row(row.row_type);

        let index = row_type.position(column_id)?;

        unsafe {
            self.get_mut_by_id(column_id, row.columns[index])
        }
    }

    pub(crate) fn insert_plan<T:Insert>(&mut self) -> InsertPlan {
        let mut builder = InsertBuilder::new(self);

        T::build(&mut builder);

        builder.build()
    }

    pub(crate) fn spawn_with_plan<T:Insert>(
        &mut self, 
        plan: InsertPlan, 
        value: T
    ) -> EntityId {
        let mut cursor = plan.cursor(self);

        unsafe {
            T::insert(&mut cursor, value);
        }
        cursor.complete()
    }

    pub(crate) fn add_row_type(&mut self, cols: Vec<ColumnId>) -> RowTypeId {
        let row_type_id = self.meta.add_row(cols);

        while self.rows_by_type.len() <= row_type_id.index() {
            self.rows_by_type.push(Vec::new());
        }
        
        row_type_id
    }

    pub(crate) fn push_row(
        &mut self, 
        row_type_id: RowTypeId, 
        columns: Vec<RowId>
    ) -> EntityId {
        let entity_id = EntityId(self.rows.len());

        let row = Row {
            id: entity_id,
            row_type: row_type_id,
            columns,
        };

        self.rows.push(row);
        
        self.rows_by_type[row_type_id.index()].push(entity_id);

        entity_id
    }

    //
    // View
    //

    pub fn iter_view<'a,T:View>(&mut self) -> ViewIterator<'_,T> {
        let plan = self.get_view_plan::<T>();
        
        unsafe { self.iter_view_with_plan(plan) }
    }

    pub(crate) fn get_view_plan<T:View>(&mut self) -> ViewPlan {
        let mut builder = ViewBuilder::new(self);

        T::build(&mut builder);

        builder.build()
    }

    pub(crate) unsafe fn iter_view_with_plan<T:View>(
        &self, 
        plan: ViewPlan
    ) -> ViewIterator<'_,T> {
        ViewIterator::new(self, plan)
    }

    pub(crate) fn view(&self, view_id: ViewId) -> &ViewType {
        self.meta.view(view_id)
    }

    pub(crate) fn add_view(&mut self, columns: &Vec<ColumnId>) -> ViewId {
        self.meta.add_view(columns)
    }

    pub(crate) unsafe fn get_by_id<T:'static>(
        &self, 
        column_id: ColumnId, 
        row_id: RowId
    ) -> Option<&T> {
        self.columns[column_id.index()].get(row_id)
    }

    pub(crate) unsafe fn get_mut_by_id<T:'static>(
        &self, 
        column_id: ColumnId, 
        row_id: RowId
    ) -> Option<&mut T> {
        self.columns[column_id.index()].get_mut(row_id)
    }

    pub(crate) fn get_row_by_type_index(
        &self, 
        row_type_id: RowTypeId, 
        row_index: usize
    ) -> Option<&Row> {
        match self.rows_by_type[row_type_id.index()].get(row_index) {
            Some(row_id) => self.rows.get(row_id.index()),
            None => None,
        }
    }
}

impl EntityId {
    fn index(&self) -> usize {
        self.0
    }
}

impl Row {
    pub(crate) fn column_row(&self, index: usize) -> RowId {
        self.columns[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{entity::{insert::InsertCursor}, prelude::Component};

    use super::{Table, View, ViewIterator, InsertBuilder, Insert};

    #[test]
    fn spawn() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.spawn(TestA(1));
        assert_eq!(table.len(), 1);

        let mut values : Vec<String> = table.iter_view::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.spawn(TestB(10000));
        assert_eq!(table.len(), 2);

        values = table.iter_view::<&TestB>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000)");

        values = table.iter_view::<&TestA>().map(|t| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        table.spawn(TestB(100));
        assert_eq!(table.len(), 3);

        values = table.iter_view::<&TestA>().map(|t: &TestA| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestA(1)");

        values = table.iter_view::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10000),TestB(100)");

        for entity in table.iter_view::<&mut TestB>() {
            entity.0 += 1;
        }
        
        values = table.iter_view::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(10001),TestB(101)");
    }

    #[test]
    fn entity_get() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        let id_0 = table.spawn(TestA(1000));
        assert_eq!(table.len(), 1);
        assert_eq!(id_0.index(), 0);

        assert_eq!(table.get::<TestA>(id_0), Some(&TestA(1000)));
        assert_eq!(table.get::<TestB>(id_0), None);

        let id_1 = table.spawn(TestB(1001));
        assert_eq!(table.len(), 2);
        assert_eq!(id_1.index(), 1);

        assert_eq!(table.get::<TestA>(id_1), None);
        assert_eq!(table.get::<TestB>(id_1), Some(&TestB(1001)));

        let id_2 = table.spawn(TestA(1002));
        assert_eq!(table.len(), 3);
        assert_eq!(id_2.index(), 2);

        assert_eq!(table.get::<TestA>(id_2), Some(&TestA(1002)));
        assert_eq!(table.get::<TestB>(id_2), None);

        assert_eq!(table.get::<TestA>(id_0), Some(&TestA(1000)));
        assert_eq!(table.get::<TestB>(id_0), None);
    }

    #[test]
    fn push_type() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.spawn::<TestA>(TestA(1));
        //table.push(TestC(1));
        assert_eq!(table.len(), 1);

        let values : Vec<String> = table.iter_view::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1)");
    }

    #[test]
    fn push_tuple() {
        let mut table = Table::new();
        assert_eq!(table.len(), 0);

        table.spawn((TestA(1),TestB(2)));
        table.spawn((TestB(3),TestA(4)));
        
        assert_eq!(table.len(), 2);

        let mut values : Vec<String> = table.iter_view::<&TestA>()
            .map(|t| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1),TestA(4)");

        values = table.iter_view::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(2),TestB(3)");

        values = table.iter_view::<(&TestA,&TestB)>().map(|v| format!("{:?}", v)).collect();
        assert_eq!(values.join(","), "(TestA(1), TestB(2)),(TestA(4), TestB(3))");
    }

    #[test]
    fn eval() {
        //let mut table = Table::new();
        //let row_id = table.push_column(TestA(1)).row_id();

    }

    /*
    #[test]
    fn test_table() {
        let mut table = TestTable::new();
        table.push(TestA(1));
        table.push((TestA(2),TestB(3)));
        table.push((TestB(4),TestA(5)));
        table.push(TestB(6));

        let mut values : Vec<String> = table.query::<&TestA>()
            .map(|t: &TestA| format!("{:?}", t))
            .collect();
        assert_eq!(values.join(","), "TestA(1),TestA(2),TestA(5)");

        values = table.query::<&TestB>().map(|t: &TestB| format!("{:?}", t)).collect();
        assert_eq!(values.join(","), "TestB(3),TestB(4),TestB(6)");
    }
    */

    #[derive(Debug, PartialEq)]
    struct TestA(u32);

    #[derive(Debug, PartialEq)]
    struct TestB(u16);

    #[derive(Debug, PartialEq)]
    struct TestC(u32);

    trait TestComponent:'static {}
    
    impl Component for TestA {}
    impl Component for TestB {}
    
    impl Insert for TestC {
        fn build(builder: &mut InsertBuilder) {
            builder.add_column::<TestC>()
        }

        unsafe fn insert(cursor: &mut InsertCursor, value: Self) {
            cursor.insert(value);
        }
    }

    /*
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
             self.table.spawn::<T>(value);
        }

        fn query<'a,T>(&mut self) -> ViewIterator<T>
        //where T:Query<IsTest,Item<'a>=T>
        where T:View<Item<'t>=T> // <'a>=T>
        {
            self.table.iter_view()
        }
    }
    */
}