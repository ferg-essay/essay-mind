use crate::entity::{
    prelude::{Query, QueryCursor, QueryBuilder, Insert, InsertBuilder, InsertCursor}};

pub trait Component:'static {}

#[derive (Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComponentId(usize);


//
// Insert tuples of components
//
//struct IsEntity;

impl<T:Component> Insert for T {
    //type Item = Self;

    fn build(builder: &mut InsertBuilder) {
        builder.add_column::<T>();
    }

    unsafe fn insert(cursor: &mut InsertCursor, this: Self) {
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

impl<T:Component> Query for &T {
    type Item<'t> = &'t T;

    fn build(builder: &mut QueryBuilder) {
        builder.add_ref::<T>();
    }

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { // Self::Item { // <'a> {
        cursor.deref::<T>()
    }
}

impl<T:Component> Query for &mut T {
    type Item<'t> = &'t mut T;

    fn build(builder: &mut QueryBuilder) {
        builder.add_ref::<T>();
    }

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor<'a,'t>) -> Self::Item<'t> { //<'a> {
        cursor.deref_mut::<T>()
    }
}
