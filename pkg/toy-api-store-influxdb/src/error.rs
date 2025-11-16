use thiserror::Error;
use toy_api_server::store::error::StoreErrorCustom;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum InfluxDBError {
    #[error(transparent)]
    Error {
        #[from]
        source: toy_influxdb::InfluxDBError,
    },
}

impl StoreErrorCustom for InfluxDBError {}
