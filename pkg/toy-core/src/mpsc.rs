use crate::error::OutgoingError;
use std::collections::{BTreeMap};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub fn channel<T>(buffer: usize) -> (Outgoing<T>, Incoming<T>) {
    let (tx, rx) = mpsc::channel(buffer);
    (Outgoing::new(tx), Incoming::new(rx))
}

#[derive(Debug)]
pub struct Incoming<T> {
    inner: Receiver<T>,
}

impl<T> Incoming<T> {
    pub fn new(rx: Receiver<T>) -> Incoming<T> {
        Incoming { inner: rx }
    }

    pub async fn next(&mut self) -> Option<T> {
        self.inner.recv().await
    }
}

#[derive(Debug)]
pub struct Outgoing<T> {
    inner: Vec<Sender<T>>,
    /// key = self output port, value = target input port.
    port_map: BTreeMap<u8, u8>,
}

impl<T> Outgoing<T> {
    pub fn empty() -> Outgoing<T> {
        Outgoing {
            inner: vec![],
            port_map: BTreeMap::new(),
        }
    }

    pub fn new(tx: Sender<T>) -> Outgoing<T> {
        let mut port_map = BTreeMap::new();
        port_map.insert(0, 0);
        Outgoing {
            inner: vec![tx],
            port_map,
        }
    }

    pub fn merge(&mut self, tx: Outgoing<T>) {
        let next = self.inner.len();
        let mut inner_idx = 0;
        for i in next..(next + tx.inner.len()) {
            self.port_map
                .insert(i as u8, *tx.port_map.get(&inner_idx).unwrap());
            inner_idx += 1;
        }
        self.inner.extend(tx.inner);
    }

    /// merge, specified input port.
    pub fn merge_by(&mut self, input_port: u8, tx: Outgoing<T>) {
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

    pub fn is_closed(&self) -> bool {
        self.inner[0].is_closed()
    }
}

impl<T> Outgoing<T>
where
    T: OutgoingMessage,
{
    pub async fn send(&mut self, mut v: T) -> Result<(), OutgoingError> {
        v.set_port(self.target_input_port(0));
        if let Err(e) = self.inner[0].send(v).await {
            Err(OutgoingError::send_error(e))
        } else {
            Ok(())
        }
    }

    pub async fn send_to(&mut self, port: u8, mut v: T) -> Result<(), OutgoingError> {
        if (port as usize) >= self.inner.len() {
            return Err(OutgoingError::not_found_output_port(port));
        }

        v.set_port(self.target_input_port(port));

        if let Err(e) = self.inner[port as usize].send(v).await {
            Err(OutgoingError::send_error(e))
        } else {
            Ok(())
        }
    }

    pub async fn send_ok(&mut self, v: T) -> Result<(), OutgoingError> {
        self.send_to(0, v).await
    }

    pub async fn send_ok_to(&mut self, port: u8, v: T) -> Result<(), OutgoingError> {
        self.send_to(port, v).await
    }

    pub async fn send_ok_all(&mut self, v: T) -> Vec<Result<(), OutgoingError>>
    where
        T: Clone,
    {
        let mut vec: Vec<Result<(), OutgoingError>> = vec![];
        for p in self.ports() {
            let r = self.send_ok_to(p, v.clone()).await;
            vec.push(r);
        }
        vec
    }

    pub fn ports(&self) -> OutgoingPortIter {
        let ports = self.port_map.keys().into_iter().map(|x| *x).collect();
        OutgoingPortIter { ports, idx: 0 }
    }

    pub fn ports_len(&self) -> usize {
        self.port_map.keys().len()
    }

    pub fn is_closed_at(&self, port: u8) -> Result<bool, ()> {
        if (port as usize) >= self.inner.len() {
            //return Result::Err(Error::custom(format!("not found output port:{}", port)));
            return Err(());
        }
        Ok(self.inner[port as usize].is_closed())
    }
}

impl<T> Clone for Outgoing<T> {
    fn clone(&self) -> Self {
        Outgoing {
            inner: self.inner.clone(),
            port_map: self.port_map.clone(),
        }
    }
}

pub struct OutgoingPortIter {
    ports: Vec<u8>,
    idx: usize,
}

impl Iterator for OutgoingPortIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.ports.get(self.idx).map(|x| *x);
        self.idx += 1;
        r
    }
}

pub trait OutgoingMessage {
    fn set_port(&mut self, port: u8);
}

impl OutgoingMessage for () {
    fn set_port(&mut self, _port: u8) {}
}
