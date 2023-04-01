use std::{mem, collections::HashMap, cmp::max, slice::Iter, any::TypeId};

use super::prelude::TypeMetas;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RowTypeId(usize);

pub struct RowType {
    id: RowTypeId,
    columns: Vec<ColumnType>,
    align: usize,
    length: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColumnTypeId(usize);

#[derive(Clone, Debug)]
pub struct ColumnType {
    id: ColumnTypeId,
    align: usize,
    length: usize,

    offset: usize,
}

pub(crate) struct RowMetas {
    col_type_metas: TypeMetas,
    col_types: Vec<ColumnType>,

    //row_type_metas: TypeMetas,
    row_col_map: HashMap<Vec<ColumnTypeId>,RowTypeId>,
    row_type_map: HashMap<TypeId,RowTypeId>,
    row_types: Vec<RowType>,

    col_type_rows: Vec<Vec<RowTypeId>>,
}

impl ColumnTypeId {
    pub fn index(&self) -> usize {
        self.0
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

    pub fn columns(&self) -> Iter<ColumnType> {
        self.columns.iter()
    }

    pub fn column(&self, index: usize) -> &ColumnType {
        self.columns.get(index).unwrap()
    }

    pub fn column_position(&self, id: ColumnTypeId) -> Option<usize> {
        self.columns.iter().position(|col| col.id() == id)
    }

    pub fn column_find(&self, id: ColumnTypeId) -> Option<&ColumnType> {
        self.columns.iter().find(|col| col.id() == id)
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

    pub fn offset(&self) -> usize {
        self.offset
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
                offset: 0,
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

    pub fn get_column(&self, id: ColumnTypeId) -> &ColumnType {
        self.col_types.get(id.index()).unwrap()
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

        let mut column_types = Vec::<ColumnType>::new();

        for column_id in &columns {
            let column_type = self.col_types.get(column_id.0).unwrap();

            let mut col = column_type.clone();
            col.offset = length;

            length += column_type.length; // TODO: align
            align = max(align, column_type.align); 

            column_types.push(col);
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
                columns: column_types,
                length: length,
                align: align,
            });
        }

        row_type_id
    }

    fn push_row_type(&mut self, row_type: RowType) {
        let row_type_id = row_type.id();

        for col in &row_type.columns {
            let col_rows = self.col_type_rows.get_mut(col.id().index()).unwrap();

            col_rows.push(row_type_id);
        }

        self.row_types.push(row_type);
    }

    /*
    pub fn add_row_cols<T:'static>(&mut self, mut columns: Vec<ColumnTypeId>) -> RowTypeId {
        // let type_id = TypeId::of::<T>();
        columns.sort();
        let mut length: usize = 0;
        let mut align: usize = 1;

        let mut column_types = Vec::<ColumnType>::new();

        for column_id in &columns {
            let column_type = self.col_types.get(column_id.0).expect("column_id");

            let mut col = column_type.clone();
            col.offset = length; // TODO: align

            length += column_type.length; // TODO: align
            align = max(align, column_type.align); 

            column_types.push(col);
        }

        //let type_id = self.row_type_metas.add_type::<T>();
        let len = self.row_col_map.len();
        let row_type_id = *self.row_col_map.entry(columns.clone()).or_insert_with(|| {
            RowTypeId(len)
        });

        if row_type_id.index() == self.row_types.len() {
            self.push_row_type(RowType {
                id: row_type_id,
                columns: column_types,
                length: length,
                align: align,
            });
        }

        row_type_id

        //self.row_type_map.insert(type_id, *row_type_id);

        //self.row_types.get(row_type_id.index()).expect("get row")
    }
    */

    pub fn add_row_type<T:'static>(&mut self, row_type: RowTypeId) -> RowTypeId {
        let type_id = TypeId::of::<T>();

        self.row_type_map.insert(type_id, row_type);

        row_type
    }

    pub fn get_row<T:'static>(&self) -> Option<&RowType> {
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
}

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::store::row_meta::{ColumnTypeId, RowTypeId, ColumnType};

    use super::RowMetas;

    #[test]
    fn add_column() {
        let mut meta = RowMetas::new();

        let col_type = meta.add_column::<TestA>();
        assert_eq!(col_type.id(), ColumnTypeId(0));
        assert_eq!(col_type.length(), mem::size_of::<usize>());
        assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.offset(), 0);

        let col_type = meta.add_column::<TestB>();
        assert_eq!(col_type.id(), ColumnTypeId(1));
        assert_eq!(col_type.length(), mem::size_of::<usize>());
        assert_eq!(col_type.align(), mem::align_of::<usize>());
        assert_eq!(col_type.offset(), 0);

        // check double add
        let col_type = meta.add_column::<TestA>();
        assert_eq!(col_type.id(), ColumnTypeId(0));
    }

    #[test]
    fn add_single_row() {
        let mut meta = RowMetas::new();

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, RowTypeId(0));

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, RowTypeId(0));

        let type_b = meta.single_row_type::<TestB>();
        assert_eq!(type_b, RowTypeId(1));
    }

    #[test]
    fn push_row() {
        let mut meta = RowMetas::new();

        let type_a = meta.single_row_type::<TestA>();
        assert_eq!(type_a, RowTypeId(0));

        let type_aa = meta.push_row_by_type::<TestA>(type_a);
        assert_eq!(type_aa, RowTypeId(0));

        let type_b = meta.single_row_type::<TestB>();
        assert_eq!(type_b, RowTypeId(1));

        let type_ab = meta.push_row_by_type::<TestB>(type_a);
        assert_eq!(type_ab, RowTypeId(2));

        let type_aba = meta.push_row_by_type::<TestA>(type_ab);
        assert_eq!(type_aba, RowTypeId(2));

        let type_ba = meta.push_row_by_type::<TestA>(type_b);
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
        let cols : Vec<&ColumnType> = row_type.columns().collect();
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
        let cols : Vec<&ColumnType> = row_type.columns().collect();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].id(), col_a);
        assert_eq!(cols[1].id(), col_b);
        assert_eq!(cols[1].offset(), cols[0].length());
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

    struct TestA(usize);
    struct TestB(usize);
    struct TestC(usize);
}