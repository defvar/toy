use crate::error::Error;
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub fn channel<T, Err>(buffer: usize) -> (Outgoing<T, Err>, Incoming<T, Err>) {
    let (tx, rx) = mpsc::channel(buffer);
    (Outgoing::new(tx), Incoming::new(rx))
}

#[derive(Debug)]
pub struct Incoming<T, Err> {
    inner: Receiver<Result<T, Err>>,
}

impl<T, Err> Incoming<T, Err> {
    pub fn new(rx: Receiver<Result<T, Err>>) -> Incoming<T, Err> {
        Incoming { inner: rx }
    }

    pub async fn next(&mut self) -> Option<Result<T, Err>> {
        self.inner.recv().await
    }
}

#[derive(Debug)]
pub struct Outgoing<T, Err> {
    inner: Vec<Sender<Result<T, Err>>>,
    /// key = self output port, value = target input port.
    port_map: HashMap<u8, u8>,
}

impl<T, Err> Outgoing<T, Err> {
    pub fn empty() -> Outgoing<T, Err> {
        Outgoing {
            inner: vec![],
            port_map: HashMap::new(),
        }
    }

    pub fn new(tx: Sender<Result<T, Err>>) -> Outgoing<T, Err> {
        let mut port_map = HashMap::new();
        port_map.insert(0, 0);
        Outgoing {
            inner: vec![tx],
            port_map,
        }
    }

    pub fn merge(&mut self, tx: Outgoing<T, Err>) {
        self.inner.extend(tx.inner)
    }

    /// merge, specified input port.
    pub fn merge_by(&mut self, input_port: u8, tx: Outgoing<T, Err>) {
        let next = self.inner.len();
        for i in next..(next + tx.inner.len()) {
            self.port_map.insert(i as u8, input_port);
        }
        self.inner.extend(tx.inner);
    }

    /// get target input port by self output port.
    fn target_input_port(&self, output_port: u8) -> u8 {
        *self.port_map.get(&output_port).unwrap_or_else(|| &0)
    }
}

impl<T, Err> Outgoing<T, Err>
where
    T: OutgoingMessage,
    Err: Error,
{
    pub async fn send(&mut self, v: Result<T, Err>) -> Result<(), Err> {
        let v = v.map(|mut x| {
            x.set_port(self.target_input_port(0));
            x
        });
        if let Result::Err(e) = self.inner[0].send(v).await {
            Result::Err(Error::custom(e))
        } else {
            Ok(())
        }
    }

    pub async fn send_to(&mut self, port: u8, v: Result<T, Err>) -> Result<(), Err> {
        if (port as usize) >= self.inner.len() {
            return Result::Err(Error::custom(format!("not found output port:{}", port)));
        }

        let v = v.map(|mut x| {
            x.set_port(self.target_input_port(port));
            x
        });
        if let Result::Err(e) = self.inner[port as usize].send(v).await {
            Result::Err(Error::custom(e))
        } else {
            Ok(())
        }
    }

    pub async fn send_ok(&mut self, v: T) -> Result<(), Err> {
        self.send_to(0, Ok(v)).await
    }

    pub async fn send_ok_to(&mut self, port: u8, v: T) -> Result<(), Err> {
        self.send_to(port, Ok(v)).await
    }
}

impl<T, Err> Clone for Outgoing<T, Err> {
    fn clone(&self) -> Self {
        Outgoing {
            inner: self.inner.clone(),
            port_map: self.port_map.clone(),
        }
    }
}

pub trait OutgoingMessage {
    fn set_port(&mut self, port: u8);
}
