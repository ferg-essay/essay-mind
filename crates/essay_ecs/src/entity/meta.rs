use std::{mem, collections::{HashMap, HashSet}, cmp::max, slice::Iter, any::{TypeId, type_name}, borrow::Cow, alloc::Layout};

use super::{prelude::{RowId}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColumnTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RowTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ViewRowTypeId(usize);

#[derive(Clone, Debug)]
pub struct ColumnType {
    id: ColumnTypeId,

    type_id: TypeId,
    name: Cow<'static, str>,

    layout: Layout,
    layout_padded: Layout,

    rows: Vec<EntityTypeId>,
    views: Vec<ViewTypeId>,
}

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityTypeId(usize);

pub struct EntityGroup {
    id: EntityTypeId,

    columns: Vec<ColumnTypeId>,
}

pub struct ViewType {
    id: ViewTypeId,
    cols: Vec<ColumnTypeId>,

    view_rows: Vec<ViewRowTypeId>,
}

pub struct ViewRowType {
    id: ViewRowTypeId,

    view_type_id: ViewTypeId,
    entity_type_id: EntityTypeId,

    index_map: Vec<usize>,
}

pub(crate) struct RowMetas {
    col_map: HashMap<TypeId,ColumnTypeId>,
    columns: Vec<ColumnType>,

    entity_row_map: HashMap<Vec<ColumnTypeId>,EntityTypeId>,
    entity_rows: Vec<EntityGroup>,

    view_map: HashMap<Vec<ColumnTypeId>,ViewTypeId>,
    views: Vec<ViewType>,

    view_row_map: HashMap<(ViewTypeId,EntityTypeId), ViewRowTypeId>,
    view_rows: Vec<ViewRowType>,
}

//
// implementation
//

impl ColumnTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl ColumnType {
    pub fn id(&self) -> ColumnTypeId {
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
}

impl EntityGroup {
    pub(crate) fn id(&self) -> EntityTypeId {
        self.id
    }

    pub(crate) fn columns(&self) -> &Vec<ColumnTypeId> {
        &self.columns
    }

    fn contains_columns(&self, cols: &Vec<ColumnTypeId>) -> bool {
        for col in cols {
            if self.column_find(*col).is_none() {
                return false;
            }
        }

        true
    }

    pub fn column_find(&self, id: ColumnTypeId) -> Option<&ColumnTypeId> {
        self.columns.iter().find(|col| **col == id)
    }
}

impl EntityTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

//
// view
//

impl ViewTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl ViewType {
    pub fn id(&self) -> ViewTypeId {
        self.id
    }

    pub(crate) fn rows(&self) -> &Vec<ViewRowTypeId> {
        &self.view_rows
    }

    pub(crate) fn column_position(&self, col_id: ColumnTypeId) -> Option<usize> {
        self.cols.iter().position(|col| *col == col_id)
    }
}

impl ViewRowTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl ViewRowType {
    pub fn new(
        id: ViewRowTypeId, 
        entity: &EntityGroup, 
        view: &ViewType
    ) -> ViewRowType {
        let mut columns = Vec::<usize>::new();

        for col in &view.cols {
            let index = entity.columns().iter()
                .position(|c| c == col).unwrap();

            columns.push(index);
        }

        ViewRowType {
            id,
            view_type_id: view.id,
            entity_type_id: entity.id(),
            index_map: columns,
        }
    }

    pub fn index_map(&self) -> &Vec<usize> {
        &self.index_map
    }

    pub fn id(&self) -> ViewRowTypeId {
        self.id
    }

    pub(crate) fn view_type_id(&self) -> ViewTypeId {
        self.view_type_id
    }

    pub(crate) fn row_type_id(&self) -> EntityTypeId {
        self.entity_type_id
    }
}

impl RowMetas {
    pub fn new() -> Self {
        Self {
            col_map: HashMap::new(),
            columns: Vec::new(),

            entity_row_map: HashMap::new(),
            entity_rows: Vec::new(),

            views: Vec::new(),
            view_map: HashMap::new(),

            view_rows: Vec::new(),
            view_row_map: HashMap::new(),
        }
    }

    pub fn get_column(&self, id: ColumnTypeId) -> &ColumnType {
        self.columns.get(id.index()).unwrap()
    }

    pub fn column_mut(&mut self, id: ColumnTypeId) -> &mut ColumnType {
        self.columns.get_mut(id.index()).unwrap()
    }

    pub fn get_column_by_type<T:'static>(&self) -> Option<&ColumnType> {
        match self.col_map.get(&TypeId::of::<T>()) {
            Some(type_id) => { 
                self.columns.get(type_id.index())
            },
            None => None,
        }
    }

    pub(crate) fn get_column_id_by_type<T:'static>(&self) -> Option<ColumnTypeId> {
        match self.col_map.get(&TypeId::of::<T>()) {
            Some(column_type_id) => {
                Some(ColumnTypeId(column_type_id.index()))
            },
            None => None,
        }
    }

    pub fn add_column<T:'static>(&mut self) -> ColumnTypeId {
        let type_id = TypeId::of::<T>();

        let id = *self.col_map.entry(type_id)
            .or_insert(ColumnTypeId(self.columns.len()));

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

    pub fn add_entity_row(&mut self, mut columns: Vec<ColumnTypeId>) -> EntityTypeId {
        columns.sort();
        columns.dedup();

        let mut length: usize = 0;
        let mut align: usize = 1;

        let len = self.entity_rows.len();
        let entity_type_id = *self.entity_row_map.entry(columns.clone()).or_insert_with(|| {
            EntityTypeId(len)
        });

        if entity_type_id.index() < len {
            return entity_type_id;
        }

        self.entity_rows.push(EntityGroup {
            id: entity_type_id,
            columns,
        });

        entity_type_id
    }

    pub fn get_entity_type(&self, id: EntityTypeId) -> &EntityGroup {
        self.entity_rows.get(id.index()).unwrap()
    }

    pub(crate) fn get_mut_entity_type(
        &mut self, 
        entity_type_id: EntityTypeId
    ) -> &mut EntityGroup {
        self.entity_rows.get_mut(entity_type_id.index()).unwrap()
    }

    pub fn push_row(
        &mut self, 
        row_id: EntityTypeId, 
        column_id: ColumnTypeId
    ) -> EntityTypeId {
        todo!();
        let row_type = self.get_entity_type(row_id);

        let mut columns : Vec<ColumnTypeId> = row_type.columns().clone();
        columns.push(column_id);

        self.add_entity_row(columns)
    }

    pub fn push_row_by_type<T:'static>(
        &mut self, 
        row_id: EntityTypeId
    ) -> EntityTypeId {
        let col_id = self.add_column::<T>();
        self.push_row(row_id, col_id)
    }

    fn fill_row_columns(&mut self, row_type_id: EntityTypeId) {
        let col_ids : Vec<ColumnTypeId> = self.get_entity_type(row_type_id)
            .columns()
            .iter()
            .map(|col| *col)
            .collect();

        for col_item_id in &col_ids {
            let col_type = self.column_mut(*col_item_id);

            col_type.rows.push(row_type_id);
        }

        self.build_row_entities(row_type_id, &col_ids);
    }

    fn build_row_entities(
        &mut self, 
        row_type_id: EntityTypeId, 
        col_ids: &Vec<ColumnTypeId>
    ) {
        let mut views: Vec<ViewTypeId> = Vec::new();

        for view_type in &self.views {
            if view_type
                .cols
                .iter()
                .filter(|c| col_ids.iter().any(|c1| *c == c1))
                .count() == view_type.cols.len() {
                views.push(view_type.id());
            }
        }

        for view_Id in views {
            self.add_view_row(row_type_id, view_Id);
        }
    }

    pub fn single_row_type<T:'static>(&mut self) -> EntityTypeId {
        let column_id = self.add_column::<T>();
        let mut columns = Vec::<ColumnTypeId>::new();
        columns.push(column_id);

        self.add_entity_row(columns)
    }

    pub fn col_rows(&self, col: ColumnTypeId) -> Iter<EntityTypeId> {
        self.columns.get(col.index()).unwrap().rows.iter()
    }

    pub fn col_join_rows(&self, cols: Vec<ColumnTypeId>) -> Vec<EntityTypeId> {
        let mut rows = self.col_rows(cols[0])
            .map(|row| *row)
            .collect::<HashSet<_>>();

        for col in cols.iter().skip(1) {
            let next_rows = self.col_rows(*col)
                .filter(|row| rows.contains(row))
                .map(|row| *row)
                .collect::<HashSet<_>>();

            rows = next_rows
        }

        let mut rows : Vec<EntityTypeId> = rows.iter().map(|row| *row).collect();
        // rows.sort();

        rows
    }

    pub fn get_view(&self, id: ViewTypeId) -> &ViewType {
        self.views.get(id.index()).unwrap()
    }

    fn get_mut_view(&mut self, id: ViewTypeId) -> &mut ViewType {
        self.views.get_mut(id.index()).unwrap()
    }

    pub(crate) fn get_view_by_cols(&self, cols: &Vec<ColumnTypeId>) -> Option<ViewTypeId> {
        match self.view_map.get(cols) {
            Some(type_id) => Some(*type_id),
            None => None,
        }
    }

    pub fn add_view(&mut self, cols: Vec<ColumnTypeId>) -> ViewTypeId {
        let len = self.views.len();

        let view_id = *self.view_map
            .entry(cols.clone())
            .or_insert_with(|| {
            ViewTypeId(len)
        });

        if view_id.0 == len {
            self.views.push(ViewType {
                id: view_id,
                cols: cols,
                view_rows: Vec::new(),
            });

            //self.view_rows.push(Vec::new());

            self.fill_view(view_id);
        }

        view_id
    }

    pub fn fill_view(&mut self, view_id: ViewTypeId) {
        let view_type = self.get_view(view_id);
        let cols = view_type.cols.clone();

        for col in &cols {
            let col_type = self.column_mut(*col);

            col_type.views.push(view_id);
        }

        let mut match_rows = Vec::<EntityTypeId>::new();

        for row in &self.entity_rows {
            if row.contains_columns(&cols) {
                match_rows.push(row.id());
            }
        }

        for row_id in match_rows {
            self.add_view_row(row_id, view_id);
        }
    }

    pub(crate) fn extend_view_type(
        &mut self, 
        view_type_id: ViewTypeId, 
        col_type_id: ColumnTypeId
    ) -> ViewTypeId {
        let view_type = self.get_view(view_type_id);

        let mut cols = view_type.cols.clone();
        cols.push(col_type_id);

        self.add_view(cols)
    }

    pub(crate) fn add_view_row(
        &mut self,
        row_id: EntityTypeId, 
        view_id: ViewTypeId
    ) -> ViewRowTypeId {
        let len = self.view_rows.len();

        let view_row_id = *self.view_row_map
            .entry((view_id, row_id))
            .or_insert_with(|| {
            ViewRowTypeId(len)
        });

        if view_row_id.index() == len {
            self.push_view_row(row_id, view_id, view_row_id);
        }

        view_row_id
    }

    fn push_view_row(
        &mut self, 
        row_id: EntityTypeId,
        view_id: ViewTypeId, 
        view_row_id: ViewRowTypeId
    ) {
        let row = self.get_entity_type(row_id);
        let view_type = self.get_view(view_id);

        assert_eq!(view_row_id.index(), self.view_rows.len());

        self.view_rows.push(ViewRowType::new(view_row_id, row, view_type));

        let view_type = self.get_mut_view(view_id);
        view_type.view_rows.push(view_row_id);
    }

    fn row_type(
        &mut self, 
        entity_id: ViewTypeId, 
        row_id: EntityTypeId, 
        type_id: ViewRowTypeId
    ) -> ViewRowType {
        let entity_type = self.get_view(entity_id);

        let row = self.get_entity_type(row_id);

        ViewRowType::new(type_id, row, entity_type)
    }

    pub fn view_row_cols(
        &mut self, 
        row_type_id: EntityTypeId, 
        columns: Vec<ColumnTypeId>
    ) -> ViewRowTypeId {
        let entity_type_id = self.add_view(columns);

        self.add_view_row(row_type_id, entity_type_id)
    }


    pub fn get_view_row(&self, id: ViewRowTypeId) -> &ViewRowType {
        self.view_rows.get(id.index()).unwrap()
    }

    pub fn get_view_rows(&self, id: ViewTypeId) -> &Vec<ViewRowTypeId> {
        &self.get_view(id).view_rows
    }

    pub fn single_view_type<T:'static>(&mut self) -> ViewTypeId {
        let column_id = self.add_column::<T>();
        let mut columns = Vec::<ColumnTypeId>::new();
        columns.push(column_id);

        self.add_view(columns)
    }

    pub(crate) fn get_single_view_type<T:'static>(&self) -> Option<ViewTypeId> {
        match self.get_column_by_type::<T>() {
            Some(col) => {
                let mut col_vec = Vec::<ColumnTypeId>::new();
                col_vec.push(col.id());

                self.get_view_by_cols(&col_vec)
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{mem, alloc::Layout};

    use crate::entity::meta::{ColumnTypeId, ViewTypeId, ViewRowTypeId, EntityTypeId};

    use super::RowMetas;

    #[test]
    fn add_column() {
        let mut meta = RowMetas::new();

        let col_id = meta.add_column::<TestA>();
        let col_type = meta.get_column(col_id);
        assert_eq!(col_type.id(), ColumnTypeId(0));
        assert_eq!(col_type.size(), mem::size_of::<usize>());
        assert_eq!(col_type.size_padded(), mem::size_of::<usize>());
        //assert_eq!(col_type.layout(), &Layout::new::<TestA>());
        assert_eq!(col_type.layout_padded(), &Layout::new::<TestA>().pad_to_align());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.views.len(), 0);

        let col_id = meta.add_column::<TestB>();
        let col_type = meta.get_column(col_id);
        assert_eq!(col_type.id(), ColumnTypeId(1));
        assert_eq!(col_type.size(), mem::size_of::<usize>());
        //assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.views.len(), 0);

        // check double add
        let col_id = meta.add_column::<TestA>();
        assert_eq!(col_id, ColumnTypeId(0));
    }

    #[test]
    fn add_single_row() {
        let mut meta = RowMetas::new();

        let type_a_id = meta.single_row_type::<TestA>();
        assert_eq!(type_a_id, EntityTypeId(0));

        let type_a = meta.get_entity_type(type_a_id);
        assert_eq!(type_a.id(), EntityTypeId(0));
        let cols = type_a.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], EntityTypeId(0));
        assert_eq!(col_a.views.len(), 0);

        let type_a_id = meta.single_row_type::<TestA>();
        assert_eq!(type_a_id, EntityTypeId(0));

        let type_a = meta.get_entity_type(type_a_id);
        assert_eq!(type_a.id(), EntityTypeId(0));
        /*
        assert_eq!(type_a.align(), mem::align_of::<TestA>());
        assert_eq!(type_a.length(), mem::size_of::<TestA>());
        */
        let cols = type_a.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.views.len(), 0);

        let type_b_id = meta.single_row_type::<TestB>();
        assert_eq!(type_b_id, EntityTypeId(1));

        let type_b = meta.get_entity_type(type_b_id);
        assert_eq!(type_b.id(), EntityTypeId(1));
        let cols = type_b.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(1));

        let col_b = meta.get_column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], EntityTypeId(1));
        assert_eq!(col_b.views.len(), 0);
    }

    #[test]
    fn push_row() {
        let mut meta = RowMetas::new();

        let type_a_id = meta.single_row_type::<TestA>();
        assert_eq!(type_a_id, EntityTypeId(0));

        let type_a = meta.get_entity_type(type_a_id);
        assert_eq!(type_a.id(), EntityTypeId(0));

        let type_aa_id = meta.push_row_by_type::<TestA>(type_a_id);
        assert_eq!(type_aa_id, EntityTypeId(0));

        let type_aa = meta.get_entity_type(type_aa_id);
        assert_eq!(type_aa.id(), EntityTypeId(0));

        let cols = type_aa.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));

        let type_b_id = meta.single_row_type::<TestB>();
        assert_eq!(type_b_id, EntityTypeId(1));

        let type_ab_id = meta.push_row_by_type::<TestB>(type_a_id);
        assert_eq!(type_ab_id, EntityTypeId(2));

        let type_ab = meta.get_entity_type(type_ab_id);
        assert_eq!(type_ab.id(), EntityTypeId(2));

        let cols = type_ab.columns();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0], ColumnTypeId(0));
        assert_eq!(cols[1], ColumnTypeId(1));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 2);
        assert_eq!(col_a.rows[0], EntityTypeId(0));
        assert_eq!(col_a.rows[1], EntityTypeId(2));

        let col_b = meta.get_column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 2);
        assert_eq!(col_b.rows[0], EntityTypeId(1));
        assert_eq!(col_b.rows[1], EntityTypeId(2));

        let type_aba = meta.push_row_by_type::<TestA>(type_ab_id);
        assert_eq!(type_aba, EntityTypeId(2));

        let type_ba = meta.push_row_by_type::<TestA>(type_b_id);
        assert_eq!(type_ba, EntityTypeId(2));
    }

    #[test]
    fn row_cols() {
        let mut meta = RowMetas::new();

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, EntityTypeId(0));

        let col_a = meta.add_column::<TestA>();

        let row_type = meta.get_entity_type(type_a);
        assert_eq!(row_type.id(), type_a);
        assert_eq!(row_type.columns().len(), 1);
        let cols = row_type.columns();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], col_a);

        let type_b = meta.single_row_type::<TestB>();
        assert_eq!(type_b, EntityTypeId(1));

        let col_b = meta.add_column::<TestB>();

        let type_ba = meta.push_row_by_type::<TestA>(type_b);
        assert_eq!(type_ba, EntityTypeId(2));

        let row_type = meta.get_entity_type(type_ba);
        assert_eq!(row_type.id(), type_ba);
        assert_eq!(row_type.columns().len(), 2);
        let cols = row_type.columns();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0], col_a);
        assert_eq!(cols[1], col_b);
    }

    #[test]
    fn col_rows() {
        let mut meta = RowMetas::new();

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, EntityTypeId(0));

        let col_a = meta.add_column::<TestA>();
        let rows: Vec<EntityTypeId> = meta.col_rows(col_a).map(|id| *id).collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], EntityTypeId(0));

        let col_c = meta.add_column::<TestC>();
        let col_b = meta.add_column::<TestB>();

        let type_c = meta.single_row_type::<TestC>();
        assert_eq!(type_c, EntityTypeId(1));

        let type_cb = meta.push_row_by_type::<TestB>(type_c);
        assert_eq!(type_cb, EntityTypeId(2));

        let type_cba = meta.push_row_by_type::<TestA>(type_cb);
        assert_eq!(type_cba, EntityTypeId(3));

        let rows: Vec<EntityTypeId> = meta.col_rows(col_a).map(|id| *id).collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], EntityTypeId(0));
        assert_eq!(rows[1], EntityTypeId(3));

        let rows: Vec<EntityTypeId> = meta.col_rows(col_b).map(|id| *id).collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], EntityTypeId(2));
        assert_eq!(rows[1], EntityTypeId(3));

        let rows: Vec<EntityTypeId> = meta.col_rows(col_c).map(|id| *id).collect();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], EntityTypeId(1));
        assert_eq!(rows[1], EntityTypeId(2));
        assert_eq!(rows[2], EntityTypeId(3));
    }

    #[test]
    fn row_then_view() {
        let mut meta = RowMetas::new();

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, EntityTypeId(0));

        let view_id_a = meta.single_view_type::<TestA>();
        assert_eq!(view_id_a, ViewTypeId(0));

        let view_a = meta.get_view(view_id_a);
        assert_eq!(view_a.id(), ViewTypeId(0));
        let cols = &view_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], EntityTypeId(0));
        assert_eq!(col_a.views.len(), 1);
        assert_eq!(col_a.views[0], ViewTypeId(0));

        let entity_row_a = meta.get_view_row(ViewRowTypeId(0));
        assert_eq!(entity_row_a.id(), ViewRowTypeId(0));
        assert_eq!(entity_row_a.row_type_id(), EntityTypeId(0));
        assert_eq!(entity_row_a.view_type_id(), ViewTypeId(0));
    }

    #[test]
    fn view_then_row() {
        let mut meta = RowMetas::new();

        let entity_id_a = meta.single_view_type::<TestA>();
        assert_eq!(entity_id_a, ViewTypeId(0));

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, EntityTypeId(0));

        //meta.push_row(row_id_a, col_id_b);

        let entity_a = meta.get_view(entity_id_a);
        assert_eq!(entity_a.id(), ViewTypeId(0));
        let cols = &entity_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));
        let rows = &entity_a.view_rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], ViewRowTypeId(0));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], EntityTypeId(0));
        assert_eq!(col_a.views.len(), 1);
        assert_eq!(col_a.views[0], ViewTypeId(0));

        let entity_row_a = meta.get_view_row(ViewRowTypeId(0));
        assert_eq!(entity_row_a.id(), ViewRowTypeId(0));
        assert_eq!(entity_row_a.row_type_id(), EntityTypeId(0));
        assert_eq!(entity_row_a.view_type_id(), ViewTypeId(0));
    }

    #[test]
    fn view_then_row2() {
        let mut meta = RowMetas::new();

        let entity_id_a = meta.single_view_type::<TestA>();
        assert_eq!(entity_id_a, ViewTypeId(0));

        let entity_id_b = meta.single_view_type::<TestB>();
        assert_eq!(entity_id_b, ViewTypeId(1));

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, EntityTypeId(0));

        let row_id_b = meta.single_row_type::<TestB>();
        assert_eq!(row_id_b, EntityTypeId(1));

        let entity_b = meta.get_view(entity_id_b);
        assert_eq!(entity_b.id(), ViewTypeId(1));
        let cols = &entity_b.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(1));
        let rows = &entity_b.view_rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], ViewRowTypeId(1));

        let col_b = meta.get_column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], EntityTypeId(1));
        assert_eq!(col_b.views.len(), 1);
        assert_eq!(col_b.views[0], ViewTypeId(1));

        let entity_row_b = meta.get_view_row(ViewRowTypeId(1));
        assert_eq!(entity_row_b.id(), ViewRowTypeId(1));
        assert_eq!(entity_row_b.row_type_id(), EntityTypeId(1));
        assert_eq!(entity_row_b.view_type_id(), ViewTypeId(1));
    }


    struct TestA(usize);
    struct TestB(usize);
    struct TestC(usize);
}