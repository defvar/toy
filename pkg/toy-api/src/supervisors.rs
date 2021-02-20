use crate::common::Format;
use crate::service::ServicesEntity;
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Supervisor {
    name: String,
    host_ip: String,
    start_time: String,
    labels: Vec<String>,
    services: ServicesEntity,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Supervisors {
    supervisors: Vec<Supervisor>,
    count: u32,
}

impl Supervisors {
    pub fn new(supervisors: Vec<Supervisor>) -> Self {
        let count = supervisors.len() as u32;
        Self { supervisors, count }
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct FindOption {
    format: Option<Format>,
}

impl FindOption {
    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ListOption {
    format: Option<Format>,
}

impl ListOption {
    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct PutOption {
    format: Option<Format>,
}

impl PutOption {
    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct DeleteOption {}

impl DeleteOption {
    pub fn new() -> Self {
        Self {}
    }
}
