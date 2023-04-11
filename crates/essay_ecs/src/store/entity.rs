use super::{prelude::RowId, meta::ColumnTypeId};

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityId2(usize);

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityId(usize);

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityTypeId(usize);

pub struct EntityRow {
    id: EntityId,

    columns: Vec<RowId>,
}

pub struct EntityGroup {
    id: EntityTypeId,

    columns: Vec<ColumnTypeId>,
    entities: Vec<EntityRow>,
}

