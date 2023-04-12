use std::{mem, collections::{HashMap, HashSet}, cmp::max, slice::Iter, any::{TypeId, type_name}, borrow::Cow, alloc::Layout};

use super::{prelude::{Row, RowId}};

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

#[derive(Clone, Debug)]
pub struct ColumnItem {
    col_id: ColumnTypeId,
    row_id: RowTypeId,

    index: usize,

    align: usize,
    length: usize,

    offset: usize,
}

pub struct RowType {
    id: RowTypeId,
    columns: Vec<ColumnItem>,
    align: usize,
    length: usize,
}

pub trait Insert2<M>:'static {
    fn build(builder: &mut InsertBuilder2);

    unsafe fn insert(cursor: &mut InsertCursor2, value: Self);
}

pub struct InsertBuilder2<'a> {
    meta: &'a mut RowMetas,
    columns: Vec<ColumnTypeId>,
}

pub struct InsertPlan2 {
    row_type: RowTypeId,
    row_cols: Vec<usize>,
}

pub struct InsertCursor2<'a, 't> {
    row: &'a mut Row<'t>,
    map: &'a InsertPlan2,
    index: usize,
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

pub trait Query2<M> {
    type Item<'a>;

    fn build(query: &mut QueryBuilder2);

    unsafe fn query<'a,'t>(cursor: &mut QueryCursor2<'a,'t>) -> Self::Item<'t>;
}

pub struct QueryCursor2<'a,'t> {
    row: &'a Row<'t>,
    cols: &'a Vec<usize>,
    index: usize,
}

enum AccessType {
    AccessRef,
    AccessMut
}

pub struct QueryBuilder2<'a> {
    meta: &'a mut RowMetas, 
    cols: Vec<ColumnTypeId>,
}

pub(crate) struct RowMetas {
    col_map: HashMap<TypeId,ColumnTypeId>,
    columns: Vec<ColumnType>,

    row_map: HashMap<Vec<ColumnTypeId>,RowTypeId>,
    rows: Vec<RowType>,

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

impl ColumnItem {
    pub fn id(&self) -> ColumnTypeId {
        self.col_id
    }

    pub fn align(&self) -> usize {
        self.align
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl RowTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl RowType {
    pub fn id(&self) -> RowTypeId {
        self.id
    }

    pub fn align(&self) -> usize {
        self.align
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn columns(&self) -> Iter<ColumnItem> {
        self.columns.iter()
    }

    pub fn column(&self, index: usize) -> &ColumnItem {
        self.columns.get(index).unwrap()
    }

    pub fn column_position(&self, id: ColumnTypeId) -> Option<usize> {
        self.columns.iter().position(|col| col.id() == id)
    }

    pub fn column_find(&self, id: ColumnTypeId) -> Option<&ColumnItem> {
        self.columns.iter().find(|col| col.id() == id)
    }

    fn contains_columns(&self, cols: &Vec<ColumnTypeId>) -> bool {
        for col in cols {
            if self.column_find(*col).is_none() {
                return false;
            }
        }

        true
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityTypeId(usize);

pub struct EntityGroup {
    id: EntityTypeId,

    columns: Vec<ColumnTypeId>,
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

impl<'a> InsertBuilder2<'a> {
    pub(crate) fn new(meta: &'a mut RowMetas) -> Self {
        Self {
            meta: meta,
            columns: Vec::new(),
        }
    }

    pub(crate) fn add_column<T:'static>(&mut self) {
        let id = self.meta.add_column::<T>();
        
        self.columns.push(id);
    }

    pub(crate) fn build(self) -> InsertPlan2 {
        let row_id = self.meta.add_row(self.columns.clone());
        let row = self.meta.get_row_id(row_id);

        let mut row_cols = Vec::<usize>::new();

        for col_id in &self.columns {
            row_cols.push(row.column_position(*col_id).unwrap());
        }

        InsertPlan2 {
            row_type: row.id(),
            row_cols: row_cols,
        }
    }
}

impl InsertPlan2 {
    pub fn index(&self, index: usize) -> usize {
        self.row_cols[index]
    }

    pub(crate) fn row_type(&self) -> RowTypeId {
        self.row_type
    }

    pub(crate) fn cursor<'a, 't>(&'a self, row: &'a mut Row<'t>) -> InsertCursor2<'a, 't> {
        InsertCursor2 {
            map: &self,
            row: row,
            index: 0, 
        }
    }
}

impl<'a, 't> InsertCursor2<'a, 't> {
    pub unsafe fn insert<T:'static>(&mut self, value: T) {
        let index = self.index;
        self.index += 1;

        self.row.write::<T>(self.map.row_cols[index], value);
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

pub(crate) struct QueryPlan2 {
    view: ViewTypeId,
    cols: Vec<usize>,
}

impl QueryPlan2 {
    pub(crate) fn new_cursor<'a,'t>(
        &'a self, 
        row: &'a Row<'t>
    ) -> QueryCursor2<'a,'t> {
        QueryCursor2 {
            row: row,
            cols: &self.cols,
            index: 0,
        }
    }

    pub(crate) fn view(&self) -> ViewTypeId {
        self.view
    }
}

impl<'a,'t> QueryCursor2<'a,'t> {
    pub unsafe fn deref<T:'static>(&mut self) -> &'t T {
        let index = self.index;
        self.index += 1;

        self.row.deref(self.cols[index])
    }

    pub unsafe fn deref_mut<T:'static>(&mut self) -> &'t mut T {
        let index = self.index;
        self.index += 1;

        self.row.deref_mut(self.cols[index])
    }
}

impl<'a> QueryBuilder2<'a> {
    pub(crate) fn new(meta: &'a mut RowMetas) -> Self {
        Self {
            meta: meta,
            cols: Vec::new(),
        }
    }

    pub fn add_ref<T:'static>(&mut self) {
        let col_id = self.meta.add_column::<T>();

        self.cols.push(col_id);
    }

    pub fn add_mut<T:'static>(&mut self) {
        let col_id = self.meta.add_column::<T>();

        self.cols.push(col_id);
    }

    pub(crate) fn build(self) -> QueryPlan2 {
        let view_id = self.meta.add_view(self.cols.clone());
        let view = self.meta.get_view(view_id);

        let cols = self.cols.iter()
            .map(|col_id| view.column_position(*col_id).unwrap())
            .collect();

        QueryPlan2 {
            view: view_id,
            cols: cols,
        }
    }
}

impl RowMetas {
    pub fn new() -> Self {
        Self {
            col_map: HashMap::new(),
            columns: Vec::new(),

            row_map: HashMap::new(),
            rows: Vec::new(),

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

    pub fn get_row_id(&self, row_type_id: RowTypeId) -> &RowType {
        self.rows.get(row_type_id.index()).unwrap()
    }

    pub fn add_row(&mut self, mut columns: Vec<ColumnTypeId>) -> RowTypeId {
        todo!();
        /*
        columns.sort();
        columns.dedup();

        let mut length: usize = 0;
        let mut align: usize = 1;

        let len = self.row_map.len();
        let row_type_id = *self.row_map.entry(columns.clone()).or_insert_with(|| {
            RowTypeId(len)
        });

        if row_type_id.index() < len {
            return row_type_id;
        }

        let mut items = Vec::<ColumnItem>::new();

        for (index, column_id) in columns.iter().enumerate() {
            let column_type = self.columns.get(column_id.0).unwrap();

            let mut item = ColumnItem {
                col_id: column_type.id(),
                row_id: row_type_id,
                index: index,
                length: column_type.size(),
                align: column_type.layout_padded().align(),
                offset: 0,
            };
            item.offset = length;

            length += column_type.size(); // TODO: align
            align = max(align, column_type.layout_padded().align()); 

            items.push(item);
        }

        self.push_row_type(RowType {
            id: row_type_id,
            columns: items,
            length: length,
            align: align,
        });

        self.fill_row_columns(row_type_id);

        row_type_id
        */
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

    fn push_row_type(&mut self, row_type: RowType) {
        todo!();
        /*
        let row_type_id = row_type.id();

        for col in &row_type.columns {
            let col_type = self.columns.get_mut(col.id().index()).unwrap();

            col_type.rows.push(row_type_id);
        }

        self.rows.push(row_type);
        */
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

    use crate::store::meta::{ColumnTypeId, RowTypeId, ColumnType, ColumnItem, ViewTypeId, ViewRowTypeId, EntityTypeId};

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