use crate::error::ServiceError;
use crate::graph::{Graph, InputWire, OutputWire};
use crate::mpsc::{self, Incoming, Outgoing};
use crate::prelude::Frame;
use crate::Uri;
use std::collections::HashMap;

const DEFAULT_CHANNEL_BUFFER_SIZE: usize = 128;

#[derive(Debug)]
struct Rx {
    rx: Incoming<Frame, ServiceError>,
    upstream_count: u32,
}

#[derive(Debug)]
pub struct Incomings {
    map: HashMap<Uri, Rx>,
}

#[derive(Debug)]
pub struct Outgoings {
    map: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

#[derive(Debug)]
pub struct Starters {
    map: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

#[derive(Debug)]
pub struct SignalOutgoings {
    map: HashMap<Uri, Outgoing<Frame, ServiceError>>,
}

impl Rx {
    pub fn from_rx(rx: Incoming<Frame, ServiceError>) -> Rx {
        Rx::from_rx_and_count(rx, 1)
    }

    pub fn from_rx_and_count(rx: Incoming<Frame, ServiceError>, upstream_count: u32) -> Rx {
        Rx { rx, upstream_count }
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

impl SignalOutgoings {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Uri, &mut Outgoing<Frame, ServiceError>)> {
        self.map.iter_mut()
    }
}

pub fn from_graph(
    graph: &Graph,
) -> (
    Incomings,
    Outgoings,
    Incoming<Frame, ServiceError>,
    Starters,
    SignalOutgoings,
) {
    let mut incomings: HashMap<Uri, Rx> = HashMap::new();
    let mut outgoings: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

    let mut starters: HashMap<Uri, Outgoing<Frame, ServiceError>> = HashMap::new();

    let (l_tx, l_rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);

    // first channel
    graph
        .inputs()
        .iter()
        .filter(|(_, w)| **w == InputWire::None)
        .for_each(|(uri, _)| {
            let (tx, rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
            incomings.insert(uri.clone(), Rx::from_rx(rx));
            starters.insert(uri.clone(), tx);
        });

    // last channel
    graph
        .outputs()
        .iter()
        .filter(|(_, w)| **w == OutputWire::None)
        .for_each(|(uri, _)| {
            outgoings
                .entry(uri.clone())
                .or_insert_with(|| Outgoing::empty())
                .merge(l_tx.clone());
        });

    for (_, wire) in graph.inputs() {
        let (tx, rx) = mpsc::channel::<Frame, ServiceError>(DEFAULT_CHANNEL_BUFFER_SIZE);
        match wire {
            InputWire::Single(o, i) => {
                incomings.insert(i.clone(), Rx::from_rx(rx));
                outgoings
                    .entry(o.clone())
                    .or_insert_with(|| Outgoing::empty())
                    .merge(tx.clone());
            }
            InputWire::Fanin(o, i) => {
                incomings.insert(i.clone(), Rx::from_rx_and_count(rx, o.len() as u32));
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
        l_rx,
        Starters { map: starters },
        SignalOutgoings { map: for_sv },
    )
}
