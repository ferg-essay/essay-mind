use std::{marker::PhantomData, any::{TypeId, type_name}};

use crate::store::{prelude::{Table, RowId, Row}, 
    row_meta::{ViewRowTypeId, ViewRowType, ViewTypeId}, 
    row_meta::{ColumnTypeId, RowTypeId}};

use super::{component::{Insert, InsertMap}, prelude::EntityRef};

pub struct EntityTable<'w> {
    table: Table<'w>,
//    entity_meta: EntityMeta,
}

impl<'t> EntityTable<'t> {
    pub fn new() -> Self {
        EntityTable {
            table: Table::new(),
  //          entity_meta: EntityMeta::new(),
        }
    }

    pub fn push<T:Insert>(&mut self, value: T) -> EntityRef {
        let mut cols = InsertMap::new();

        T::add_cols(self, &mut cols);

        let row_type = self.table.row_meta_mut().add_row(cols.column_types().clone());

        let row = self.table.push_empty_row(row_type);

        let type_id = self.entity_row_by_type::<T>(row_type);

        EntityRef::new(
            row,
            row_type,
            type_id,
        )
    }

    pub(crate) fn add_row_type<T:Insert>(&mut self) -> RowTypeId {
        let mut cols = InsertMap::new();

        T::add_cols(self, &mut cols);

        self.table.row_meta_mut().add_row(cols.column_types().clone())
    }

    pub(crate) fn add_entity_type<T:Insert>(&mut self) -> ViewTypeId {
        let mut cols = InsertMap::new();

        T::add_cols(self, &mut cols);

        self.entity_type(cols.column_types().clone())
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

    pub fn entity_type(&mut self, cols: Vec<ColumnTypeId>) -> ViewTypeId {
        self.table.row_meta_mut().add_view_type(cols)
    }

    pub fn entity_row_type(
        &mut self, 
        row_id: RowTypeId, 
        entity_id: ViewTypeId
    ) -> ViewRowTypeId {
        let row_type = self.table.get_row_type(row_id);

        todo!()
        //self.entity_meta.entity_row(row_type, entity_id)
    }

    pub fn entity_row_by_type<T:'static>(&mut self, row_id: RowTypeId) -> ViewRowTypeId {
        let entity_id = self.table.row_meta_mut().single_view_type::<T>();

        self.table.row_meta_mut().add_view_row(row_id, entity_id)
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
        match self.table.row_meta().get_single_view_type::<T>() {
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
        match self.table.row_meta().get_single_view_type::<T>() {
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

    pub(crate) fn iter_mut_by_type<T:Insert>(&mut self) -> Entity3MutIterator<T> {
        let entity_type = self.add_entity_type::<T>();

        Entity3MutIterator {
            table: self,
            entity_type: entity_type,
            entity_type_index: 0,
            row_index: 0,
            marker: PhantomData,
        }
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
        let entity = self.table.table.row_meta().get_view_type(self.entity_type);

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
        let entity = self.table.table.row_meta().get_view_type(self.entity_type);

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
        let entity = self.table.table.row_meta().get_view_type(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let entity_row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;

            let entity_row = self.table.table.row_meta().get_view_row(entity_row_type_id);
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
        let entity = self.table.table.row_meta().get_view_type(self.entity_type);

        while self.entity_type_index < entity.rows().len() {
            let entity_row_type_id = entity.rows()[self.entity_type_index];

            let row_index = self.row_index;
            self.row_index += 1;

            let entity_row = self.table.table.row_meta().get_view_row(entity_row_type_id);
            let row_type_id = entity_row.row_type_id();

            match self.table.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
                    println!("iter-row {:?} {:?}", entity_row.columns()[0], type_name::<T>());
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
