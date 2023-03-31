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
    offset: usize,
    length: usize,
}

pub struct RowMetas {
    col_type_metas: TypeMetas,
    col_types: Vec<ColumnType>,

    //row_type_metas: TypeMetas,
    row_col_map: HashMap<Vec<ColumnTypeId>,RowTypeId>,
    row_type_map: HashMap<TypeId,RowTypeId>,
    row_types: Vec<RowType>,
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
        self.columns.get(index).expect("index column")
    }
}

impl ColumnType {
    pub fn id(&self) -> ColumnTypeId {
        self.id
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn align(&self) -> usize {
        self.align
    }

    pub fn length(&self) -> usize {
        self.length
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
        }
    }

    pub fn add_row<T:'static>(&mut self, mut columns: Vec<ColumnTypeId>) -> &RowType {
        let type_id = TypeId::of::<T>();
        columns.sort();
        let mut len: usize = 0;
        let mut align: usize = 1;

        let mut column_types = Vec::<ColumnType>::new();

        for column_id in &columns {
            let column_type = self.col_types.get(column_id.0).expect("column_id");

            len += column_type.length; // TODO: align
            align = max(align, column_type.align); 

            column_types.push(column_type.clone());
        }

        //let type_id = self.row_type_metas.add_type::<T>();
        let len = self.row_col_map.len();
        let row_type_id = self.row_col_map.entry(columns.clone()).or_insert_with(|| {
            RowTypeId(len)
        });

        if row_type_id.index() == self.row_types.len() {
            self.row_types.push(RowType {
                id: *row_type_id,
                columns: column_types,
                length: len,
                align: align,
            });
        }

        self.row_type_map.insert(type_id, *row_type_id);

        self.row_types.get(row_type_id.index()).expect("get row")
    }

    pub fn get_row<T:'static>(&self) -> Option<&RowType> {
        match self.row_type_map.get(&TypeId::of::<T>()) {
            Some(row_id) => {
                self.row_types.get(row_id.index())
            },
            None => None,
        }
    }

    pub fn get_row_id(&mut self, row_type_id: RowTypeId) -> Option<&RowType> {
        self.row_types.get(row_type_id.index())
    }

    pub fn single_row_type<T:'static>(&mut self) -> &RowType {
        let column_type = self.add_column::<T>();
        let mut col_vec = Vec::<ColumnTypeId>::new();
        col_vec.push(column_type.id());

        self.add_row::<T>(col_vec)
    }

    pub fn add_column<T:'static>(&mut self) -> &ColumnType {
        let type_index = self.col_type_metas.add_type::<T>();

        let mut offset = 0;

        if self.col_types.len() <= type_index.index() {
            assert!(type_index.index() == self.col_types.len());

            let align = mem::align_of::<T>();
            let length = mem::size_of::<T>();

            let col_type = ColumnType {
                id: ColumnTypeId(type_index.index()),
                align: align,
                offset: offset,
                length: length,
            };

            offset += length;

            self.col_types.push(col_type);
        }

        return self.col_types.get(type_index.index()).expect("missing col");
    }

    pub fn get_column<T:'static>(&self) -> Option<&ColumnType> {
        match self.col_type_metas.get_id::<T>() {
            Some(type_id) => { 
                self.col_types.get(type_id.index())
            },
            None => None,
        }
    }
}
