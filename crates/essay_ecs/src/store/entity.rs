use std::{marker::PhantomData, any::{TypeId, type_name}};

use super::{prelude::Table, row::{RowId, Row}, row_meta::{EntityRowTypeId, EntityRowType, EntityTypeId}, row_meta::{ColumnTypeId, RowTypeId}};

pub struct EntityTable<'w> {
    table: Table<'w>,
//    entity_meta: EntityMeta,
}

pub struct EntityRef<T> {
    row_id: RowId,
    row_type: RowTypeId,
    entity_row_type: EntityRowTypeId,
    marker: PhantomData<T>,
}

pub trait Component {}

impl<'t> EntityTable<'t> {
    pub fn new() -> Self {
        EntityTable {
            table: Table::new(),
  //          entity_meta: EntityMeta::new(),
        }
    }

    pub fn push<T:'static>(&mut self, value: T) -> EntityRef<T> {
        let row_ref = self.table.push(value);

        let type_id = self.entity_row_by_type::<T>(row_ref.row_type_id());

        EntityRef {
            row_id: row_ref.row_id(),
            row_type: row_ref.row_type_id(),
            entity_row_type: type_id,
            marker: PhantomData,
        }        
    }

    pub(crate) fn add_entity_type<T:EntityCols>(&mut self) -> EntityTypeId {
        let mut cols : Vec<ColumnTypeId> = Vec::new();

        T::add_cols(self, &mut cols);

        self.entity_type(cols)
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
        self.table.row_meta_mut().add_column::<T>().id()
    }

    pub fn entity_type(&mut self, cols: Vec<ColumnTypeId>) -> EntityTypeId {
        self.table.row_meta_mut().entity_type(cols)
    }

    pub fn entity_row_type(
        &mut self, 
        row_id: RowTypeId, 
        entity_id: EntityTypeId
    ) -> EntityRowTypeId {
        let row_type = self.table.get_row_type(row_id);

        todo!()
        //self.entity_meta.entity_row(row_type, entity_id)
    }

    pub fn entity_row_by_type<T:'static>(&mut self, row_id: RowTypeId) -> EntityRowTypeId {
        let entity_id = self.table.row_meta_mut().single_entity_type::<T>();

        self.table.row_meta_mut().add_entity_row(row_id, entity_id)
    }

    pub(crate) fn push_entity_type(
        &mut self, 
        entity_row_type: EntityRowTypeId, 
        col_type_id: ColumnTypeId
    ) -> EntityRowTypeId {
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

    fn iter_type<T:'static>(&self) -> &T {
        todo!()
    }

    pub(crate) fn len(&self) -> usize {
        self.table.len()
    }

    pub(crate) fn iter_by_type<T:'static>(&self) -> Entity3Iterator<T> {
        match self.table.row_meta().get_single_entity_type::<T>() {
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

    pub(crate) fn iter_mut_by_type<T:EntityCols+'static>(&mut self) -> Entity3MutIterator<T> {
        let entity_type = self.add_entity_type::<T>();

        Entity3MutIterator {
            table: self,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
            marker: PhantomData,
        }
    }

    pub(crate) fn get_single_entity_type<T:'static>(&self) -> Option<EntityTypeId> {
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
    entity_type: EntityTypeId,
    entity_type_index: usize,
    row_index: usize,
}

impl<'a, 't> EntityCursor<'a, 't> {
    fn new(table: &'a EntityTable<'t>, entity_type: EntityTypeId) -> Self {
        Self {
            table: table,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
        }
    }

    fn next(&mut self) -> Option<&Row<'a>> {
        let entity = self.table.table.row_meta().get_entity_type(self.entity_type);

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
    entity_type: EntityTypeId,
    entity_type_index: usize,
    row_index: usize,
    marker: PhantomData<T>,
}

impl<'a, 't, T> Entity3Iterator<'a, 't, T> {
    fn new(table: &'a EntityTable<'t>, entity_type: EntityTypeId) -> Self {
        Self {
            table: table,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
            marker: PhantomData,
        }
    }

    fn next(&mut self) -> Option<&Row<'a>> {
        let entity = self.table.table.row_meta().get_entity_type(self.entity_type);

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
        let entity = self.table.table.row_meta().get_entity_type(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let entity_row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;

            let entity_row = self.table.table.row_meta().get_entity_row(entity_row_type_id);
            let row_type_id = entity_row.row_type_id();

            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
                    return unsafe {
                        Some(row.ptr(entity_row.columns()[0]).deref())
                    } 
                }
                None => {},
            };

            self.entity_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

pub struct Entity3MutIterator<'a, 't, T> {
    table: &'a EntityTable<'t>,
    entity_type: EntityTypeId,
    entity_type_index: usize,
    row_index: usize,
    marker: PhantomData<T>,
}

impl<'a, 't, T> Entity3MutIterator<'a, 't, T> {
    fn new(table: &'a EntityTable<'t>, entity_type: EntityTypeId) -> Self {
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
        let entity = self.table.table.row_meta().get_entity_type(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let entity_row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;

            let entity_row = self.table.table.row_meta().get_entity_row(entity_row_type_id);
            let row_type_id = entity_row.row_type_id();

            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
                    return unsafe {
                        Some(row.ptr(entity_row.columns()[0]).deref_mut())
                    } 
                }
                None => {},
            };

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
    pub fn new(table: &'a EntityTable<'t>, entity_type: EntityTypeId) -> Self {
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
    pub fn new(table: &'a mut EntityTable<'t>, entity_type: EntityTypeId) -> Self {
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

impl<T:'static> EntityRef<T> {
    pub fn get<'a>(&self, table: &'a EntityTable) -> Option<&'a T> {
        todo!();
        /*
        let row_type = table.entity_meta.get_entity_row(self.entity_row_type);
            
        table.table.get_row(self.row_id, row_type.columns())
        */
    }

    pub fn push<S:'static>(&self, table: &mut EntityTable, value: S) {
        table.table.replace_push(self.row_id, value);
    }
}

pub trait EntityCols {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>);
}
/*
impl EntityCols for () {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>) {
    }
}
*/

impl<T:Component + 'static> EntityCols for T {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>) {
        cols.push(table.add_column::<T>());
    }
}
/*
impl<P1:'static,P2:'static> EntityCols for (P1,P2) {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>) {
        cols.push(table.add_column::<P1>());
        cols.push(table.add_column::<P2>());
    }
}
*/

//
// EntityCols composed of tuples
//

macro_rules! impl_entity_tuple {
    ($($param:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($param:'static),*> EntityCols for ($($param,)*)
        {
            fn add_cols(
                table: &mut EntityTable, 
                cols: &mut Vec<ColumnTypeId>
            ) {
                ($(cols.push(table.add_column::<$param>()),
                )*);
            }
        }
    }
}

impl_entity_tuple!();
impl_entity_tuple!(P1,P2);
impl_entity_tuple!(P1,P2,P3);
impl_entity_tuple!(P1,P2,P3,P4);
impl_entity_tuple!(P1,P2,P3,P4,P5);

#[cfg(test)]
mod tests {
    use essay_ecs_macros::Component;

    use crate::store::row_meta::ColumnTypeId;

    use super::EntityTable;

    #[test]
    fn test_entity_ref() {
        let mut table = EntityTable::new();

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
    }

    #[test]
    fn test_entity_iter() {
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
    }

    #[test]
    fn test_entity_mut_iter() {
        let mut table = EntityTable::new();

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
    }

    #[test]
    fn test_entity_push() {
        let mut table = EntityTable::new();

        let ref_a = table.push(TestA(1));
        let value_a = ref_a.get(&table).unwrap();
        assert_eq!(value_a, &TestA(1));

        let ref_b = ref_a.push(&mut table, TestB(2));

        assert_eq!(ref_a.get(&table).unwrap(), &TestA(1));

        let ref_a2 = table.push(TestA(3));
        let ref_a3 = table.push(TestA(4));
    }

    #[test]
    fn test_entity_fun() {
        let mut table = EntityTable::new();

        let ref_a = table.push(TestA(1));
        ref_a.push(&mut table, TestB(2));

        let ref_a2 = table.push(TestA(3));
        let ref_b3 = table.push(TestB(4));

        let col_a = table.table.column_type::<TestA>().id();
        let col_b = table.table.column_type::<TestA>().id();

        let mut cols_a = Vec::<ColumnTypeId>::new();
        cols_a.push(col_a);

        let type_a = table.entity_type(cols_a);

        let mut cols_b = Vec::<ColumnTypeId>::new();
        cols_b.push(col_b);
        
        let type_b = table.entity_type(cols_b);

        let ert_a = table.entity_row_type(ref_a.row_type, type_a);
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
