
//
// Query
//

use std::marker::PhantomData;

use super::{
    prelude::{Table, ViewId}, 
    meta::{RowType, ViewRowType, ColumnId}, 
    table::{Row, Component}
};

pub trait View {
    type Item<'a>;

    fn build(builder: &mut ViewBuilder);

    unsafe fn deref<'a,'t>(cursor: &mut ViewCursor<'a,'t>) -> Self::Item<'t>;
}

pub struct ViewCursor<'a,'t> {
    table: &'a Table<'t>,
    row_type: &'a RowType,
    view_row: &'a ViewRowType,
    row: &'a Row,
    cols: &'a Vec<usize>,
    index: usize,
}

pub struct ViewBuilder<'a, 't> {
    table: &'a mut Table<'t>, 
    columns: Vec<ColumnId>,
}

pub(crate) struct ViewPlan {
    view: ViewId,
    cols: Vec<usize>,
}

impl ViewPlan {
    pub(crate) fn new_cursor<'a,'t>(
        &'a self, 
        table: &'a Table<'t>,
        row_type: &'a RowType,
        view_row: &'a ViewRowType,
        row: &'a Row
    ) -> ViewCursor<'a,'t> {
        ViewCursor {
            table,
            row_type,
            row: row,
            view_row: view_row,
            cols: &self.cols,
            index: 0,
        }
    }

    pub(crate) fn view(&self) -> ViewId {
        self.view
    }
}

impl<'a,'t> ViewCursor<'a,'t> {
    pub unsafe fn deref<T:'static>(&mut self) -> &'t T {
        let index = self.view_row.index_map()[self.cols[self.index]];
        self.index += 1;

        let column_id = self.row_type.columns()[index];
        let row_id = self.row.column_row(index);

        self.table.get_by_id::<T>(column_id, row_id).unwrap()
    }

    pub unsafe fn deref_mut<T:'static>(&mut self) -> &'t mut T {
        let index = self.view_row.index_map()[self.cols[self.index]];
        self.index += 1;

        let column_id = self.row_type.columns()[index];
        let row_id = self.row.column_row(index);

        self.table.get_mut_by_id(column_id, row_id).unwrap()
    }
}

impl<'a, 't> ViewBuilder<'a, 't> {
    pub(crate) fn new(table: &'a mut Table<'t>) -> Self {
        Self {
            table: table,
            columns: Vec::new(),
        }
    }

    pub fn add_ref<T:'static>(&mut self) {
        let col_id = self.table.add_column::<T>();

        self.columns.push(col_id);
    }

    pub fn add_mut<T:'static>(&mut self) {
        let col_id = self.table.add_column::<T>();

        self.columns.push(col_id);
    }

    pub(crate) fn build(self) -> ViewPlan {
        let view_id = self.table.add_view(&self.columns);
        let view = self.table.view(view_id);

        let cols = self.columns.iter()
            .map(|col_id| view.column_position(*col_id).unwrap())
            .collect();

        ViewPlan {
            view: view_id,
            cols: cols,
        }
    }
}

pub struct ViewIterator<'a, 't, T:View> {
    table: &'a Table<'t>,

    view_id: ViewId,
    plan: ViewPlan,

    view_type_index: usize,

    row_index: usize,

    marker: PhantomData<T>,
}

impl<'a, 't, T:View> ViewIterator<'a, 't, T> {
    pub(crate) fn new(
        table: &'a Table<'t>, 
        plan: ViewPlan,
    ) -> Self {
        Self {
            table: table,

            view_id: plan.view(),
            plan,

            view_type_index: 0,
            row_index: 0,

            marker: PhantomData,
        }
    }
}

impl<'a, 't, T:View> Iterator for ViewIterator<'a, 't, T>
{
    type Item = T::Item<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        let view = self.table.meta().view(self.view_id);

        while self.view_type_index < view.view_rows().len() {
            let view_row_id = view.view_rows()[self.view_type_index];
            let view_row = self.table.meta().view_row(view_row_id);
            let row_type_id = view_row.row_id();
            let row_type = self.table.meta().row(row_type_id);
            let row_index = self.row_index;
            self.row_index += 1;

            match self.table.get_row_by_type_index(row_type_id, row_index) {
                Some(row) => {
                    return unsafe { 
                        let mut cursor = self.plan.new_cursor(
                            self.table,
                            row_type, 
                            view_row,
                            row
                        );
                        
                        Some(T::deref(&mut cursor))
                    }
                }
                None => {},
            };

            self.view_type_index += 1;
            self.row_index = 0;
        }

        None
    }
}

//
// view implementation
//

impl<T:Component> View for &T {
    type Item<'t> = &'t T;

    fn build(builder: &mut ViewBuilder) {
        builder.add_ref::<T>();
    }

    unsafe fn deref<'a,'t>(cursor: &mut ViewCursor<'a,'t>) -> Self::Item<'t> { // Self::Item { // <'a> {
        cursor.deref::<T>()
    }
}

impl<T:Component> View for &mut T {
    type Item<'t> = &'t mut T;

    fn build(builder: &mut ViewBuilder) {
        builder.add_ref::<T>();
    }

    unsafe fn deref<'a,'t>(cursor: &mut ViewCursor<'a,'t>) -> Self::Item<'t> { //<'a> {
        cursor.deref_mut::<T>()
    }
}

//
// View composed of tuples
//

macro_rules! impl_query_tuple {
    ($($part:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($part:View,)*> View for ($($part,)*)
        {
            type Item<'t> = ($(
                <$part as View>::Item<'t>,
            )*);

            fn build(builder: &mut ViewBuilder) {
                $(
                    $part::build(builder);
                )*
            }

            unsafe fn deref<'a,'t>(cursor: &mut ViewCursor<'a,'t>) -> Self::Item<'t> { // <'a> {
                ($(
                    $part::deref(cursor),
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
