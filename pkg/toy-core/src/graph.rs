use crate::data::Value;
use crate::error::ServiceError;
use crate::service_id::ServiceId;
use std::rc::Rc;

#[derive(Debug)]
pub struct Graph {
    node: Node,
}

impl Graph {
    pub fn from(v: Value) -> Result<Graph, ServiceError> {
        let mut r: Node = Node(
            Inner {
                id: "__root__".into(),
                config_value: v.clone(),
            },
            Vec::new(),
        );

        let seq = match v {
            Value::Map(ref map) => match map.get("services") {
                Some(services) => match Graph::try_traverse_services(services) {
                    Ok(seq) => seq,
                    Err(e) => return Err(e),
                },
                None => {
                    return Err(ServiceError::error(
                        "invalid config. not found key:[services]",
                    ))
                }
            },
            _ => return Err(ServiceError::error("invalid config.")),
        };
        r.1.extend_from_slice(&seq);
        Ok(Graph { node: r })
    }

    fn try_traverse_services(v: &Value) -> Result<Vec<Rc<Node>>, ServiceError> {
        let mut r: Vec<Rc<Node>> = Vec::new();
        match v {
            Value::Seq(ref seq) => {
                for v in seq {
                    match Graph::try_traverse_service(v) {
                        Ok(Some(n)) => r.push(Rc::new(n)),
                        Ok(None) => (),
                        Err(e) => return Err(e),
                    }
                }
            }
            _ => (),
        };
        Ok(r)
    }

    fn try_traverse_service(v: &Value) -> Result<Option<Node>, ServiceError> {
        match v {
            Value::Map(ref map) => {
                let kind = match map.get("kind") {
                    Some(kind) => match kind {
                        Value::String(k) => k,
                        _ => {
                            return Err(ServiceError::error(
                                "invalid config. [kind] must be String.",
                            ))
                        }
                    },
                    None => {
                        return Err(ServiceError::error("invalid config. not found key:[kind]."))
                    }
                };
                Ok(Some(Node(
                    Inner {
                        id: kind.into(),
                        config_value: v.clone(),
                    },
                    Vec::new(),
                )))
            }
            _ => {
                return Err(ServiceError::error(
                    "invalid config. [service] must be map.",
                ))
            }
        }
    }
}

#[derive(Debug)]
struct Node(Inner, Vec<Rc<Node>>);

#[derive(Debug)]
struct Inner {
    id: ServiceId,
    config_value: Value,
}
