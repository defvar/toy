//! # node channel
//! Channel for node communication.
//! - `Incomings`
//!   Receive channel for each URI.
//! - `Outgoings`
//!   Sending channel for each URI.
//! - `Starters`
//!   Sending channel for first node URI of task.
//! - `Awaiters`
//!   Receive channel for last node URI of task.
//! - `SignalOutgoings`
//!   Sending channel for Supervisor use. (Outgoins + Starters)

use std::collections::HashMap;
use toy_core::data::Frame;
use toy_core::error::ServiceError;
use toy_core::graph::{Graph, InputWire, OutputWire};
use toy_core::mpsc::{self, Incoming, Outgoing};
use toy_core::Uri;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 128;

#[derive(Debug)]
struct IncomingInner {
    rx: Incoming<Frame, ServiceError>,
    upstream_count: u32,
}

#[derive(Debug)]
pub struct Incomings {
    map: HashMap<Uri, IncomingInner>,
}

#[derive(Debug)]
pub struct Outgoings {
    map: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

#[derive(Debug)]
pub struct Awaiter {
    inner: IncomingInner,
}

#[derive(Debug)]
pub struct Starters {
    map: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

#[derive(Debug)]
pub struct SignalOutgoings {
    map: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

impl IncomingInner {
    pub fn from_rx(rx: Incoming<Frame, ServiceError>) -> IncomingInner {
        IncomingInner::from_rx_and_count(rx, 1)
    }

    pub fn from_rx_and_count(
        rx: Incoming<Frame, ServiceError>,
        upstream_count: u32,
    ) -> IncomingInner {
        IncomingInner { rx, upstream_count }
    }

    fn into_tuple(self) -> (Incoming<Frame, ServiceError>, u32) {
        (self.rx, self.upstream_count)
    }
}

impl Incomings {
    pub fn pop(&mut self, uri: &Uri) -> Option<(Incoming<Frame, ServiceError>, u32)> {
        self.map.remove(uri).map(|x| x.into_tuple())
    }
}

impl Outgoings {
    pub fn pop(&mut self, uri: &Uri) -> Option<Outgoing<Frame, ServiceError>> {
        self.map.remove(uri)
    }
}

impl Starters {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Uri, &mut Outgoing<Frame, ServiceError>)> {
        self.map.iter_mut()
    }
}

impl Awaiter {
    pub async fn next(&mut self) -> Option<Result<Frame, ServiceError>> {
        self.inner.rx.next().await
    }

    pub fn upstream_count(&self) -> u32 {
        self.inner.upstream_count
    }
}

impl SignalOutgoings {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Uri, &mut Outgoing<Frame, ServiceError>)> {
        self.map.iter_mut()
    }
}

pub fn from_graph(graph: &Graph) -> (Incomings, Outgoings, Awaiter, Starters, SignalOutgoings) {
    let mut incomings: HashMap<Uri, IncomingInner> = HashMap::new();
    let mut outgoings: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

    let mut starters: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

    let mut awaiter_upsteram_count = 0;
    let (awaiter_tx, awaiter_rx) =
        mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);

    // first channel
    graph
        .inputs()
        .iter()
        .filter(|(_, w)| **w == InputWire::None)
        .for_each(|(uri, _)| {
            let (tx, rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
            incomings.insert(uri.clone(), IncomingInner::from_rx(rx));
            starters.insert(uri.clone(), tx);
        });

    // last channel
    graph
        .outputs()
        .iter()
        .filter(|(_, w)| **w == OutputWire::None)
        .for_each(|(uri, _)| {
            awaiter_upsteram_count += 1;
            outgoings
                .entry(uri.clone())
                .or_insert_with(|| Outgoing::empty())
                .merge(awaiter_tx.clone());
        });

    for (_, wire) in graph.inputs() {
        let (tx, rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
        match wire {
            InputWire::Single(o, i) => {
                incomings.insert(i.clone(), IncomingInner::from_rx(rx));
                outgoings
                    .entry(o.clone())
                    .or_insert_with(|| Outgoing::empty())
                    .merge(tx.clone());
            }
            InputWire::Fanin(o, i) => {
                incomings.insert(
                    i.clone(),
                    IncomingInner::from_rx_and_count(rx, o.len() as u32),
                );
                o.iter().enumerate().for_each(|(idx, x)| {
                    outgoings
                        .entry(x.clone())
                        .or_insert_with(|| Outgoing::empty())
                        .merge_by(idx as u8, tx.clone());
                });
            }
            _ => (),
        }
    }

    let for_sv = {
        let mut map = HashMap::new();
        starters.iter().for_each(|(k, v)| {
            map.insert(k.clone(), v.clone());
        });
        outgoings.iter().for_each(|(k, v)| {
            map.insert(k.clone(), v.clone());
        });
        map
    };

    (
        Incomings { map: incomings },
        Outgoings { map: outgoings },
        Awaiter {
            inner: IncomingInner::from_rx_and_count(awaiter_rx, awaiter_upsteram_count),
        },
        Starters { map: starters },
        SignalOutgoings { map: for_sv },
    )
}
