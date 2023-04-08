use crate::store::{row::Row, row_meta::{ColumnTypeId, RowType}};

use super::{prelude::EntityTable};

pub trait Component:'static {}

#[derive (Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComponentId(usize);


//
// Insert tuples of components
//

pub trait Insert:'static {
    fn add_cols(table: &mut EntityTable, cols: &mut InsertMap);

    fn insert(row: &mut Row, cols: &InsertMap, index: usize, this: Self) -> usize;
}

pub struct InsertMap {
    col_ids: Vec<ColumnTypeId>,
    row_cols: Vec<usize>,
}

impl InsertMap {
    pub fn new() -> Self {
        Self {
            col_ids: Vec::new(),
            row_cols: Vec::new(),
        }
    }

    fn push(&mut self, id: ColumnTypeId) {
        self.col_ids.push(id);
    }

    fn fill(&mut self, row: &RowType) {
        for col_id in &self.col_ids {
            self.row_cols.push(row.column_position(*col_id).unwrap());
        }
    }

    fn index(&self, index: usize) -> usize {
        self.row_cols[index]
    }

    pub(crate) fn column_types(&self) -> &Vec<ColumnTypeId> {
        &self.col_ids
    }
}

impl<T:Component> Insert for T {
    fn add_cols(table: &mut EntityTable, cols: &mut InsertMap) {
        cols.push(table.add_column::<T>());
    }

    fn insert(row: &mut Row, cols: &InsertMap, index: usize, this: Self) -> usize {
        row.insert(cols.index(index), this);

        index + 1
    }
}

macro_rules! impl_insert_tuple {
    ($($part:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($part:Insert),*> Insert for ($($part,)*)
        {
            fn add_cols(
                table: &mut EntityTable, 
                cols: &mut InsertMap
            ) {
                $(
                    $part::add_cols(table, cols);
                )*
            }

            fn insert(row: &mut Row, cols: &InsertMap, index: usize, this: Self) -> usize {
                let mut index = index;

                let ($($part),*) = this;

                $(
                    index = $part::insert(row, cols, index, $part);
                )*
        
                index
            }
        }
    }
}

impl_insert_tuple!();
impl_insert_tuple!(P1,P2);
impl_insert_tuple!(P1,P2,P3);
impl_insert_tuple!(P1,P2,P3,P4);
impl_insert_tuple!(P1,P2,P3,P4,P5);

//
// query tuples of components
//

pub trait ViewQuery<'a> {
    fn query(row: &'a Row, i: &mut usize) -> Self;
}

impl<'a,T:Component> ViewQuery<'a> for &'a T {
    fn query(row: &'a Row, i: &mut usize) -> Self {
        let index = *i;
        *i += 1;

        unsafe { row.ptr(index).deref() }
    }
}

impl<'a,T:Component> ViewQuery<'a> for &'a mut T {
    fn query(row: &'a Row, i: &mut usize) -> Self {
        let index = *i;
        *i += 1;

        unsafe { row.ptr(index).deref_mut() }
    }
}

//
// View query composed of tuples
//

macro_rules! impl_query_tuple {
    ($($part:ident),*) => {
        #[allow(non_snake_case)]
        impl<'a,$($part:ViewQuery<'a>),*> ViewQuery<'a> for ($($part,)*)
        {
            fn query(row: &'a Row, i: &mut usize) -> Self {
                ($(
                    $part::query(row, i),
                )*)
            }
        }
    }
}

impl_query_tuple!();
impl_query_tuple!(P1,P2);
impl_query_tuple!(P1,P2,P3);
impl_query_tuple!(P1,P2,P3,P4);
impl_query_tuple!(P1,P2,P3,P4,P5);
