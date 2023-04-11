use std::{marker::PhantomData, any::{TypeId, type_name}};

use crate::store::{prelude::{Table, RowId, Row, Query, QueryIterator}, 
    row_meta::{ViewRowTypeId, ViewRowType, ViewTypeId, InsertPlan, InsertBuilder, Insert}, 
    row_meta::{ColumnTypeId, RowTypeId}};

use super::{prelude::EntityRef};

pub struct EntityTable<'w> {
    table: Table<'w,IsEntity>,
}

pub struct IsEntity;
//type InsertEntity = Insert<IsEntity>;

impl<'t> EntityTable<'t> {
    pub fn new() -> Self {
        EntityTable {
            table: Table::new(),
  //          entity_meta: EntityMeta::new(),
        }
    }

    pub fn push<T:Insert<IsEntity>>(&mut self, value: T) -> EntityRef {
        let row_ref = self.table.push(value);

        EntityRef::new(
            row_ref.row_id(),
            row_ref.row_type_id(),
        )
    }

    pub(crate) fn add_insert_map<M,T:Insert<M>>(&mut self) -> InsertPlan {
        todo!();
        /*
        let mut cols = InsertMapBuilder::new();

        T::build(&mut cols);

        let row_type_id = self.table.meta_mut().add_row(cols.columns().clone());
        let row_type = self.table.meta_mut().get_row_id(row_type_id);

        cols.build_insert(row_type)
        */
    }
    
    pub(crate) fn add_entity_type<M,T:Insert<M>>(&mut self) -> ViewTypeId {
        todo!();
        /*
        let mut cols = InsertMapBuilder::new();

        T::build(&mut cols);

        self.entity_type(cols.columns().clone())
        */
    }
    
    /*
    pub(crate) fn add_entity_type_cols(&mut self, e_cols: impl EntityCols) -> EntityTypeId {
        let mut cols : Vec<ColumnTypeId> = Vec::new();

        //let e_cols : dyn EntityCols = T;
        e_cols.add_cols(self, &mut cols);

        //println!("Add-entity {:?}", type_name::<T>());
        self.entity_type(cols)
    }
    */

    pub(crate) fn add_column<T:'static>(&mut self) -> ColumnTypeId {
        self.table.meta_mut().add_column::<T>()
    }

    pub fn entity_type(&mut self, cols: Vec<ColumnTypeId>) -> ViewTypeId {
        self.table.meta_mut().add_view(cols)
    }

    pub fn entity_row_type(
        &mut self, 
        row_id: RowTypeId, 
        entity_id: ViewTypeId
    ) -> ViewRowTypeId {
        let row_type = self.table.meta().get_row_id(row_id);

        todo!()
        //self.entity_meta.entity_row(row_type, entity_id)
    }

    pub fn entity_row_by_type<T:'static>(&mut self, row_id: RowTypeId) -> ViewRowTypeId {
        let entity_id = self.table.meta_mut().single_view_type::<T>();

        self.table.meta_mut().add_view_row(row_id, entity_id)
    }

    pub(crate) fn push_entity_type(
        &mut self, 
        entity_row_type: ViewRowTypeId, 
        col_type_id: ColumnTypeId
    ) -> ViewRowTypeId {
        todo!();
        /*
        let entity_row = self.entity_meta
            .get_entity_row(entity_row_type);

        let row_type_id = self.table.push_row_type(
            entity_row.row_type_id(),
            col_type_id
        );

        let row_type = self.table.get_row_type(row_type_id);

        let entity_id = self.entity_meta.push_entity_type(
            entity_row.entity_type_id(),
            col_type_id
        );

        self.entity_meta.entity_row(row_type, entity_id)
         */
    }

    pub(crate) fn get_row<T:'static>(&self, row_id: RowId) -> &T {
        match self.table.meta().get_single_view_type::<T>() {
            Some(view_type) => { 
                todo!()
            },
            None => todo!(),
        }
    }

    fn iter_type<T:'static>(&self) -> &T {
        todo!()
    }

    pub(crate) fn len(&self) -> usize {
        self.table.len()
    }

    pub(crate) fn iter_by_type<T:'static>(&self) -> Entity3Iterator<T> {
        match self.table.meta().get_single_view_type::<T>() {
            Some(entity_type) => { 
                Entity3Iterator {
                    table: self,
                    entity_type: entity_type,
                    entity_type_index: 0,
                    row_index: 0,
                    marker: PhantomData,
                }
            },
            None => todo!(),
        }
    }

    pub(crate) fn iter_mut_by_type<M,T:Insert<M>>(&mut self) -> Entity3MutIterator<T> {
        todo!()
        /*
        let entity_type = self.add_entity_type::<T>();

        Entity3MutIterator {
            table: self,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
            marker: PhantomData,
        }
        */
    }

    pub(crate) fn get_single_entity_type<T:'static>(&self) -> Option<ViewTypeId> {
        todo!();
        /*
        match self.table.get_column_type_id::<T>() {
            Some(component_id) => { 
                let cols = Vec::from([component_id]);

                self.entity_meta.get_entity_type_cols(&cols)
            },
            None => None,
        }
         */
    }

    pub(crate) fn query<T:Query<IsEntity,Item<'t>=T>>(&mut self) -> QueryIterator<IsEntity,T> {
        self.table.query::<T>()
    }

    /*
    pub(crate) fn iter_mut_by_type<T:'static>(&mut self) -> Entity2MutIterator<T> {
        match self.get_single_entity_type::<T>() {
            Some(entity_type) => {
                Entity2MutIterator::new(self, entity_type)
            },
            None => todo!(),
        } 
    }
     */

    /*
    fn get_row(&self, row_id: RowId) -> Option<&'t Row> {
        self.table.get_row(row_id)
    }
     */
}

struct EntityCursor<'a, 't> {
    table: &'a EntityTable<'t>,
    entity_type: ViewTypeId,
    entity_type_index: usize,
    row_index: usize,
}

impl<'a, 't> EntityCursor<'a, 't> {
    fn new(table: &'a EntityTable<'t>, entity_type: ViewTypeId) -> Self {
        Self {
            table: table,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
        }
    }

    fn next(&mut self) -> Option<&Row<'a>> {
        let entity = self.table.table.meta().get_view(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;
            /*
            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => return Some(row),
                None => {},
            };
            */

            self.entity_type_index += 1;
            self.row_index = 0;
        }

        None
    }

    fn next_mut<T:'static>(&self) -> Option<&'a mut T> {
        todo!();
        /*
        let type_rows = self.table.entity_meta.get_entity_rows(self.entity_type);

        if type_rows.len() <= self.entity_type_index {
            return None;
        }

        todo!()
        */
    }
}

pub struct Entity3Iterator<'a, 't, T> {
    table: &'a EntityTable<'t>,
    entity_type: ViewTypeId,
    entity_type_index: usize,
    row_index: usize,
    marker: PhantomData<T>,
}

impl<'a, 't, T> Entity3Iterator<'a, 't, T> {
    fn new(table: &'a EntityTable<'t>, entity_type: ViewTypeId) -> Self {
        Self {
            table: table,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
            marker: PhantomData,
        }
    }

    fn next(&mut self) -> Option<&Row<'a>> {
        let entity = self.table.table.meta().get_view(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;
            /*
            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => return Some(row),
                None => {},
            };
            */

            self.entity_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

impl<'a, 't, T:'static> Iterator for Entity3Iterator<'a, 't, T> {
    type Item=&'a T;

    fn next(&mut self) -> Option<&'a T> {
        let entity = self.table.table.meta().get_view(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let entity_row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;

            let entity_row = self.table.table.meta().get_view_row(entity_row_type_id);
            let row_type_id = entity_row.row_type_id();

            todo!();
            /*
            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
                    return unsafe {
                        Some(row.get(entity_row.columns()[0]))
                    } 
                }
                None => {},
            };
            */

            self.entity_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

pub struct Entity3MutIterator<'a, 't, T> {
    table: &'a EntityTable<'t>,
    entity_type: ViewTypeId,
    entity_type_index: usize,
    row_index: usize,
    marker: PhantomData<T>,
}

impl<'a, 't, T> Entity3MutIterator<'a, 't, T> {
    fn new(table: &'a EntityTable<'t>, entity_type: ViewTypeId) -> Self {
        Self {
            table: table,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:'static> Iterator for Entity3MutIterator<'a, 't, T> {
    type Item=&'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        let entity = self.table.table.meta().get_view(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let entity_row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;

            let entity_row = self.table.table.meta().get_view_row(entity_row_type_id);
            let row_type_id = entity_row.row_type_id();

            todo!();
            /*
            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
                    println!("iter-row {:?} {:?}", entity_row.columns()[0], type_name::<T>());
                    return unsafe {
                        Some(row.get_mut(entity_row.columns()[0]))
                    } 
                }
                None => {},
            };
            */

            self.entity_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

pub struct Entity2Iterator<'a, 't, T> {
    cursor: EntityCursor<'a, 't>,
    marker: PhantomData<T>,
}

impl<'a, 't, T> Entity2Iterator<'a, 't, T> {
    pub fn new(table: &'a EntityTable<'t>, entity_type: ViewTypeId) -> Self {
        Self {
            cursor: EntityCursor::new(table, entity_type),
            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:'static> Iterator for Entity2Iterator<'a, 't, T> {
    type Item=&'a T;

    fn next(&mut self) -> Option<&'t T> {
        match self.cursor.next() {
            Some(row) => { 
                //row.ptr(index).deref()
                None
            },
            None => None,
        }
    }
}

pub struct Entity2MutIterator<'a, 't, T> {
    cursor: EntityCursor<'a, 't>,
    marker: PhantomData<T>,
}

impl<'a, 't, T> Entity2MutIterator<'a, 't, T> {
    pub fn new(table: &'a mut EntityTable<'t>, entity_type: ViewTypeId) -> Self {
        Self {
            cursor: EntityCursor::new(table, entity_type),
            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:'static> Iterator for Entity2MutIterator<'a, 't, T> {
    type Item=&'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use essay_ecs_macros::Component;

    use crate::store::row_meta::ColumnTypeId;

    use super::EntityTable;

    #[test]
    fn test_entity_ref() {
        let mut table = EntityTable::new();

        /*
        let ref_a = table.push(TestA(1));
        let value_a = ref_a.get(&table).unwrap();
        assert_eq!(value_a, &TestA(1));

        let ref_b = table.push(TestB(2));
        let value_b = ref_b.get(&table).unwrap();
        assert_eq!(value_b, &TestB(2));

        let ref_a3 = table.push(TestA(3));
        let value_a = ref_a3.get(&table).unwrap();
        assert_eq!(value_a, &TestA(3));

        let value_a = ref_a.get(&table).unwrap();
        assert_eq!(value_a, &TestA(1));
        */
    }

    #[test]
    fn test_entity_iter() {
        /*
        let mut table = EntityTable::new();

        table.push(TestA(1));

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(1)]));

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(1)]));

        table.push(TestA(2));

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(1), TestA(2)]));

        table.push(TestA(0));

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(1), TestA(2), TestA(0)]));
        */
    }

    #[test]
    fn test_entity_mut_iter() {
        let mut table = EntityTable::new();

        /*
        table.push(TestA(0));
        

        //let rows : Vec<TestA> = table.iter_mut_by_type::<TestA>().cloned().collect();
        for test in table.iter_mut_by_type::<TestA>() {
            test.0 += 1;
        }
        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(1)]));

        for test in table.iter_mut_by_type::<TestA>() {
            test.0 += 1;
        }

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(2)]));

        table.push(TestA(0));

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(2), TestA(0)]));

        for test in table.iter_mut_by_type::<TestA>() {
            test.0 += 1;
        }

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(3), TestA(1)]));

        table.push(TestA(0));

        let rows : Vec<TestA> = table.iter_by_type::<TestA>().cloned().collect();
        assert_eq!(rows, Vec::from([TestA(3), TestA(1), TestA(0)]));
        */
    }

    #[test]
    fn test_entity_push() {
        let mut table = EntityTable::new();

        /*
        let ref_a = table.push(TestA(1));
        let value_a = ref_a.get(&table).unwrap();
        assert_eq!(value_a, &TestA(1));

        let ref_b = ref_a.push(&mut table, TestB(2));

        assert_eq!(ref_a.get(&table).unwrap(), &TestA(1));

        let ref_a2 = table.push(TestA(3));
        let ref_a3 = table.push(TestA(4));
        */
    }

    #[test]
    fn test_entity_fun() {
        /*
        let mut table = EntityTable::new();

        let ref_a = table.push(TestA(1));
        ref_a.push(&mut table, TestB(2));

        let ref_a2 = table.push(TestA(3));
        let ref_b3 = table.push(TestB(4));

        let col_a = table.table.meta_mut().add_column::<TestA>();
        let col_b = table.table.meta_mut().add_column::<TestA>();

        let mut cols_a = Vec::<ColumnTypeId>::new();
        cols_a.push(col_a);

        let type_a = table.entity_type(cols_a);

        let mut cols_b = Vec::<ColumnTypeId>::new();
        cols_b.push(col_b);
        
        let type_b = table.entity_type(cols_b);
        */

        //let ert_a = table.entity_row_type(ref_a.row_type, type_a);
        // let ert_a = table.entity_meta.get_entity_row(ert_a);

        /*
        unsafe {
            let value = ert_a.get_fun(&table.table, ref_a2.row_id, |row, map| {
                row.ptr(map[0]).deref::<TestA>()
            });
            assert_eq!(value, &TestA(3));
        }
         */
        /*
        for item in table.iter_type::<TestA>() {

        }
         */
    }

    #[derive(Component, Debug, Clone, PartialEq)]
    struct TestA(usize);

    #[derive(Component, Debug, Clone, PartialEq)]
    struct TestB(usize);
}
