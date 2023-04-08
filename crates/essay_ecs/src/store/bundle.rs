use super::{component::Component, prelude::EntityTable, row_meta::ColumnTypeId};


pub trait Bundle:'static {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>);

    // fn build<'a>(row: &'a mut Row) -> Self;
}
/*
impl EntityCols for () {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>) {
    }
}
*/

impl<T:Component> Bundle for T {
    fn add_cols(table: &mut EntityTable, cols: &mut Vec<ColumnTypeId>) {
        cols.push(table.add_column::<T>());
    }

    /*
    fn build<'a>(row: &'a mut Row) -> Self {
        todo!()
        // unsafe { row.ptr(0).read() }
    }
    */
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
        impl<$($param:Bundle),*> Bundle for ($($param,)*)
        {
            fn add_cols(
                table: &mut EntityTable, 
                cols: &mut Vec<ColumnTypeId>
            ) {
                ($($param::add_cols(table, cols),
                )*);
            }

            /*
            fn build<'a>(row: &'a mut Row) -> Self {
                ($($param::build(row),)*)
            }
            */
        }
    }
}

impl_entity_tuple!();
impl_entity_tuple!(P1,P2);
impl_entity_tuple!(P1,P2,P3);
impl_entity_tuple!(P1,P2,P3,P4);
impl_entity_tuple!(P1,P2,P3,P4,P5);
