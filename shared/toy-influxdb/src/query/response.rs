use crate::models::flux_table;
use crate::models::flux_table::FluxTable;
use crate::InfluxDBError;
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct QueryResponse {
    raw: Vec<FluxTable>,
}

impl QueryResponse {
    pub fn with(tables: Vec<FluxTable>) -> Self {
        Self { raw: tables }
    }

    pub fn raw(&self) -> &[FluxTable] {
        &self.raw
    }

    pub fn table_len(&self) -> usize {
        self.raw.len()
    }

    pub fn unpack<T>(mut self) -> Result<Vec<T>, InfluxDBError>
    where
        T: DeserializeOwned,
    {
        self.raw.iter_mut().try_fold(Vec::new(), |mut vec, x| {
            let v: Vec<T> = flux_table::unpack(x)?;
            vec.extend(v);
            Ok(vec)
        })
    }

    pub fn unpack_by<T, F>(mut self, mut f: F) -> Result<Vec<T>, InfluxDBError>
    where
        T: DeserializeOwned,
        F: FnMut(&FluxTable) -> Result<Vec<T>, InfluxDBError>,
    {
        self.raw.iter_mut().try_fold(Vec::new(), |mut vec, x| {
            let v = f(x)?;
            vec.extend(v);
            Ok(vec)
        })
    }
}
