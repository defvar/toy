use crate::models::field_value::FieldValue;

#[derive(Clone, Debug)]
pub struct FluxTable {
    annotation: AnnotationRecord,
    headers: Vec<String>,
    data: Vec<DataRecord>,
}

impl FluxTable {
    pub fn with(annotation: AnnotationRecord, headers: Vec<String>, data: Vec<DataRecord>) -> Self {
        Self {
            annotation,
            headers,
            data,
        }
    }

    pub fn annotation(&self) -> &AnnotationRecord {
        &self.annotation
    }

    pub fn headers(&self) -> impl Iterator<Item = &str> {
        self.headers.iter().map(|x| x.as_str())
    }

    pub fn header(&self, index: usize) -> Option<&str> {
        self.headers.get(index).map(|x| x.as_str())
    }

    pub fn column_size(&self) -> usize {
        self.headers.len()
    }

    pub fn row_size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[DataRecord] {
        &self.data
    }
}

#[derive(Clone, Debug)]
pub struct AnnotationRecord {
    group: Option<GroupRecord>,
    data_type: Option<DataTypeRecord>,
}

impl AnnotationRecord {
    pub fn with(group: Option<GroupRecord>, data_type: Option<DataTypeRecord>) -> Self {
        Self { group, data_type }
    }

    pub fn group(&self) -> Option<&GroupRecord> {
        self.group.as_ref()
    }

    pub fn data_type(&self) -> Option<&DataTypeRecord> {
        self.data_type.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct DataRecord {
    table: u32,
    fields: Vec<FieldValue>,
}

impl DataRecord {
    pub fn new(table: u32) -> Self {
        Self {
            table,
            fields: Vec::new(),
        }
    }

    /// Capacity is default value. Extend as needed.
    pub fn with_capacity(table: u32, field_capacity: usize) -> Self {
        Self {
            table,
            fields: Vec::with_capacity(field_capacity),
        }
    }

    pub fn with_fields(table: u32, fields: Vec<FieldValue>) -> Self {
        Self { table, fields }
    }

    pub fn table(&self) -> u32 {
        self.table
    }

    pub fn push(&mut self, value: FieldValue) {
        self.fields.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&FieldValue> {
        self.fields.get(index)
    }
}

#[derive(Clone, Debug)]
pub struct GroupRecord {
    is_group: Vec<bool>,
}

impl GroupRecord {
    pub fn new() -> Self {
        Self { is_group: vec![] }
    }

    pub fn with(is_group: Vec<bool>) -> Self {
        Self { is_group }
    }

    pub fn push(&mut self, v: bool) {
        self.is_group.push(v);
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        self.is_group.get(index).map(|x| *x)
    }
}

#[derive(Clone, Debug)]
pub struct DataTypeRecord {
    data_types: Vec<String>,
}

impl DataTypeRecord {
    pub fn new() -> Self {
        Self { data_types: vec![] }
    }

    pub fn with(data_types: Vec<String>) -> Self {
        Self { data_types }
    }

    pub fn push(&mut self, v: impl Into<String>) {
        self.data_types.push(v.into());
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.data_types.get(index).map(|x| x.as_str())
    }
}
