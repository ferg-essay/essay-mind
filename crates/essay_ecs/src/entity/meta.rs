use std::{mem, collections::{HashMap, HashSet}, slice::Iter, any::{TypeId, type_name}, borrow::Cow, alloc::Layout};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColumnId(usize);

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct RowTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewRowId(usize);

#[derive(Clone, Debug)]
pub struct ColumnType {
    id: ColumnId,

    type_id: TypeId,
    name: Cow<'static, str>,

    layout: Layout,
    layout_padded: Layout,

    rows: Vec<RowTypeId>,
    views: Vec<ViewId>,
}

pub struct RowType {
    id: RowTypeId,

    columns: Vec<ColumnId>,
}

pub struct ViewType {
    id: ViewId,
    cols: Vec<ColumnId>,

    view_rows: Vec<ViewRowId>,
}

pub struct ViewRowType {
    id: ViewRowId,

    view_id: ViewId,
    row_id: RowTypeId,

    index_map: Vec<usize>,
}

pub(crate) struct TableMeta {
    column_map: HashMap<TypeId,ColumnId>,
    columns: Vec<ColumnType>,

    row_map: HashMap<Vec<ColumnId>,RowTypeId>,
    rows: Vec<RowType>,

    view_map: HashMap<Vec<ColumnId>,ViewId>,
    views: Vec<ViewType>,

    view_row_map: HashMap<(ViewId,RowTypeId), ViewRowId>,
    view_rows: Vec<ViewRowType>,
}

//
// implementation
//

//
// Column
//

impl ColumnId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl ColumnType {
    pub fn id(&self) -> ColumnId {
        self.id
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.layout.size()
    }
    
    pub fn layout_padded(&self) -> &Layout {
        &self.layout_padded
    }

    #[inline]
    pub fn size_padded(&self) -> usize {
        self.layout_padded.size()
    }

    fn get_rows(&self) -> &Vec<RowTypeId> {
        &self.rows
    }

    fn rows(&self) -> impl Iterator<Item=&RowTypeId> {
        self.rows.iter()
    }
}

//
// Row
//

impl RowTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl RowType {
    pub(crate) fn id(&self) -> RowTypeId {
        self.id
    }

    pub(crate) fn columns(&self) -> &Vec<ColumnId> {
        &self.columns
    }

    fn contains_column(&self, column: ColumnId) -> bool {
        self.columns.iter().any(|col| *col == column)
    }

    pub fn find_column(&self, id: ColumnId) -> Option<&ColumnId> {
        self.columns.iter().find(|col| **col == id)
    }

    fn contains_columns(&self, cols: &Vec<ColumnId>) -> bool {
        for col in cols {
            if self.find_column(*col).is_none() {
                return false;
            }
        }

        true
    }
}

//
// View
//

impl ViewId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl ViewType {
    pub fn id(&self) -> ViewId {
        self.id
    }

    pub(crate) fn column_position(&self, col_id: ColumnId) -> Option<usize> {
        self.cols.iter().position(|col| *col == col_id)
    }

    pub(crate) fn view_rows(&self) -> &Vec<ViewRowId> {
        &self.view_rows
    }
}

//
// ViewRow
//

impl ViewRowId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl ViewRowType {
    pub fn new(
        id: ViewRowId, 
        row: &RowType, 
        view: &ViewType
    ) -> ViewRowType {
        let mut index_map = Vec::<usize>::new();

        for col in &view.cols {
            let index = row.columns().iter()
                .position(|c| c == col).unwrap();

            index_map.push(index);
        }

        ViewRowType {
            id,
            view_id: view.id,
            row_id: row.id(),
            index_map,
        }
    }

    pub fn id(&self) -> ViewRowId {
        self.id
    }

    pub(crate) fn view_id(&self) -> ViewId {
        self.view_id
    }

    pub(crate) fn row_id(&self) -> RowTypeId {
        self.row_id
    }

    pub fn index_map(&self) -> &Vec<usize> {
        &self.index_map
    }
}

//
// TableMeta
//

impl TableMeta {
    pub fn new() -> Self {
        Self {
            column_map: HashMap::new(),
            columns: Vec::new(),

            row_map: HashMap::new(),
            rows: Vec::new(),

            view_map: HashMap::new(),
            views: Vec::new(),

            view_row_map: HashMap::new(),
            view_rows: Vec::new(),
        }
    }

    //
    // Column
    //

    pub fn get_column(&self, id: ColumnId) -> &ColumnType {
        self.columns.get(id.index()).unwrap()
    }

    pub fn column_mut(&mut self, id: ColumnId) -> &mut ColumnType {
        self.columns.get_mut(id.index()).unwrap()
    }

    pub(crate) fn get_column_by_type<T:'static>(&self) -> Option<ColumnId> {
        match self.column_map.get(&TypeId::of::<T>()) {
            Some(column_type_id) => {
                Some(ColumnId(column_type_id.index()))
            },
            None => None,
        }
    }

    pub fn add_column<T:'static>(&mut self) -> ColumnId {
        let type_id = TypeId::of::<T>();

        let id = *self.column_map.entry(type_id)
            .or_insert(ColumnId(self.columns.len()));

        if self.columns.len() == id.index() {
            let align = mem::align_of::<T>();
            let length = mem::size_of::<T>();

            let col_type = ColumnType {
                id: id,

                type_id: TypeId::of::<T>(),
                name: Cow::Borrowed(type_name::<T>()),

                layout: Layout::new::<T>(),
                layout_padded: Layout::new::<T>().pad_to_align(),

                rows: Vec::new(),
                views: Vec::new(),
            };

            self.columns.push(col_type);
        }

        id
    }

    //
    // Row
    //

    pub fn get_row(&self, id: RowTypeId) -> &RowType {
        self.rows.get(id.index()).unwrap()
    }

    pub(crate) fn get_mut_row(
        &mut self, 
        entity_type_id: RowTypeId
    ) -> &mut RowType {
        self.rows.get_mut(entity_type_id.index()).unwrap()
    }

    pub fn add_row(&mut self, mut columns: Vec<ColumnId>) -> RowTypeId {
        columns.sort();
        columns.dedup();

        let len = self.rows.len();
        let row_id = *self.row_map.entry(columns.clone()).or_insert_with(|| {
            RowTypeId(len)
        });

        if row_id.index() < len {
            return row_id;
        }

        self.rows.push(RowType {
            id: row_id,
            columns,
        });

        self.fill_row_columns(row_id);
        self.add_view_rows_from_row(row_id);

        row_id
    }

    fn fill_row_columns(&mut self, row_id: RowTypeId) {
        let columns = self.get_row(row_id).columns().clone();

        for column_id in columns {
            let column = self.column_mut(column_id);

            column.rows.push(row_id);
        }
    }

    //
    // View
    //

    pub fn get_view(&self, id: ViewId) -> &ViewType {
        self.views.get(id.index()).unwrap()
    }

    fn get_mut_view(&mut self, id: ViewId) -> &mut ViewType {
        self.views.get_mut(id.index()).unwrap()
    }

    pub(crate) fn find_view_by_columns(&self, cols: &Vec<ColumnId>) -> Option<ViewId> {
        match self.view_map.get(cols) {
            Some(type_id) => Some(*type_id),
            None => None,
        }
    }

    pub fn add_view(&mut self, columns: &Vec<ColumnId>) -> ViewId {
        let len = self.views.len();

        let view_id = *self.view_map
            .entry(columns.clone())
            .or_insert_with(|| {
            ViewId(len)
        });

        if view_id.0 == len {
            self.views.push(ViewType {
                id: view_id,
                cols: columns.clone(),
                view_rows: Vec::new(),
            });

            //self.view_rows.push(Vec::new());

            self.add_view_rows_from_view(view_id);
        }

        view_id
    }

    //
    // ViewRow
    //

    pub fn get_view_row(&self, id: ViewRowId) -> &ViewRowType {
        self.view_rows.get(id.index()).unwrap()
    }

    pub(crate) fn add_view_row(
        &mut self,
        row_id: RowTypeId, 
        view_id: ViewId
    ) -> ViewRowId {
        let len = self.view_rows.len();

        let view_row_id = *self.view_row_map
            .entry((view_id, row_id))
            .or_insert_with(|| {
            ViewRowId(len)
        });

        if view_row_id.index() == len {
            self.push_view_row(row_id, view_id, view_row_id);
        }

        view_row_id
    }

    fn push_view_row(
        &mut self, 
        row_id: RowTypeId,
        view_id: ViewId, 
        view_row_id: ViewRowId
    ) {
        let row = self.get_row(row_id);
        let view_type = self.get_view(view_id);

        assert_eq!(view_row_id.index(), self.view_rows.len());

        self.view_rows.push(ViewRowType::new(view_row_id, row, view_type));

        let view_type = self.get_mut_view(view_id);
        view_type.view_rows.push(view_row_id);
    }

    fn add_view_rows_from_row(
        &mut self, 
        row_id: RowTypeId, 
    ) {
        let row = self.get_row(row_id);

        let mut views: Vec<ViewId> = Vec::new();

        for view_type in &self.views {
            if row.contains_columns(&view_type.cols) {
                views.push(view_type.id());
            }
        }

        for view_id in views {
            self.add_view_row(row_id, view_id);
        }
    }

    fn add_view_rows_from_view(&mut self, view_id: ViewId) {
        let view_type = self.get_view(view_id);
        let cols = view_type.cols.clone();

        for col in &cols {
            let col_type = self.column_mut(*col);

            col_type.views.push(view_id);
        }

        let mut match_rows = Vec::<RowTypeId>::new();

        for row in &self.rows {
            if row.contains_columns(&cols) {
                match_rows.push(row.id());
            }
        }

        for row_id in match_rows {
            self.add_view_row(row_id, view_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{mem, alloc::Layout};

    use crate::entity::meta::{ColumnId, ViewId, ViewRowId, RowTypeId};

    use super::TableMeta;

    #[test]
    fn add_column() {
        let mut meta = TableMeta::new();

        let col_id = meta.add_column::<TestA>();
        let col_type = meta.get_column(col_id);
        assert_eq!(col_type.id(), ColumnId(0));
        assert_eq!(col_type.size(), mem::size_of::<usize>());
        assert_eq!(col_type.size_padded(), mem::size_of::<usize>());
        //assert_eq!(col_type.layout(), &Layout::new::<TestA>());
        assert_eq!(col_type.layout_padded(), &Layout::new::<TestA>().pad_to_align());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.views.len(), 0);

        let col_id = meta.add_column::<TestB>();
        let col_type = meta.get_column(col_id);
        assert_eq!(col_type.id(), ColumnId(1));
        assert_eq!(col_type.size(), mem::size_of::<usize>());
        //assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.views.len(), 0);

        // check double add
        let col_id = meta.add_column::<TestA>();
        assert_eq!(col_id, ColumnId(0));
    }

    #[test]
    fn add_single_row() {
        let mut meta = TableMeta::new();

        let type_a_id = single_row_type::<TestA>(&mut meta);
        assert_eq!(type_a_id, RowTypeId(0));

        let type_a = meta.get_row(type_a_id);
        assert_eq!(type_a.id(), RowTypeId(0));
        let cols = type_a.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(0));

        let col_a = meta.get_column(ColumnId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.views.len(), 0);

        let type_a_id = single_row_type::<TestA>(&mut meta);
        assert_eq!(type_a_id, RowTypeId(0));

        let type_a = meta.get_row(type_a_id);
        assert_eq!(type_a.id(), RowTypeId(0));
        /*
        assert_eq!(type_a.align(), mem::align_of::<TestA>());
        assert_eq!(type_a.length(), mem::size_of::<TestA>());
        */
        let cols = type_a.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(0));

        let col_a = meta.get_column(ColumnId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.views.len(), 0);

        let type_b_id = single_row_type::<TestB>(&mut meta);
        assert_eq!(type_b_id, RowTypeId(1));

        let type_b = meta.get_row(type_b_id);
        assert_eq!(type_b.id(), RowTypeId(1));
        let cols = type_b.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(1));

        let col_b = meta.get_column(ColumnId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.views.len(), 0);
    }

    fn single_row_type<T:'static>(meta: &mut TableMeta) -> RowTypeId {
        let column_id = meta.add_column::<T>();
        let mut columns = Vec::<ColumnId>::new();
        columns.push(column_id);

        meta.add_row(columns)
    }

    #[test]
    fn push_row() {
        let mut meta = TableMeta::new();

        let type_a_id = single_row_type::<TestA>(&mut meta);
        assert_eq!(type_a_id, RowTypeId(0));

        let type_a = meta.get_row(type_a_id);
        assert_eq!(type_a.id(), RowTypeId(0));

        /*
        let type_aa_id = meta.push_row_by_type::<TestA>(type_a_id);
        assert_eq!(type_aa_id, RowTypeId(0));

        let type_aa = meta.get_row(type_aa_id);
        assert_eq!(type_aa.id(), RowTypeId(0));

        let cols = type_aa.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(0));

        let type_b_id = meta.single_row_type::<TestB>();
        assert_eq!(type_b_id, RowTypeId(1));

        let type_ab_id = meta.push_row_by_type::<TestB>(type_a_id);
        assert_eq!(type_ab_id, RowTypeId(2));

        let type_ab = meta.get_row(type_ab_id);
        assert_eq!(type_ab.id(), RowTypeId(2));

        let cols = type_ab.columns();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0], ColumnId(0));
        assert_eq!(cols[1], ColumnId(1));

        let col_a = meta.get_column(ColumnId(0));
        assert_eq!(col_a.rows.len(), 2);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.rows[1], RowTypeId(2));

        let col_b = meta.get_column(ColumnId(1));
        assert_eq!(col_b.rows.len(), 2);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.rows[1], RowTypeId(2));

        let type_aba = meta.push_row_by_type::<TestA>(type_ab_id);
        assert_eq!(type_aba, RowTypeId(2));

        let type_ba = meta.push_row_by_type::<TestA>(type_b_id);
        assert_eq!(type_ba, RowTypeId(2));
        */
    }

    #[test]
    fn row_cols() {
        let mut meta = TableMeta::new();

        let type_a = single_row_type::<TestA>(&mut meta);
        assert_eq!(type_a, RowTypeId(0));

        let col_a = meta.add_column::<TestA>();

        let row_type = meta.get_row(type_a);
        assert_eq!(row_type.id(), type_a);
        assert_eq!(row_type.columns().len(), 1);
        let cols = row_type.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], col_a);

        let type_b = single_row_type::<TestB>(&mut meta);
        assert_eq!(type_b, RowTypeId(1));

        let col_b = meta.add_column::<TestB>();

        /*
        let type_ba = meta.push_row_by_type::<TestA>(type_b);
        assert_eq!(type_ba, RowTypeId(2));

        let row_type = meta.get_row(type_ba);
        assert_eq!(row_type.id(), type_ba);
        assert_eq!(row_type.columns().len(), 2);
        let cols = row_type.columns();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0], col_a);
        assert_eq!(cols[1], col_b);
        */
    }

    #[test]
    fn col_rows() {
        let mut meta = TableMeta::new();

        let type_a = single_row_type::<TestA>(&mut meta);
        assert_eq!(type_a, RowTypeId(0));

        /*
        let col_a = meta.add_column::<TestA>();
        let rows: Vec<RowTypeId> = meta
            .select_rows_by_column(col_a)
            .map(|id| *id).collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], RowTypeId(0));

        let col_c = meta.add_column::<TestC>();
        let col_b = meta.add_column::<TestB>();

        let type_c = meta.single_row_type::<TestC>();
        assert_eq!(type_c, RowTypeId(1));
        */

        /*
        let type_cb = meta.push_row_by_type::<TestB>(type_c);
        assert_eq!(type_cb, RowTypeId(2));

        let type_cba = meta.push_row_by_type::<TestA>(type_cb);
        assert_eq!(type_cba, RowTypeId(3));

        let rows: Vec<RowTypeId> = meta.col_rows(col_a).map(|id| *id).collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], RowTypeId(0));
        assert_eq!(rows[1], RowTypeId(3));

        let rows: Vec<RowTypeId> = meta.col_rows(col_b).map(|id| *id).collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], RowTypeId(2));
        assert_eq!(rows[1], RowTypeId(3));

        let rows: Vec<RowTypeId> = meta.col_rows(col_c).map(|id| *id).collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], RowTypeId(1));
        assert_eq!(rows[1], RowTypeId(2));
        assert_eq!(rows[2], RowTypeId(3));
        */
    }

    #[test]
    fn row_then_view() {
        let mut meta = TableMeta::new();

        let row_id_a = single_row_type::<TestA>(&mut meta);
        assert_eq!(row_id_a, RowTypeId(0));

        let view_id_a = add_view_single::<TestA>(&mut meta);
        assert_eq!(view_id_a, ViewId(0));

        let view_a = meta.get_view(view_id_a);
        assert_eq!(view_a.id(), ViewId(0));
        let cols = &view_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(0));

        let col_a = meta.get_column(ColumnId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.views.len(), 1);
        assert_eq!(col_a.views[0], ViewId(0));

        let entity_row_a = meta.get_view_row(ViewRowId(0));
        assert_eq!(entity_row_a.id(), ViewRowId(0));
        assert_eq!(entity_row_a.row_id(), RowTypeId(0));
        assert_eq!(entity_row_a.view_id(), ViewId(0));
    }

    #[test]
    fn view_then_row() {
        let mut meta = TableMeta::new();

        let entity_id_a = add_view_single::<TestA>(&mut meta);
        assert_eq!(entity_id_a, ViewId(0));

        let row_id_a = single_row_type::<TestA>(&mut meta);
        assert_eq!(row_id_a, RowTypeId(0));

        //meta.push_row(row_id_a, col_id_b);

        let entity_a = meta.get_view(entity_id_a);
        assert_eq!(entity_a.id(), ViewId(0));
        let cols = &entity_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(0));
        let rows = &entity_a.view_rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], ViewRowId(0));

        let col_a = meta.get_column(ColumnId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.views.len(), 1);
        assert_eq!(col_a.views[0], ViewId(0));

        let entity_row_a = meta.get_view_row(ViewRowId(0));
        assert_eq!(entity_row_a.id(), ViewRowId(0));
        assert_eq!(entity_row_a.row_id(), RowTypeId(0));
        assert_eq!(entity_row_a.view_id(), ViewId(0));
    }

    #[test]
    fn view_then_row2() {
        let mut meta = TableMeta::new();

        let entity_id_a = add_view_single::<TestA>(&mut meta);
        assert_eq!(entity_id_a, ViewId(0));

        let entity_id_b = add_view_single::<TestB>(&mut meta);
        assert_eq!(entity_id_b, ViewId(1));

        let row_id_a = single_row_type::<TestA>(&mut meta);
        assert_eq!(row_id_a, RowTypeId(0));

        let row_id_b = single_row_type::<TestB>(&mut meta);
        assert_eq!(row_id_b, RowTypeId(1));

        let entity_b = meta.get_view(entity_id_b);
        assert_eq!(entity_b.id(), ViewId(1));
        let cols = &entity_b.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnId(1));
        let rows = &entity_b.view_rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], ViewRowId(1));

        let col_b = meta.get_column(ColumnId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.views.len(), 1);
        assert_eq!(col_b.views[0], ViewId(1));

        let entity_row_b = meta.get_view_row(ViewRowId(1));
        assert_eq!(entity_row_b.id(), ViewRowId(1));
        assert_eq!(entity_row_b.row_id(), RowTypeId(1));
        assert_eq!(entity_row_b.view_id(), ViewId(1));
    }

    fn add_view_single<T:'static>(meta: &mut TableMeta) -> ViewId {
        let column_id = meta.add_column::<T>();
        let mut columns = Vec::<ColumnId>::new();
        columns.push(column_id);

        meta.add_view(&columns)
    }

    struct TestA(usize);
    struct TestB(usize);
    struct TestC(usize);
}