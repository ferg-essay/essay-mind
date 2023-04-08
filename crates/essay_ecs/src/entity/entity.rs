use std::marker::PhantomData;

use crate::store::{prelude::{RowId, RowTypeId}, row_meta::ViewRowTypeId};

use super::prelude::EntityTable;


pub struct EntityRef<T> {
    row_id: RowId,
    row_type: RowTypeId,
    entity_row_type: ViewRowTypeId,
    marker: PhantomData<T>,
}

impl<T:'static> EntityRef<T> {
    pub(crate) fn new(
        row_id: RowId, 
        row_type: RowTypeId,
        entity_row_type: ViewRowTypeId
    ) -> Self {
        Self {
            row_id: row_id,
            row_type: row_type,
            entity_row_type: entity_row_type,
            marker: PhantomData,
        }
    }

    pub fn get<'a>(&self, table: &'a EntityTable) -> Option<&'a T> {
        todo!();
        /*
        let row_type = table.entity_meta.get_entity_row(self.entity_row_type);
            
        table.table.get_row(self.row_id, row_type.columns())
        */
    }

    pub fn push<S:'static>(&self, table: &mut EntityTable, value: S) {
        todo!();
        //table.table.replace_push(self.row_id, value);
    }
}
