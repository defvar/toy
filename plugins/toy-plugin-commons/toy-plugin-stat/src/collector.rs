use toy_core::data::{Value};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use toy_core::map_value;

pub trait StatCollector {
    fn to_stat_value(&mut self) -> Value;
}

pub struct CpuCollector {
    sys: System,
}

impl CpuCollector {
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()))
        }
    }
}

impl StatCollector for CpuCollector {
    fn to_stat_value(&mut self) -> Value {
        self.sys.refresh_cpu();

        let mut usages = Vec::new();
        for c in self.sys.cpus() {
            usages.push(Value::from(c.cpu_usage()));
        }
        let usages = Value::Seq(usages);
        map_value! {
            "global_usage" => self.sys.global_cpu_info().cpu_usage(),
            "usages" => usages
        }
    }
}

pub struct MemoryCollector {
    sys: System,
}

impl MemoryCollector {
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()))
        }
    }
}

impl StatCollector for MemoryCollector {
    fn to_stat_value(&mut self) -> Value {
        self.sys.refresh_memory();
        map_value! {
            "total_memory" => self.sys.total_memory(),
            "used_memory" => self.sys.used_memory(),
            "total_swap" => self.sys.total_swap(),
            "used_swap" => self.sys.used_swap(),
         }
    }
}

#[cfg(test)]
mod tests {
    use crate::collector::{CpuCollector, MemoryCollector, StatCollector};

    #[test]
    fn cpu() {
        let mut c = CpuCollector::new();
        let v = c.to_stat_value();

        println!("{:?}", v);
    }

    #[test]
    fn memory() {
        let mut c = MemoryCollector::new();
        let v = c.to_stat_value();

        println!("{:?}", v);
    }
}