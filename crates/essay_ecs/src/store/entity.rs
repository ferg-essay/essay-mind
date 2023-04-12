use super::{prelude::RowId, meta::ColumnTypeId};

#[derive(Debug,Clone,Copy,PartialEq,Hash,PartialOrd,Eq)]
pub struct EntityId2(usize);
