use crate::common::Format;
use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Supervisor {
    name: String,
    start_time: String,
    labels: Vec<String>,
}

impl Supervisor {
    pub fn new(name: String, start_time: String, labels: Vec<String>) -> Self {
        Self {
            name,
            start_time,
            labels,
        }
    }
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
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct ListOption {
    format: Option<Format>,
}

impl ListOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack, Default)]
pub struct PutOption {
    format: Option<Format>,
}

impl PutOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct DeleteOption {
    format: Option<Format>,
}

impl DeleteOption {
    pub fn new() -> Self {
        Self { format: None }
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }
}
