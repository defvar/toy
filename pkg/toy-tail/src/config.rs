use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

const DEFAULT_CAPACITY: usize = 8 * (1 << 10);

fn default_addrs() -> Vec<SocketAddr> {
    vec![SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        6060,
    )]
}

#[derive(Debug, Clone)]
pub struct TailConfig {
    buffer_capacity: usize,
    check_interval_millis: u64,
    threshold_millis: u64,

    addrs: Vec<SocketAddr>,
}

#[derive(Debug, Clone)]
pub struct TailConfigBuilder {
    c: TailConfig,
}

impl TailConfig {
    pub fn buffer_capacity(&self) -> usize {
        self.buffer_capacity
    }

    pub fn check_interval_millis(&self) -> u64 {
        self.check_interval_millis
    }

    pub fn threshold_millis(&self) -> u64 {
        self.threshold_millis
    }

    pub fn addrs(&self) -> &[SocketAddr] {
        &self.addrs
    }
}

impl TailConfigBuilder {
    pub fn new() -> TailConfig {
        TailConfig::default()
    }

    pub fn buffer_capacity(mut self, v: usize) -> TailConfigBuilder {
        self.c.buffer_capacity = v;
        self
    }

    pub fn check_interval_millis(mut self, v: u64) -> TailConfigBuilder {
        self.c.check_interval_millis = v;
        self
    }

    pub fn threshold_millis(mut self, v: u64) -> TailConfigBuilder {
        self.c.threshold_millis = v;
        self
    }

    pub fn addr<A: ToSocketAddrs>(mut self, v: A) -> TailConfigBuilder {
        let addrs = match v.to_socket_addrs() {
            Ok(addrs) => addrs.collect(),
            Err(_) => default_addrs(),
        };
        self.c.addrs = addrs;
        self
    }

    pub fn build(self) -> TailConfig {
        self.c
    }
}

impl Default for TailConfig {
    fn default() -> Self {
        Self {
            buffer_capacity: DEFAULT_CAPACITY,
            check_interval_millis: 2000,
            threshold_millis: 2000,
            addrs: default_addrs(),
        }
    }
}
