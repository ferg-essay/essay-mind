
//
// Insert
//

use crate::prelude::Component;

use super::{meta::{RowTypeId, ColumnId}, prelude::{Table}, column::RowId, table::EntityId};

pub trait Insert:'static {
    fn build(builder: &mut InsertBuilder);

    unsafe fn insert(cursor: &mut InsertCursor, value: Self);
}

pub struct InsertBuilder<'a,'t> {
    table: &'a mut Table<'t>,
    columns: Vec<ColumnId>,
}

pub struct InsertPlan {
    row_type: RowTypeId,
    columns: Vec<ColumnId>,
    index_map: Vec<usize>,
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
        let row_id = self.table.add_row_type(self.columns.clone());
        let row_type = self.table.meta().row(row_id);

        let mut index_map = Vec::<usize>::new();

        for row_column in row_type.columns() {
            index_map.push(self.columns.iter()
                .position(|c| c == row_column)
                .unwrap()
            );
        }

        InsertPlan {
            row_type: row_id,
            columns: self.columns.clone(),
            index_map: index_map,
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
            table.column_mut(column_id).push(value)
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

    pub(crate) fn complete(self) -> EntityId {
        let mut columns = Vec::<RowId>::new();

        for index in &self.plan.index_map {
            columns.push(self.rows[*index]);
        }

        self.table.push_row(self.plan.row_type, columns)
    }
}

//
// Insert tuples of components
//

impl<T:Component> Insert for T {
    fn build(builder: &mut InsertBuilder) {
        builder.add_column::<T>();
    }

    unsafe fn insert(cursor: &mut InsertCursor, this: Self) {
        cursor.insert(this);
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
impl_insert_tuple!(P1,P2,P3);
impl_insert_tuple!(P1,P2,P3,P4);
impl_insert_tuple!(P1,P2,P3,P4,P5);
