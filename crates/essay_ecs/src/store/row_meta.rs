use std::{mem, collections::{HashMap, HashSet, hash_set}, cmp::max, slice::Iter, any::TypeId};

use super::prelude::TypeMetas;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColumnTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RowTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityTypeId(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityRowTypeId(usize);

#[derive(Clone, Debug)]
pub struct ColumnType {
    id: ColumnTypeId,
    align: usize,
    length: usize,

    rows: Vec<RowTypeId>,
    entities: Vec<EntityTypeId>,
}

#[derive(Clone, Debug)]
pub struct ColumnItem {
    id: ColumnTypeId,
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

pub struct EntityType {
    id: EntityTypeId,
    cols: Vec<ColumnTypeId>,

    row_types: Vec<RowTypeId>,
}

pub struct EntityRowType {
    id: EntityRowTypeId,

    entity_type_id: EntityTypeId,
    row_type_id: RowTypeId,

    columns: Vec<usize>,
}

pub(crate) struct RowMetas {
    col_type_metas: TypeMetas,
    col_types: Vec<ColumnType>,
    col_type_rows: Vec<Vec<RowTypeId>>,


    //row_type_metas: TypeMetas,
    row_col_map: HashMap<Vec<ColumnTypeId>,RowTypeId>,
    row_type_map: HashMap<TypeId,RowTypeId>,
    row_types: Vec<RowType>,

    entity_types: Vec<EntityType>,
    entity_col_map: HashMap<Vec<ColumnTypeId>,EntityTypeId>,

    entity_row_types: Vec<EntityRowType>,
    entity_row_map: HashMap<(EntityTypeId,RowTypeId), EntityRowTypeId>,

    entity_rows: Vec<Vec<EntityRowTypeId>>,
}

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
        self.id
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
}

impl EntityTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl EntityType {
    pub fn id(&self) -> EntityTypeId {
        self.id
    }
}

impl EntityRowTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl EntityRowType {
    pub fn id(&self) -> EntityRowTypeId {
        self.id
    }

    pub(crate) fn entity_type_id(&self) -> EntityTypeId {
        self.entity_type_id
    }

    pub(crate) fn row_type_id(&self) -> RowTypeId {
        self.row_type_id
    }

    /*
    pub(crate) unsafe fn get_fun<'a,F,R:'static>(
        &'a self, 
        table: &'a Table<'a>,
        row_id: RowId, 
        mut fun: F
    ) -> &'a R
    where F: FnMut(&'a Row, &Vec<usize>) -> &'a R {
        table.get_fun(row_id, &self.columns, fun)
    }
    */

    //pub(crate) fn 
}

impl EntityRowType {
    pub fn new(
        id: EntityRowTypeId, 
        row: &RowType, 
        entity: &EntityType
    ) -> EntityRowType {
        let mut columns = Vec::<usize>::new();

        for col in &entity.cols {
            let (index, _) = row.columns().enumerate()
                .find(|(_, col_type)| {
                    col_type.id() == *col
            }).expect("entity column missing in row");

            columns.push(index);
        }

        EntityRowType {
            id,
            entity_type_id: entity.id,
            row_type_id: row.id(),
            columns: columns,
        }
    }

    pub fn columns(&self) -> &Vec<usize> {
        &self.columns
    }
}

impl RowMetas {
    pub fn new() -> Self {
        Self {
            col_type_metas: TypeMetas::new(),
            col_types: Vec::new(),

            row_col_map: HashMap::new(),
            row_type_map: HashMap::new(),
            row_types: Vec::new(),

            col_type_rows: Vec::new(),

            entity_types: Vec::new(),
            entity_col_map: HashMap::new(),

            entity_row_types: Vec::new(),
            entity_row_map: HashMap::new(),

            entity_rows: Vec::new(),
        }
    }

    pub fn add_column<T:'static>(&mut self) -> &ColumnType {
        let type_index = self.col_type_metas.add_type::<T>();

        if self.col_types.len() <= type_index.index() {
            assert!(type_index.index() == self.col_types.len());

            let align = mem::align_of::<T>();
            let length = mem::size_of::<T>();

            let col_type = ColumnType {
                id: ColumnTypeId(type_index.index()),
                align: align,
                length: length,
                rows: Vec::new(),
                entities: Vec::new(),
            };

            self.push_col(col_type);
        }

        return self.col_types.get(type_index.index()).unwrap();
    }

    fn push_col(&mut self, col_type: ColumnType) {
        self.col_types.push(col_type);
        self.col_type_rows.push(Vec::new());
    }

    pub fn get_column_type<T:'static>(&self) -> Option<&ColumnType> {
        match self.col_type_metas.get_id::<T>() {
            Some(type_id) => { 
                self.col_types.get(type_id.index())
            },
            None => None,
        }
    }

    pub(crate) fn get_column_type_id<T:'static>(&self) -> Option<ColumnTypeId> {
        match self.col_type_metas.get_id::<T>() {
            Some(column_type_id) => {
                Some(ColumnTypeId(column_type_id.index()))
            },
            None => None,
        }
    }

    pub fn get_column(&self, id: ColumnTypeId) -> &ColumnType {
        self.col_types.get(id.index()).unwrap()
    }

    pub fn get_mut_column(&mut self, id: ColumnTypeId) -> &mut ColumnType {
        self.col_types.get_mut(id.index()).unwrap()
    }

    pub fn push_row(
        &mut self, 
        row_id: RowTypeId, 
        column_id: ColumnTypeId
    ) -> RowTypeId {
        let row_type = self.row_types.get(row_id.index()).unwrap();

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
        let col_id = self.add_column::<T>().id();
        self.push_row(row_id, col_id)
    }

    pub fn add_row(&mut self, mut columns: Vec<ColumnTypeId>) -> RowTypeId {
        columns.sort();
        columns.dedup();

        let mut length: usize = 0;
        let mut align: usize = 1;

        let mut column_items = Vec::<ColumnItem>::new();

        for column_id in &columns {
            let column_type = self.col_types.get(column_id.0).unwrap();

            let mut col = ColumnItem {
                id: column_type.id(),
                length: column_type.length(),
                align: column_type.align(),
                offset: 0,
            };
            col.offset = length;

            length += column_type.length; // TODO: align
            align = max(align, column_type.align); 

            column_items.push(col);
        }

        //let type_id = self.row_type_metas.add_type::<T>();
        let len = self.row_col_map.len();
        let row_type_id = self.row_col_map.entry(columns.clone()).or_insert_with(|| {
            RowTypeId(len)
        });
        let row_type_id = *row_type_id;

        if row_type_id.index() == self.row_types.len() {
            self.push_row_type(RowType {
                id: row_type_id,
                columns: column_items,
                length: length,
                align: align,
            });

            self.fill_row_columns(row_type_id);
        }

        row_type_id
    }

    fn fill_row_columns(&mut self, row_type_id: RowTypeId) {
        let col_ids : Vec<ColumnTypeId> = self.get_row_id(row_type_id)
            .columns()
            .map(|col| col.id())
            .collect();

        for col_item_id in &col_ids {
            let col_type = self.get_mut_column(*col_item_id);

            col_type.rows.push(row_type_id);
        }

        self.build_row_entities(row_type_id, &col_ids);
    }

    fn build_row_entities(
        &mut self, 
        row_type_id: RowTypeId, 
        col_ids: &Vec<ColumnTypeId>
    ) {
        let mut entities: Vec<EntityTypeId> = Vec::new();

        for entity_type in &self.entity_types {
            if entity_type
                .cols
                .iter()
                .filter(|c| col_ids.iter().any(|c1| *c == c1))
                .count() == entity_type.cols.len() {
                entities.push(entity_type.id());
            }
        }

        for entity_id in entities {
            self.add_entity_row(row_type_id, entity_id);
        }
    }

    fn push_row_type(&mut self, row_type: RowType) {
        let row_type_id = row_type.id();

        for col in &row_type.columns {
            let col_rows = self.col_type_rows.get_mut(col.id().index()).unwrap();

            col_rows.push(row_type_id);
        }

        self.row_types.push(row_type);
    }

    pub fn add_row_type<T:'static>(&mut self, row_type: RowTypeId) -> RowTypeId {
        let type_id = TypeId::of::<T>();

        self.row_type_map.insert(type_id, row_type);

        row_type
    }

    pub fn get_row_by_type<T:'static>(&self) -> Option<&RowType> {
        match self.row_type_map.get(&TypeId::of::<T>()) {
            Some(row_id) => {
                self.row_types.get(row_id.index())
            },
            None => None,
        }
    }

    pub fn get_row_id(&self, row_type_id: RowTypeId) -> &RowType {
        self.row_types.get(row_type_id.index()).unwrap()
    }

    pub fn single_row_type<T:'static>(&mut self) -> RowTypeId {
        let column_type = self.add_column::<T>();
        let mut col_vec = Vec::<ColumnTypeId>::new();
        col_vec.push(column_type.id());

        self.add_row(col_vec)
    }

    pub fn col_rows(&self, col: ColumnTypeId) -> Iter<RowTypeId> {
        self.col_type_rows.get(col.index()).unwrap().iter()
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

    pub fn entity_type(&mut self, cols: Vec<ColumnTypeId>) -> EntityTypeId {
        let len = self.entity_types.len();

        let type_id = *self.entity_col_map
            .entry(cols.clone())
            .or_insert_with(|| {
            EntityTypeId(len)
        });

        if type_id.0 == len {
            self.entity_types.push(EntityType {
                id: type_id,
                cols: cols,
                row_types: Vec::new(),
            });

            self.entity_rows.push(Vec::new());

            self.fill_entity(type_id);
        }

        type_id
    }

    pub fn fill_entity(&mut self, entity_type_id: EntityTypeId) {
        let entity_type = self.get_entity_type(entity_type_id);
        let cols = entity_type.cols.clone();

        for col in &cols {
            let col_type = self.get_mut_column(*col);

            col_type.entities.push(entity_type_id);
        }

        let rows : Vec<RowTypeId> = self.row_types.iter().map(|row| row.id()).collect();
         
         let mut match_rows = Vec::<RowTypeId>::new();

        for row_id in rows {
            let row_type = self.get_row_id(row_id);

            if cols.iter().filter(|col| cols.contains(col)).count() == cols.len() {
                match_rows.push(row_type.id());
            }

        }

        for row_id in match_rows {
            self.add_entity_row(row_id, entity_type_id);
        }
    }

    pub fn single_entity_type<T:'static>(&mut self) -> EntityTypeId {
        let column_type = self.add_column::<T>();
        let mut col_vec = Vec::<ColumnTypeId>::new();
        col_vec.push(column_type.id());

        self.entity_type(col_vec)
    }

    pub(crate) fn get_entity_type_cols(&self, cols: &Vec<ColumnTypeId>) -> Option<EntityTypeId> {
        match self.entity_col_map.get(cols) {
            Some(type_id) => Some(*type_id),
            None => None,
        }
    }

    pub fn get_entity_type(&self, id: EntityTypeId) -> &EntityType {
        self.entity_types.get(id.index()).unwrap()
    }

    fn get_mut_entity_type(&mut self, id: EntityTypeId) -> &mut EntityType {
        self.entity_types.get_mut(id.index()).unwrap()
    }

    /*
    pub fn entity_row(
        &mut self, 
        row_id: RowTypeId, 
        entity_id: EntityTypeId
    ) -> EntityRowTypeId {
        let len = self.entity_row_types.len();

        let type_id = self.entity_row_map
            .entry((entity_id, row_id))
            .or_insert_with(|| {
            EntityRowTypeId(len)
        });

        let type_id = *type_id;

        if type_id.index() == len {
            // let entity_type = self.get_entity_type(entity_id).expect("entity-type");
            let row_type = self.row_type(entity_id, row_id, type_id);

            self.entity_row_types.push(row_type);
        }

        // self.entity_row_types.get(type_id.index()).expect("known entity type")
        type_id
    }
     */

    fn add_entity_row(
        &mut self,
        row_id: RowTypeId, 
        entity_id: EntityTypeId
    ) -> EntityRowTypeId {
        let len = self.entity_row_types.len();

        let type_id = self.entity_row_map
            .entry((entity_id, row_id))
            .or_insert_with(|| {
            EntityRowTypeId(len)
        });

        let type_id = *type_id;

        if type_id.index() == len {
            // let entity_type = self.get_entity_type(entity_id).expect("entity-type");
            self.push_entity_row(row_id, entity_id, type_id);
        }

        // self.entity_row_types.get(type_id.index()).expect("known entity type")
        type_id
    }

    fn push_entity_row(
        &mut self, 
        row_type_id: RowTypeId,
        entity_id: EntityTypeId, 
        type_id: EntityRowTypeId
    ) {
        let row = self.get_row_id(row_type_id);
        let row_id = row.id();
        let entity_type = self.get_entity_type(entity_id);

        assert_eq!(type_id.index(), self.entity_row_types.len());

        self.entity_row_types.push(EntityRowType::new(type_id, row, entity_type));

        let entity_type = self.get_mut_entity_type(entity_id);
        entity_type.row_types.push(row_id);
    }

    fn row_type(
        &mut self, 
        entity_id: EntityTypeId, 
        row_id: RowTypeId, 
        type_id: EntityRowTypeId
    ) -> EntityRowType {
        let entity_type = self.get_entity_type(entity_id);

        let row = self.get_row_id(row_id);

        EntityRowType::new(type_id, row, entity_type)
    }

    pub fn entity_row_cols(
        &mut self, 
        row_type_id: RowTypeId, 
        columns: Vec<ColumnTypeId>
    ) -> EntityRowTypeId {
        let entity_type_id = self.entity_type(columns);
        //let entity_type = self.entity_types.get(entity_type_id.index()).unwrap();

        self.add_entity_row(row_type_id, entity_type_id)
    }


    pub fn get_entity_row(&self, id: EntityRowTypeId) -> &EntityRowType {
        self.entity_row_types.get(id.index()).unwrap()
    }

    pub fn get_entity_rows(&self, id: EntityTypeId) -> &Vec<EntityRowTypeId> {
        self.entity_rows.get(id.index()).unwrap()
    }

    /*
    pub(crate) fn push_entity(
        &self, 
        entity_row_type: EntityRowTypeId, 
        col_type_id: ColumnTypeId
    ) -> _ {
        let entity_row = self.get_entity_row(entity_row_type).expect("row");

        let entity_type_id = self.push_entity_type(
            entity_row.entity_type_id, 
            col_type_id
        );

        let row_type_id = entity_row.row_type_id;
        let row_type = self.get_row_type(row_type_id).expect("expect");

        let mut cols = entity_type.cols.clone();
        cols.push(col_type_id);

        todo!()
    }
    */

    pub(crate) fn push_entity_type(
        &mut self, 
        entity_type_id: EntityTypeId, 
        col_type_id: ColumnTypeId
    ) -> EntityTypeId {
        let entity_type = self.get_entity_type(entity_type_id);

        let mut cols = entity_type.cols.clone();
        cols.push(col_type_id);

        self.entity_type(cols)
    }

    pub fn entity_rows(&self, entity_type: EntityTypeId) -> &Vec<EntityRowTypeId> {
        self.entity_rows.get(entity_type.index()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::store::row_meta::{ColumnTypeId, RowTypeId, ColumnType, ColumnItem, EntityTypeId, EntityRowTypeId};

    use super::RowMetas;

    #[test]
    fn add_column() {
        let mut meta = RowMetas::new();

        let col_type = meta.add_column::<TestA>();
        assert_eq!(col_type.id(), ColumnTypeId(0));
        assert_eq!(col_type.length(), mem::size_of::<usize>());
        assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.entities.len(), 0);

        let col_type = meta.add_column::<TestB>();
        assert_eq!(col_type.id(), ColumnTypeId(1));
        assert_eq!(col_type.length(), mem::size_of::<usize>());
        assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.rows.len(), 0);
        assert_eq!(col_type.entities.len(), 0);

        // check double add
        let col_type = meta.add_column::<TestA>();
        assert_eq!(col_type.id(), ColumnTypeId(0));
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

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.entities.len(), 0);

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

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.entities.len(), 0);

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

        let col_b = meta.get_column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.entities.len(), 0);
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

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 2);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.rows[1], RowTypeId(2));

        let col_b = meta.get_column(ColumnTypeId(1));
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

        let col_a = meta.add_column::<TestA>().id();

        let row_type = meta.get_row_id(type_a);
        assert_eq!(row_type.id(), type_a);
        assert_eq!(row_type.columns().len(), 1);
        let cols : Vec<&ColumnItem> = row_type.columns().collect();
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0].id(), col_a);

        let type_b = meta.single_row_type::<TestB>();
        assert_eq!(type_b, RowTypeId(1));

        let col_b = meta.add_column::<TestB>().id();

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

        let col_a = meta.add_column::<TestA>().id();
        let rows: Vec<RowTypeId> = meta.col_rows(col_a).map(|id| *id).collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], RowTypeId(0));

        let col_c = meta.add_column::<TestC>().id();
        let col_b = meta.add_column::<TestB>().id();

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
    fn row_then_entity() {
        let mut meta = RowMetas::new();

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, RowTypeId(0));

        let entity_id_a = meta.single_entity_type::<TestA>();
        assert_eq!(entity_id_a, EntityTypeId(0));

        let entity_a = meta.get_entity_type(entity_id_a);
        assert_eq!(entity_a.id(), EntityTypeId(0));
        let cols = &entity_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.entities.len(), 1);
        assert_eq!(col_a.entities[0], EntityTypeId(0));

        let entity_row_a = meta.get_entity_row(EntityRowTypeId(0));
        assert_eq!(entity_row_a.id(), EntityRowTypeId(0));
        assert_eq!(entity_row_a.row_type_id(), RowTypeId(0));
        assert_eq!(entity_row_a.entity_type_id(), EntityTypeId(0));
    }

    #[test]
    fn entity_then_row() {
        let mut meta = RowMetas::new();

        let entity_id_a = meta.single_entity_type::<TestA>();
        assert_eq!(entity_id_a, EntityTypeId(0));

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, RowTypeId(0));

        //meta.push_row(row_id_a, col_id_b);

        let entity_a = meta.get_entity_type(entity_id_a);
        assert_eq!(entity_a.id(), EntityTypeId(0));
        let cols = &entity_a.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(0));
        let rows = &entity_a.row_types;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], RowTypeId(0));

        let col_a = meta.get_column(ColumnTypeId(0));
        assert_eq!(col_a.rows.len(), 1);
        assert_eq!(col_a.rows[0], RowTypeId(0));
        assert_eq!(col_a.entities.len(), 1);
        assert_eq!(col_a.entities[0], EntityTypeId(0));

        let entity_row_a = meta.get_entity_row(EntityRowTypeId(0));
        assert_eq!(entity_row_a.id(), EntityRowTypeId(0));
        assert_eq!(entity_row_a.row_type_id(), RowTypeId(0));
        assert_eq!(entity_row_a.entity_type_id(), EntityTypeId(0));
    }

    #[test]
    fn entity_then_row2() {
        let mut meta = RowMetas::new();

        let entity_id_a = meta.single_entity_type::<TestA>();
        assert_eq!(entity_id_a, EntityTypeId(0));

        let entity_id_b = meta.single_entity_type::<TestB>();
        assert_eq!(entity_id_b, EntityTypeId(1));

        let row_id_a = meta.single_row_type::<TestA>();
        assert_eq!(row_id_a, RowTypeId(0));

        let row_id_b = meta.single_row_type::<TestB>();
        assert_eq!(row_id_b, RowTypeId(1));

        //meta.push_row(row_id_a, col_id_b);

        let entity_b = meta.get_entity_type(entity_id_b);
        assert_eq!(entity_b.id(), EntityTypeId(1));
        let cols = &entity_b.cols;
        assert_eq!(cols.len(), 1);
        assert_eq!(cols[0], ColumnTypeId(1));
        let rows = &entity_b.row_types;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0], RowTypeId(1));

        let col_b = meta.get_column(ColumnTypeId(1));
        assert_eq!(col_b.rows.len(), 1);
        assert_eq!(col_b.rows[0], RowTypeId(1));
        assert_eq!(col_b.entities.len(), 1);
        assert_eq!(col_b.entities[0], EntityTypeId(1));

        let entity_row_b = meta.get_entity_row(EntityRowTypeId(1));
        assert_eq!(entity_row_b.id(), EntityRowTypeId(1));
        assert_eq!(entity_row_b.row_type_id(), RowTypeId(1));
        assert_eq!(entity_row_b.entity_type_id(), EntityTypeId(1));
    }


    struct TestA(usize);
    struct TestB(usize);
    struct TestC(usize);
}