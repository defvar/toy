use serde::{Deserialize, Serialize};
use toy_pack::Schema;
use crate::collector::{CpuCollector, MemoryCollector, StatCollector};

#[derive(Debug, Clone, Deserialize, Serialize, Schema)]
pub struct CpuConfig {}

#[derive(Debug, Clone, Deserialize, Serialize, Schema)]
pub struct MemoryConfig {}

pub trait ToCollector<T>
where
    T: StatCollector,
{
    fn to_collector(&self) -> T;
}

impl ToCollector<CpuCollector> for CpuConfig {
    fn to_collector(&self) -> CpuCollector {
        CpuCollector::new()
    }
}

impl ToCollector<MemoryCollector> for MemoryConfig {
    fn to_collector(&self) -> MemoryCollector {
        MemoryCollector::new()
    }
}
