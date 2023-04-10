use crate::store::{row::Row, row_meta::{InsertPlan, InsertBuilder, Insert, InsertCursor}};

use super::{prelude::EntityTable};

pub trait Component:'static {}

#[derive (Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComponentId(usize);


//
// Insert tuples of components
//
struct IsEntity;
impl<T:Component> Insert<IsEntity> for T {
    type Item = Self;

    fn build(builder: &mut InsertBuilder) {
        builder.add_column::<T>();
    }

    unsafe fn insert(cursor: &mut InsertCursor, this: Self::Item) {
        cursor.insert(this);
    }
}

macro_rules! impl_insert_tuple {
    ($($part:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($part:Insert<Item = $part>),*> Insert for ($($part,)*) {
            type Item = Self;

            fn build(
                builder: &mut InsertBuilder, 
            ) {
                $(
                    $part::build(builder);
                )*
            }

            unsafe fn insert(cursor: &mut InsertCursor, this: Self) {
                let ($($part),*) = this;

                $(
                    $part::insert(cursor, $part);
                )*
            }
        }
    }
}
/*
//impl_insert_tuple!();
impl_insert_tuple!(P1,P2);
impl_insert_tuple!(P1,P2,P3);
impl_insert_tuple!(P1,P2,P3,P4);
impl_insert_tuple!(P1,P2,P3,P4,P5);
*/
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

        unsafe { row.deref::<T>(index) }
    }
}

impl<'a,T:Component> ViewQuery<'a> for &'a mut T {
    fn query(row: &'a Row, i: &mut usize) -> Self {
        let index = *i;
        *i += 1;

        unsafe { row.deref_mut::<T>(index) }
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

//impl_query_tuple!();
impl_query_tuple!(P1,P2);
impl_query_tuple!(P1,P2,P3);
impl_query_tuple!(P1,P2,P3,P4);
impl_query_tuple!(P1,P2,P3,P4,P5);
