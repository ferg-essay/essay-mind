use std::{mem, collections::{HashMap, HashSet}, cmp::max, slice::Iter, any::{TypeId, type_name}, borrow::Cow};

use super::prelude::{Row};

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

    align: usize,
    length: usize,

    rows: Vec<RowTypeId>,
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

pub trait Insert:'static {
    type Item;

    fn build(builder: &mut InsertBuilder);

    unsafe fn insert(cursor: &mut InsertCursor, value: Self::Item);
}

pub struct InsertBuilder<'a> {
    meta: &'a mut RowMetas,
    columns: Vec<ColumnTypeId>,
}

pub struct InsertPlan {
    row_type: RowTypeId,
    row_cols: Vec<usize>,
}

pub struct InsertCursor<'a, 't> {
    row: &'a mut Row<'t>,
    map: &'a InsertPlan,
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
    row_type_id: RowTypeId,

    columns: Vec<usize>,
}

pub trait Query<'t> {
    type Item;

    fn build(query: &mut QueryBuilder);

    unsafe fn query<'a>(row: &'a Row<'t>, cursor: &mut QueryCursor) -> Self::Item;
}

enum AccessType {
    AccessRef,
    AccessMut
}

pub struct QueryBuilder<'a> {
    meta: &'a mut RowMetas, 
    cols: Vec<ColumnTypeId>,
}

pub struct QueryCursor<'a> {
    cols: &'a Vec<usize>,
    index: usize,
}

pub(crate) struct RowMetas {
    col_map: HashMap<TypeId,ColumnTypeId>,
    columns: Vec<ColumnType>,

    row_map: HashMap<Vec<ColumnTypeId>,RowTypeId>,
    rows: Vec<RowType>,

    view_map: HashMap<Vec<ColumnTypeId>,ViewTypeId>,
    views: Vec<ViewType>,

    view_row_map: HashMap<(ViewTypeId,RowTypeId), ViewRowTypeId>,
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

    pub fn align(&self) -> usize {
        self.align
    }

    pub fn length(&self) -> usize {
        self.length
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

impl<'a> InsertBuilder<'a> {
    pub(crate) fn new(meta: &'a mut RowMetas) -> Self {
        Self {
            meta: meta,
            columns: Vec::new(),
        }
    }

    /*
    fn push(&mut self, id: ColumnTypeId) {
        self.columns.push(id);
    }

    pub(crate) fn columns(&self) -> &Vec<ColumnTypeId> {
        &self.columns
    }

    pub(crate) fn build_insert(&self, row: &RowType) -> InsertMap {
        let mut row_cols = Vec::<usize>::new();

        for col_id in &self.columns {
            row_cols.push(row.column_position(*col_id).unwrap());
        }

        InsertMap {
            row_type: row.id(),
            row_cols: row_cols,
        }
    }
    */

    pub(crate) fn add_column<T:'static>(&mut self) {
        let id = self.meta.add_column::<T>();
        
        self.columns.push(id);
    }

    pub(crate) fn build(self) -> InsertPlan {
        let row_id = self.meta.add_row(self.columns.clone());
        let row = self.meta.get_row_id(row_id);

        let mut row_cols = Vec::<usize>::new();

        for col_id in &self.columns {
            row_cols.push(row.column_position(*col_id).unwrap());
        }

        InsertPlan {
            row_type: row.id(),
            row_cols: row_cols,
        }
    }
}

impl InsertPlan {
    pub fn index(&self, index: usize) -> usize {
        self.row_cols[index]
    }

    pub(crate) fn row_type(&self) -> RowTypeId {
        self.row_type
    }

    pub(crate) fn cursor<'a, 't>(&'a self, row: &'a mut Row<'t>) -> InsertCursor<'a, 't> {
        InsertCursor {
            map: &self,
            row: row,
            index: 0, 
        }
    }
}

impl<'a, 't> InsertCursor<'a, 't> {
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
    pub fn id(&self) -> ViewRowTypeId {
        self.id
    }

    pub(crate) fn entity_type_id(&self) -> ViewTypeId {
        self.view_type_id
    }

    pub(crate) fn row_type_id(&self) -> RowTypeId {
        self.row_type_id
    }
}

impl ViewRowType {
    pub fn new(
        id: ViewRowTypeId, 
        row: &RowType, 
        entity: &ViewType
    ) -> ViewRowType {
        let mut columns = Vec::<usize>::new();

        for col in &entity.cols {
            let (index, _) = row.columns().enumerate()
                .find(|(_, col_type)| {
                    col_type.id() == *col
            }).expect("entity column missing in row");

            columns.push(index);
        }

        ViewRowType {
            id,
            view_type_id: entity.id,
            row_type_id: row.id(),
            columns: columns,
        }
    }

    pub fn columns(&self) -> &Vec<usize> {
        &self.columns
    }
}

pub(crate) struct QueryPlan {
    view: ViewTypeId,
    cols: Vec<usize>,
}

impl QueryPlan {
    pub(crate) fn new_cursor(&self) -> QueryCursor {
        QueryCursor {
            cols: &self.cols,
            index: 0,
        }
    }

    pub(crate) fn view(&self) -> ViewTypeId {
        self.view
    }
}

impl<'a> QueryCursor<'a> {
    pub fn next(&mut self) -> usize {
        let index = self.index;
        self.index += 1;

        self.cols[index]
    }
}

impl<'a> QueryBuilder<'a> {
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

    pub(crate) fn build(self) -> QueryPlan {
        let view_id = self.meta.add_view(self.cols.clone());
        let view = self.meta.get_view(view_id);

        let cols = self.cols.iter()
            .map(|col_id| view.column_position(*col_id).unwrap())
            .collect();

        QueryPlan {
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

            views: Vec::new(),
            view_map: HashMap::new(),

            view_rows: Vec::new(),
            view_row_map: HashMap::new(),
        }
    }

    pub fn column(&self, id: ColumnTypeId) -> &ColumnType {
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

                align: align,
                length: length,
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
                length: column_type.length(),
                align: column_type.align(),
                offset: 0,
            };
            item.offset = length;

            length += column_type.length; // TODO: align
            align = max(align, column_type.align); 

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
    }

    pub fn push_row(
        &mut self, 
        row_id: RowTypeId, 
        column_id: ColumnTypeId
    ) -> RowTypeId {
        let row_type = self.rows.get(row_id.index()).unwrap();

        let mut columns : Vec<ColumnTypeId> = row_type.columns.iter().map(
            |col| col.id()
        ).collect();
        columns.push(column_id);

        self.add_row(columns)
    }

    pub fn push_row_by_type<T:'static>(
        &mut self, 
        row_id: RowTypeId
    ) -> RowTypeId {
        let col_id = self.add_column::<T>();
        self.push_row(row_id, col_id)
    }

    fn fill_row_columns(&mut self, row_type_id: RowTypeId) {
        let col_ids : Vec<ColumnTypeId> = self.get_row_id(row_type_id)
            .columns()
            .map(|col| col.id())
            .collect();

        for col_item_id in &col_ids {
            let col_type = self.column_mut(*col_item_id);

            col_type.rows.push(row_type_id);
        }

        self.build_row_entities(row_type_id, &col_ids);
    }

    fn build_row_entities(
        &mut self, 
        row_type_id: RowTypeId, 
        col_ids: &Vec<ColumnTypeId>
    ) {
        let mut entities: Vec<ViewTypeId> = Vec::new();

        for entity_type in &self.views {
            if entity_type
                .cols
                .iter()
                .filter(|c| col_ids.iter().any(|c1| *c == c1))
                .count() == entity_type.cols.len() {
                entities.push(entity_type.id());
            }
        }

        for entity_id in entities {
            self.add_view_row(row_type_id, entity_id);
        }
    }

    fn push_row_type(&mut self, row_type: RowType) {
        let row_type_id = row_type.id();

        for col in &row_type.columns {
            let col_type = self.columns.get_mut(col.id().index()).unwrap();

            col_type.rows.push(row_type_id);
        }

        self.rows.push(row_type);
    }

    pub fn single_row_type<T:'static>(&mut self) -> RowTypeId {
        let column_id = self.add_column::<T>();
        let mut columns = Vec::<ColumnTypeId>::new();
        columns.push(column_id);

        self.add_row(columns)
    }

    pub fn col_rows(&self, col: ColumnTypeId) -> Iter<RowTypeId> {
        self.columns.get(col.index()).unwrap().rows.iter()
    }

    pub fn col_join_rows(&self, cols: Vec<ColumnTypeId>) -> Vec<RowTypeId> {
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

        let mut rows : Vec<RowTypeId> = rows.iter().map(|row| *row).collect();
        rows.sort();

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
        row_id: RowTypeId, 
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
        row_id: RowTypeId,
        view_id: ViewTypeId, 
        view_row_id: ViewRowTypeId
    ) {
        let row = self.get_row_id(row_id);
        let view_type = self.get_view(view_id);

        assert_eq!(view_row_id.index(), self.view_rows.len());

        self.view_rows.push(ViewRowType::new(view_row_id, row, view_type));

        let view_type = self.get_mut_view(view_id);
        view_type.view_rows.push(view_row_id);
    }

    fn row_type(
        &mut self, 
        entity_id: ViewTypeId, 
        row_id: RowTypeId, 
        type_id: ViewRowTypeId
    ) -> ViewRowType {
        let entity_type = self.get_view(entity_id);

        let row = self.get_row_id(row_id);

        ViewRowType::new(type_id, row, entity_type)
    }

    pub fn view_row_cols(
        &mut self, 
        row_type_id: RowTypeId, 
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
    use std::mem;

    use crate::store::row_meta::{ColumnTypeId, RowTypeId, ColumnType, ColumnItem, ViewTypeId, ViewRowTypeId};

    use super::RowMetas;

    #[test]
    fn add_column() {
        let mut meta = RowMetas::new();

        let col_id = meta.add_column::<TestA>();
        let col_type = meta.column(col_id);
        assert_eq!(col_type.id(), ColumnTypeId(0));
        assert_eq!(col_type.length(), mem::size_of::<usize>());
        assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.views.len(), 0);

        let col_id = meta.add_column::<TestB>();
        let col_type = meta.column(col_id);
        assert_eq!(col_type.id(), ColumnTypeId(1));
        assert_eq!(col_type.length(), mem::size_of::<usize>());
        assert_eq!(col_type.align(), mem::align_of::<usize>());
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
        assert_eq!(type_a_id, RowTypeId(0));

        let type_a = meta.get_row_id(type_a_id);
        assert_eq!(type_a.id(), RowTypeId(0));
        assert_eq!(type_a.align(), mem::align_of::<TestA>());
        assert_eq!(type_a.length(), mem::size_of::<TestA>());
        let cols : Vec<&ColumnItem> = type_a.columns().collect();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].id(), ColumnTypeId(0));
        assert_eq!(cols[0].align(), mem::align_of::<usize>());
        assert_eq!(cols[0].length(), mem::size_of::<usize>());
        assert_eq!(cols[0].offset(), 0);

        let col_a = meta.column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.views.len(), 0);

        let type_a_id = meta.single_row_type::<TestA>();
        assert_eq!(type_a_id, RowTypeId(0));

        let type_a = meta.get_row_id(type_a_id);
        assert_eq!(type_a.id(), RowTypeId(0));
        assert_eq!(type_a.align(), mem::align_of::<TestA>());
        assert_eq!(type_a.length(), mem::size_of::<TestA>());
        let cols : Vec<&ColumnItem> = type_a.columns().collect();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].id(), ColumnTypeId(0));
        assert_eq!(cols[0].align(), mem::align_of::<usize>());
        assert_eq!(cols[0].length(), mem::size_of::<usize>());
        assert_eq!(cols[0].offset(), 0);

        let col_a = meta.column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.views.len(), 0);

        let type_b_id = meta.single_row_type::<TestB>();
        assert_eq!(type_b_id, RowTypeId(1));

        let type_b = meta.get_row_id(type_b_id);
        assert_eq!(type_b.id(), RowTypeId(1));
        assert_eq!(type_b.align(), mem::align_of::<TestB>());
        assert_eq!(type_b.length(), mem::size_of::<TestB>());
        let cols : Vec<&ColumnItem> = type_b.columns().collect();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].id(), ColumnTypeId(1));
        assert_eq!(cols[0].align(), mem::align_of::<usize>());
        assert_eq!(cols[0].length(), mem::size_of::<usize>());
        assert_eq!(cols[0].offset(), 0);

        let col_b = meta.column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.views.len(), 0);
    }

    #[test]
    fn push_row() {
        let mut meta = RowMetas::new();

        let type_a_id = meta.single_row_type::<TestA>();
        assert_eq!(type_a_id, RowTypeId(0));

        let type_a = meta.get_row_id(type_a_id);
        assert_eq!(type_a.id(), RowTypeId(0));
        assert_eq!(type_a.align(), mem::align_of::<TestA>());
        assert_eq!(type_a.length(), mem::size_of::<TestA>());

        let type_aa_id = meta.push_row_by_type::<TestA>(type_a_id);
        assert_eq!(type_aa_id, RowTypeId(0));

        let type_aa = meta.get_row_id(type_aa_id);
        assert_eq!(type_aa.id(), RowTypeId(0));
        assert_eq!(type_aa.align(), mem::align_of::<TestA>());
        assert_eq!(type_aa.length(), mem::size_of::<TestA>());

        let cols : Vec<&ColumnItem> = type_aa.columns().collect();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].id(), ColumnTypeId(0));
        assert_eq!(cols[0].align(), mem::align_of::<usize>());
        assert_eq!(cols[0].length(), mem::size_of::<usize>());
        assert_eq!(cols[0].offset(), 0);

        let type_b_id = meta.single_row_type::<TestB>();
        assert_eq!(type_b_id, RowTypeId(1));

        let type_ab_id = meta.push_row_by_type::<TestB>(type_a_id);
        assert_eq!(type_ab_id, RowTypeId(2));

        let type_ab = meta.get_row_id(type_ab_id);
        assert_eq!(type_ab.id(), RowTypeId(2));
        assert_eq!(type_ab.align(), mem::align_of::<TestA>());
        assert_eq!(type_ab.length(), 2 * mem::size_of::<TestA>());

        let cols : Vec<&ColumnItem> = type_ab.columns().collect();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].id(), ColumnTypeId(0));
        assert_eq!(cols[0].align(), mem::align_of::<usize>());
        assert_eq!(cols[0].length(), mem::size_of::<usize>());
        assert_eq!(cols[0].offset(), 0);
        assert_eq!(cols[1].id(), ColumnTypeId(1));
        assert_eq!(cols[1].align(), mem::align_of::<usize>());
        assert_eq!(cols[1].length(), mem::size_of::<usize>());
        assert_eq!(cols[1].offset(), mem::size_of::<usize>());

        let col_a = meta.column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 2);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.rows[1], RowTypeId(2));

        let col_b = meta.column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 2);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.rows[1], RowTypeId(2));

        let type_aba = meta.push_row_by_type::<TestA>(type_ab_id);
        assert_eq!(type_aba, RowTypeId(2));

        let type_ba = meta.push_row_by_type::<TestA>(type_b_id);
        assert_eq!(type_ba, RowTypeId(2));
    }

    #[test]
    fn row_cols() {
        let mut meta = RowMetas::new();

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, RowTypeId(0));

        let col_a = meta.add_column::<TestA>();

        let row_type = meta.get_row_id(type_a);
        assert_eq!(row_type.id(), type_a);
        assert_eq!(row_type.columns().len(), 1);
        let cols : Vec<&ColumnItem> = row_type.columns().collect();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].id(), col_a);

        let type_b = meta.single_row_type::<TestB>();
        assert_eq!(type_b, RowTypeId(1));

        let col_b = meta.add_column::<TestB>();

        let type_ba = meta.push_row_by_type::<TestA>(type_b);
        assert_eq!(type_ba, RowTypeId(2));

        let row_type = meta.get_row_id(type_ba);
        assert_eq!(row_type.id(), type_ba);
        assert_eq!(row_type.columns().len(), 2);
        let cols : Vec<&ColumnItem> = row_type.columns().collect();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].id(), col_a);
        assert_eq!(cols[1].id(), col_b);
    }

    #[test]
    fn col_rows() {
        let mut meta = RowMetas::new();

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, RowTypeId(0));

        let col_a = meta.add_column::<TestA>();
        let rows: Vec<RowTypeId> = meta.col_rows(col_a).map(|id| *id).collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], RowTypeId(0));

        let col_c = meta.add_column::<TestC>();
        let col_b = meta.add_column::<TestB>();

        let type_c = meta.single_row_type::<TestC>();
        assert_eq!(type_c, RowTypeId(1));

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
    }

    #[test]
    fn row_then_view() {
        let mut meta = RowMetas::new();

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, RowTypeId(0));

        let view_id_a = meta.single_view_type::<TestA>();
        assert_eq!(view_id_a, ViewTypeId(0));

        let view_a = meta.get_view(view_id_a);
        assert_eq!(view_a.id(), ViewTypeId(0));
        let cols = &view_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));

        let col_a = meta.column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.views.len(), 1);
        assert_eq!(col_a.views[0], ViewTypeId(0));

        let entity_row_a = meta.get_view_row(ViewRowTypeId(0));
        assert_eq!(entity_row_a.id(), ViewRowTypeId(0));
        assert_eq!(entity_row_a.row_type_id(), RowTypeId(0));
        assert_eq!(entity_row_a.entity_type_id(), ViewTypeId(0));
    }

    #[test]
    fn view_then_row() {
        let mut meta = RowMetas::new();

        let entity_id_a = meta.single_view_type::<TestA>();
        assert_eq!(entity_id_a, ViewTypeId(0));

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, RowTypeId(0));

        //meta.push_row(row_id_a, col_id_b);

        let entity_a = meta.get_view(entity_id_a);
        assert_eq!(entity_a.id(), ViewTypeId(0));
        let cols = &entity_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));
        let rows = &entity_a.view_rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], ViewRowTypeId(0));

        let col_a = meta.column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.views.len(), 1);
        assert_eq!(col_a.views[0], ViewTypeId(0));

        let entity_row_a = meta.get_view_row(ViewRowTypeId(0));
        assert_eq!(entity_row_a.id(), ViewRowTypeId(0));
        assert_eq!(entity_row_a.row_type_id(), RowTypeId(0));
        assert_eq!(entity_row_a.entity_type_id(), ViewTypeId(0));
    }

    #[test]
    fn view_then_row2() {
        let mut meta = RowMetas::new();

        let entity_id_a = meta.single_view_type::<TestA>();
        assert_eq!(entity_id_a, ViewTypeId(0));

        let entity_id_b = meta.single_view_type::<TestB>();
        assert_eq!(entity_id_b, ViewTypeId(1));

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, RowTypeId(0));

        let row_id_b = meta.single_row_type::<TestB>();
        assert_eq!(row_id_b, RowTypeId(1));

        let entity_b = meta.get_view(entity_id_b);
        assert_eq!(entity_b.id(), ViewTypeId(1));
        let cols = &entity_b.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(1));
        let rows = &entity_b.view_rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], ViewRowTypeId(1));

        let col_b = meta.column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.views.len(), 1);
        assert_eq!(col_b.views[0], ViewTypeId(1));

        let entity_row_b = meta.get_view_row(ViewRowTypeId(1));
        assert_eq!(entity_row_b.id(), ViewRowTypeId(1));
        assert_eq!(entity_row_b.row_type_id(), RowTypeId(1));
        assert_eq!(entity_row_b.entity_type_id(), ViewTypeId(1));
    }


    struct TestA(usize);
    struct TestB(usize);
    struct TestC(usize);
}