use std::marker::PhantomData;

use super::{prelude::Table, row::RowId, entity_meta::{EntityRowTypeId, EntityMeta, EntityRowType}, row_meta::{ColumnTypeId, RowTypeId}};

pub struct EntityTable<'w> {
    table: Table<'w>,
    entity_meta: EntityMeta,
}

pub struct EntityRef<T> {
    row_id: RowId,
    entity_row_type: EntityRowTypeId,
    marker: PhantomData<T>,
}

impl<'w> EntityTable<'w> {
    pub fn new() -> Self {
        EntityTable {
            table: Table::new(),
            entity_meta: EntityMeta::new(),
        }
    }

    pub fn push<T:'static>(&mut self, value: T) -> EntityRef<T> {
        let row_ref = self.table.push(value);

        let type_id = self.entity_type::<T>(row_ref.row_type_id());

        EntityRef {
            row_id: row_ref.row_id(),
            entity_row_type: type_id,
            marker: PhantomData,
        }        
    }

    pub fn entity_type<T:'static>(&mut self, row_type: RowTypeId) -> EntityRowTypeId {
        let col_type = self.table.column_type::<T>();

        let mut columns = Vec::<ColumnTypeId>::new();

        columns.push(col_type.id());

        let row_type = self.table.get_row_type(row_type);

        let entity_meta = &mut self.entity_meta;

        let type_id = entity_meta.entity_type(columns);
        // let entity_type = entity_meta.get_entity_type(type_id).expect("entity-type");

        let entity_row_type = entity_meta.entity_row(row_type, type_id);

        entity_row_type.id()
    }

    pub(crate) fn push_entity_type(
        &mut self, 
        entity_row_type: EntityRowTypeId, 
        col_type_id: ColumnTypeId
    ) -> EntityRowTypeId {
        let entity_row = self.entity_meta
            .get_entity_row(entity_row_type);

        let row_type_id = self.table.push_row_type(
            entity_row.row_type_id(),
            col_type_id
        );

        let row_type = self.table.get_row_type(row_type_id);

        let entity_type = self.entity_meta.push_entity_type(
            entity_row.entity_type_id(),
            col_type_id
        );

        let entity_row_type = self.entity_meta.entity_row(
            row_type, entity_type
        );

        entity_row_type.id()
    }
}

impl<T:'static> EntityRef<T> {
    pub fn get<'a>(&self, table: &'a EntityTable) -> Option<&'a T> {
        let row_type = table.entity_meta.get_entity_row(self.entity_row_type);
            
        table.table.get_row(self.row_id, row_type.columns())
    }

    pub fn push<S:'static>(&self, table: &mut EntityTable, value: S) {
        table.table.replace_push(self.row_id, value);
        /*
        let col_type = table.table.column_type::<S>();
        let col_type_id = col_type.id();

        let entity_type = table.push_entity_type(
            self.entity_row_type, 
            col_type_id
        );

        let row_type = table.entity_meta
            .get_entity_row(self.entity_row_type);

        table.table.get_row(self.row_id, row_type.columns())
        */
    }
}

#[cfg(test)]
mod tests {
    use super::EntityTable;

    #[test]
    fn test_entity() {
        let mut table = EntityTable::new();

        let ref_a = table.push(TestA(1));
        let value_a = ref_a.get(&table).expect("entity");
        assert_eq!(value_a, &TestA(1));

        let ref_b = table.push(TestB(2));
        let value_b = ref_b.get(&table).expect("entity");
        assert_eq!(value_b, &TestB(2));

        let ref_a3 = table.push(TestA(3));
        let value_a = ref_a3.get(&table).expect("entity");
        assert_eq!(value_a, &TestA(3));

        let value_a = ref_a.get(&table).expect("entity");
        assert_eq!(value_a, &TestA(1));
    }

    #[test]
    fn test_entity_push() {
        let mut table = EntityTable::new();

        let ref_a = table.push(TestA(1));
        let value_a = ref_a.get(&table).unwrap();
        assert_eq!(value_a, &TestA(1));

        let ref_b = ref_a.push(&mut table, TestB(2));

        assert_eq!(ref_a.get(&table).unwrap(), &TestA(1));
    }

    #[derive(Debug, PartialEq)]
    struct TestA(usize);

    #[derive(Debug, PartialEq)]
    struct TestB(usize);
}
