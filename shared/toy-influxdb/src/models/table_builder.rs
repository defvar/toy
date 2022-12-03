use crate::models::field_value::FieldValue;
use crate::models::table::{AnnotationRecord, DataRecord, DataTypeRecord, FluxTable, GroupRecord};
use crate::InfluxDBError;

pub struct FluxTableBuilder {
    buf: Option<DataRecord>,
    table_index_being_edited: Option<usize>,
    tables: Vec<Inner>,
    field_capacity: usize,
    row_capacity: usize,
}

struct Inner {
    headers: Vec<String>,
    data_type: Option<DataTypeRecord>,
    group: Option<GroupRecord>,
    records: Vec<DataRecord>,
}

impl FluxTableBuilder {
    /// Capacity is default value. Extend as needed.
    pub fn with_capacity(field_capacity: usize, row_capacity: usize) -> Self {
        Self {
            buf: None,
            table_index_being_edited: None,
            tables: vec![FluxTableBuilder::new_inner(field_capacity, row_capacity)],
            field_capacity,
            row_capacity,
        }
    }

    fn new_inner(field_capacity: usize, row_capacity: usize) -> Inner {
        Inner {
            headers: Vec::with_capacity(field_capacity),
            data_type: None,
            group: None,
            records: Vec::with_capacity(row_capacity),
        }
    }

    fn ensure_table(&mut self, index: usize) {
        while self.tables.len() <= index {
            self.tables.push(FluxTableBuilder::new_inner(
                self.field_capacity,
                self.row_capacity,
            ))
        }
    }

    pub fn is_header_empty(&self, table_index: usize) -> bool {
        if self.tables.len() > table_index {
            self.tables[table_index].headers.is_empty()
        } else {
            false
        }
    }

    pub fn is_record_started(&self) -> bool {
        self.buf.is_some()
    }

    pub fn table_index_being_edited(&self) -> Option<usize> {
        self.table_index_being_edited
    }

    pub fn get_field_index(&self, table_index: usize, field: &str) -> Option<usize> {
        if self.tables.len() > table_index {
            self.tables[table_index]
                .headers
                .iter()
                .position(|x| x == field)
        } else {
            None
        }
    }

    pub fn headers<I, S>(&mut self, table_index: usize, fields: I)
    where
        S: Into<String>,
        I: Iterator<Item = S>,
    {
        self.ensure_table(table_index);
        self.tables[table_index].headers.clear();
        fields.for_each(|x| self.tables[table_index].headers.push(x.into()))
    }

    pub fn data_type<I, S>(&mut self, table_index: usize, fields: I)
    where
        S: Into<String>,
        I: Iterator<Item = S>,
    {
        self.ensure_table(table_index);
        let items = fields.map(|x| x.into()).collect();
        self.tables[table_index].data_type = Some(DataTypeRecord::with(items));
    }

    pub fn group<'a, I>(&mut self, table_index: usize, fields: I)
    where
        I: Iterator<Item = &'a str>,
    {
        self.ensure_table(table_index);
        let items = fields
            .map(|x| !x.is_empty() && (x.as_bytes()[0] == b't' || x.as_bytes()[0] == b'T'))
            .collect();
        self.tables[table_index].group = Some(GroupRecord::with(items));
    }

    pub fn start_record(&mut self, table_index: usize, table: u32) {
        self.ensure_table(table_index);
        self.table_index_being_edited = Some(table_index);
        self.buf = Some(DataRecord::with_capacity(table, self.field_capacity));
    }

    pub fn end_record(&mut self) {
        if self.buf.is_some() {
            let idx = self.table_index_being_edited.take().unwrap();
            self.tables[idx].records.push(self.buf.take().unwrap());
        }
    }

    pub fn push(&mut self, field_index: usize, value: &str) -> Result<(), InfluxDBError> {
        if !self.is_record_started() {
            Err(InfluxDBError::error(
                "illegal builder state, must be call after start_record.",
            ))
        } else {
            let table_index = self.table_index_being_edited.unwrap();
            let field = match self.tables[table_index].headers.get(field_index) {
                Some(f) => f.as_str(),
                None => return Err(InfluxDBError::error("unknwon field")),
            };
            let tp = self.tables[table_index]
                .data_type
                .as_ref()
                .map(|x| x.get(field_index))
                .flatten();
            let fv = FieldValue::from(field, tp.unwrap_or("string"), value)?;

            self.buf.as_mut().unwrap().push(fv);
            Ok(())
        }
    }

    pub fn build(self) -> Vec<FluxTable> {
        self.tables
            .into_iter()
            .map(|x| {
                FluxTable::with(
                    AnnotationRecord::with(x.group, x.data_type),
                    x.headers,
                    x.records,
                )
            })
            .collect()
    }
}
