use std::marker::PhantomData;

use crate::{store::{prelude::{RowId, RowTypeId}, meta::{ViewRowTypeId, ColumnTypeId}}, prelude::Component};

use super::{prelude::EntityTable, component::ViewQuery};

pub struct EntityRef {
    row_id: RowId,
    row_type: RowTypeId,
    //entity_row_type: ViewRowTypeId,
}

impl EntityRef {
    pub(crate) fn new(
        row_id: RowId, 
        row_type: RowTypeId,
        //entity_row_type: ViewRowTypeId
    ) -> Self {
        Self {
            row_id: row_id,
            row_type: row_type,
            //entity_row_type: entity_row_type,
        }
    }
    /*
    pub fn id(&self) -> EntityId2 {
        EntityId2(self.row_id.index())
    }
    */

    pub fn get<'a,T:Component>(&self, table: &'a EntityTable) -> &'a T {
        table.get_row::<T>(self.row_id)
    }

    pub fn push<S:'static>(&self, table: &mut EntityTable, value: S) {
        todo!();
        //table.table.replace_push(self.row_id, value);
    }
}
