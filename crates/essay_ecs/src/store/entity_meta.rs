use std::{collections::HashMap};

use super::{prelude::Table, row::{RowId, Row}, row_meta::{ColumnTypeId, RowTypeId, RowType}};
/*
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityTypeId(usize);

pub struct EntityMeta {
    entity_types: Vec<EntityType>,
    column_entity_map: HashMap<Vec<ColumnTypeId>,EntityTypeId>,

    entity_row_types: Vec<EntityRowType>,
    entity_row_map: HashMap<(EntityTypeId,RowTypeId), EntityRowTypeId>,

    entity_rows: Vec<Vec<EntityRowTypeId>>,
}

pub struct EntityType {
    id: EntityTypeId,
    cols: Vec<ColumnTypeId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityRowTypeId(usize);

pub struct EntityRowType {
    type_id: EntityRowTypeId,

    entity_type_id: EntityTypeId,
    row_type_id: RowTypeId,

    columns: Vec<usize>,
}

impl EntityTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl EntityRowTypeId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl EntityRowType {
    pub fn id(&self) -> EntityRowTypeId {
        self.type_id
    }

    pub(crate) fn entity_type_id(&self) -> EntityTypeId {
        self.entity_type_id
    }

    pub(crate) fn row_type_id(&self) -> RowTypeId {
        self.row_type_id
    }

    pub(crate) unsafe fn get_fun<'a,F,R:'static>(
        &'a self, 
        table: &'a Table<'a>,
        row_id: RowId, 
        mut fun: F
    ) -> &'a R
    where F: FnMut(&'a Row, &Vec<usize>) -> &'a R {
        table.get_fun(row_id, &self.columns, fun)
    }

    //pub(crate) fn 
}

impl EntityMeta {
    pub fn new() -> Self {
        EntityMeta {
            entity_types: Vec::new(),
            column_entity_map: HashMap::new(),

            entity_row_types: Vec::new(),
            entity_row_map: HashMap::new(),

            entity_rows: Vec::new(),
        }
    }

    pub fn entity_type(&mut self, cols: Vec<ColumnTypeId>) -> EntityTypeId {
        let len = self.entity_types.len();

        let type_id = self.column_entity_map
            .entry(cols.clone())
            .or_insert_with(|| {
            EntityTypeId(len)
        });

        if type_id.0 == len {
            self.entity_types.push(EntityType {
                id: *type_id,
                cols: cols,
            });

            self.entity_rows.push(Vec::new());
        }

        *type_id
    }

    pub(crate) fn get_entity_type_cols(&self, cols: &Vec<ColumnTypeId>) -> Option<EntityTypeId> {
        match self.column_entity_map.get(cols) {
            Some(type_id) => Some(*type_id),
            None => None,
        }
    }


    pub fn get_entity_type(&self, id: EntityTypeId) -> &EntityType {
        self.entity_types.get(id.index()).unwrap()
    }

    pub fn entity_row(&mut self, row: &RowType, entity_id: EntityTypeId) -> EntityRowTypeId {
        let len = self.entity_row_types.len();

        let type_id = self.entity_row_map
            .entry((entity_id, row.id()))
            .or_insert_with(|| {
            EntityRowTypeId(len)
        });

        let type_id = *type_id;

        if type_id.index() == len {
            // let entity_type = self.get_entity_type(entity_id).expect("entity-type");
            let row_type = self.row_type(entity_id, row, type_id);

            self.entity_row_types.push(row_type);
        }

        // self.entity_row_types.get(type_id.index()).expect("known entity type")
        type_id
    }

    fn row_type(
        &mut self, 
        entity_id: EntityTypeId, 
        row: &RowType, 
        type_id: EntityRowTypeId
    ) -> EntityRowType {
        let entity_type = self.get_entity_type(entity_id);

        EntityRowType::new(type_id, row, entity_type)
    }

    pub fn entity_row_cols(
        &mut self, 
        row_type: &RowType, 
        columns: Vec<ColumnTypeId>
    ) -> EntityRowTypeId {
        let entity_type_id = self.entity_type(columns);
        let entity_type = self.entity_types.get(entity_type_id.index()).unwrap();

        self.entity_row(row_type, entity_type_id)
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
            type_id: id,
            entity_type_id: entity.id,
            row_type_id: row.id(),
            columns: columns,
        }
    }

    pub fn columns(&self) -> &Vec<usize> {
        &self.columns
    }
}
*/