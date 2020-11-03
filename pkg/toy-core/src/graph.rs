use crate::data::Value;
use crate::error::ConfigError;
use crate::service_type::ServiceType;
use crate::service_uri::Uri;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Graph {
    name: String,
    nodes: Vec<Arc<Node>>,
    outputs: HashMap<Uri, OutputWire>,
    inputs: HashMap<Uri, InputWire>,
    config_value: Value,
}

#[derive(Debug, Clone)]
pub struct Node(Inner);

#[derive(Debug, Clone)]
struct Inner {
    tp: ServiceType,
    uri: Uri,
    config_value: Value,
}

impl Graph {
    pub fn from(v: Value) -> Result<Graph, ConfigError> {
        let name = match v {
            Value::Map(ref map) => match map.get("name") {
                Some(name) => name.as_str().unwrap(),
                None => return Err(ConfigError::not_found_key("name")),
            },
            _ => {
                return Err(ConfigError::error(
                    "invalid config. config value must be map type.",
                ))
            }
        };
        let seq = match v {
            Value::Map(ref map) => match map.get("services") {
                Some(services) => Graph::try_traverse_services(services)?,
                None => return Err(ConfigError::not_found_key("services")),
            },
            _ => {
                return Err(ConfigError::error(
                    "invalid config. config value must be map type.",
                ))
            }
        };

        let config_value = get_config_value(&v)?;

        Ok(Graph {
            name: name.to_string(),
            nodes: Vec::<Arc<Node>>::from(seq.0),
            outputs: HashMap::<Uri, OutputWire>::from(seq.1),
            inputs: HashMap::<Uri, InputWire>::from(seq.2),
            config_value,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn by_uri<U: AsRef<Uri>>(&self, uri: U) -> Option<Arc<Node>> {
        self.nodes
            .iter()
            .find(|x| x.uri() == *uri.as_ref())
            .map(|x| x.clone())
    }

    pub fn iter(&self) -> NodeIterator {
        let len = self.nodes.len();
        NodeIterator {
            graph: self,
            cur: 0,
            len,
        }
    }

    pub fn outputs(&self) -> &HashMap<Uri, OutputWire> {
        &self.outputs
    }

    pub fn inputs(&self) -> &HashMap<Uri, InputWire> {
        &self.inputs
    }

    pub fn config(&self) -> Value {
        self.config_value.clone()
    }

    fn try_traverse_services(
        v: &Value,
    ) -> Result<
        (
            Vec<Arc<Node>>,
            HashMap<Uri, OutputWire>,
            HashMap<Uri, InputWire>,
        ),
        ConfigError,
    > {
        let mut nodes: Vec<Arc<Node>> = Vec::new();
        let mut output_wires: HashMap<Uri, OutputWire> = HashMap::new();
        let mut input_wires: HashMap<Uri, InputWire> = HashMap::new();

        fn push_input_wire_from_output(
            me: &Uri,
            other: Uri,
            input_wires: &mut HashMap<Uri, InputWire>,
        ) {
            let new_wire = if input_wires.contains_key(&other) {
                let v = input_wires.get(&other).unwrap();
                v.put_output(me.clone())
            } else {
                InputWire::Single(me.clone(), other.clone())
            };
            input_wires.insert(other.clone(), new_wire);
        }

        match v {
            Value::Seq(ref seq) => {
                for v in seq {
                    let (n, w) = Graph::try_traverse_service(v)?;
                    output_wires.insert(n.uri(), w.clone());
                    match w {
                        OutputWire::Single(o, i) => {
                            push_input_wire_from_output(&o, i, &mut input_wires);
                        }
                        OutputWire::Fanout(me, outs) => {
                            for o in outs {
                                push_input_wire_from_output(&me, o, &mut input_wires);
                            }
                        }
                        _ => (),
                    };
                    nodes.push(Arc::new(n));
                }
            }
            _ => (),
        };

        for (uri, _) in &output_wires {
            input_wires.entry(uri.clone()).or_insert(InputWire::None);
        }

        Ok((nodes, output_wires, input_wires))
    }

    fn try_traverse_service(v: &Value) -> Result<(Node, OutputWire), ConfigError> {
        if !v.is_map() {
            return Err(ConfigError::invalid_key_type("service", "map"));
        }
        let map = v.as_map().unwrap();

        let tp = match map.get("type") {
            Some(kind) => match kind {
                Value::String(k) => k,
                _ => return Err(ConfigError::invalid_key_type("type", "String")),
            },
            None => return Err(ConfigError::not_found_key("type")),
        };
        let uri = match map.get("uri") {
            Some(uri) => match uri {
                Value::String(v) => v,
                _ => return Err(ConfigError::invalid_key_type("uri", "String")),
            },
            None => return Err(ConfigError::not_found_key("uri")),
        };

        let config_value = get_config_value(&v)?;

        let wire = match map.get("wires") {
            Some(wires) => match wires {
                Value::None => OutputWire::None,
                Value::String(v) => OutputWire::Single(uri.into(), v.into()),
                Value::Seq(ref seq) => {
                    let wires = seq
                        .iter()
                        .filter_map(|x| {
                            (match x {
                                Value::String(v) => Ok(Uri::from(v)),
                                _ => Err(ConfigError::invalid_key_type(
                                    "wires",
                                    "String or Seq(element:String) or None",
                                )),
                            })
                            .ok()
                        })
                        .collect::<Vec<_>>();
                    OutputWire::Fanout(uri.into(), wires)
                }
                _ => {
                    return Err(ConfigError::invalid_key_type(
                        "wires",
                        "String or Seq(element:String) or None",
                    ))
                }
            },
            None => return Err(ConfigError::not_found_key("wires")),
        };
        match ServiceType::from_full_name(tp) {
            Ok(st) => Ok((Node::new(st, uri.into(), config_value), wire)),
            Err(e) => Err(e),
        }
    }
}

fn get_config_value(v: &Value) -> Result<Value, ConfigError> {
    match v {
        Value::Map(ref map) => match map.get("config") {
            Some(c) => Ok(c.clone()),
            None => Ok(Value::None),
        },
        _ => Err(ConfigError::error(
            "invalid config. config value must be map type.",
        )),
    }
}

pub struct NodeIterator<'a> {
    graph: &'a Graph,
    cur: usize,
    len: usize,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Arc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.len {
            None
        } else {
            let e = self.graph.nodes.get(self.cur).map(|x| x.clone());
            self.cur += 1;
            e
        }
    }
}

impl<'a> DoubleEndedIterator for NodeIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Arc<Node>> {
        if self.cur == self.len {
            None
        } else {
            self.len -= 1;
            self.graph.nodes.get(self.len).map(|x| x.clone())
        }
    }
}

impl Node {
    pub fn new(tp: ServiceType, uri: Uri, config_value: Value) -> Node {
        Node(Inner {
            tp,
            uri,
            config_value,
        })
    }

    pub fn uri(&self) -> Uri {
        self.0.uri.clone()
    }

    pub fn service_type(&self) -> ServiceType {
        self.0.tp.clone()
    }

    pub fn config(&self) -> Value {
        self.0.config_value.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputWire {
    /// Wire of "One to One".
    /// Sender is "0" Value of Tuple.
    /// "0" of Tuple is Me.
    ///
    Single(Uri, Uri),

    /// Wire of "One To Many".
    ///
    Fanout(Uri, Vec<Uri>),

    /// Without output.
    ///
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputWire {
    /// Wire of "One to One".
    /// Sender is "0" of Tuple.
    /// "1" of Tuple is Me.
    ///
    Single(Uri, Uri),

    /// Wire of "Many to One".
    ///
    Fanin(Vec<Uri>, Uri),

    /// Without Input.
    ///
    None,
}

impl InputWire {
    pub fn put_output(&self, uri: Uri) -> InputWire {
        match self {
            InputWire::Single(o, i) => InputWire::Fanin(vec![o.clone(), uri], i.clone()),
            InputWire::Fanin(o, i) => {
                let mut o2 = Vec::from(o.clone());
                o2.push(uri);
                InputWire::Fanin(o2, i.clone())
            }
            InputWire::None => unimplemented!(),
        }
    }
}
