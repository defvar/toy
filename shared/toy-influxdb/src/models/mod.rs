mod common;
mod field_value;
pub mod line_protocol;
pub mod query_param;

mod table;
mod table_builder;

pub use common::{Annotation, ErrorInfo};
pub use field_value::FieldValue;

pub mod flux_table {
    pub use super::table::*;
    pub use super::table_builder::FluxTableBuilder;
}

pub mod constants {
    pub static ANNOTATION_IDENT_GROUP: &[u8] = b"#group";
    pub static ANNOTATION_IDENT_DATATYPE: &[u8] = b"#datatype";
    pub static ANNOTATION_IDENT_DEFAULT: &[u8] = b"#default";

    pub static ANNOTATION_IDENTS: &[&[u8]] = &[
        ANNOTATION_IDENT_GROUP,
        ANNOTATION_IDENT_DATATYPE,
        ANNOTATION_IDENT_DEFAULT,
    ];
}
