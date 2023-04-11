use crate::store::{row::Row, meta::{InsertBuilder2, Insert2, InsertCursor2}, prelude::{Query2, QueryBuilder2, QueryCursor2}};

use super::prelude::IsEntity;

pub trait Component:'static {}

#[derive (Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComponentId(usize);


//
// Insert tuples of components
//
//struct IsEntity;

impl<T:Component> Insert2<IsEntity> for T {
    //type Item = Self;

    fn build(builder: &mut InsertBuilder2) {
        builder.add_column::<T>();
    }

    unsafe fn insert(cursor: &mut InsertCursor2, this: Self) {
        cursor.insert(this);
    }
}
/*
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
*/
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

impl<T:Component> Query2<IsEntity> for &T {
    type Item<'t> = &'t T;

    fn build(builder: &mut QueryBuilder2) {
        builder.add_ref::<T>();
    }

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor2<'a,'t>) -> Self::Item<'t> { // Self::Item { // <'a> {
        cursor.deref::<T>()
    }
}

impl<T:Component> Query2<IsEntity> for &mut T {
    type Item<'t> = &'t mut T;

    fn build(builder: &mut QueryBuilder2) {
        builder.add_ref::<T>();
    }

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor2<'a,'t>) -> Self::Item<'t> { //<'a> {
        cursor.deref_mut::<T>()
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
