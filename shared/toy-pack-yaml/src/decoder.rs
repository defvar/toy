use std::f64;
use std::fmt;

use yaml_rust::parser::{Event as YamlEvent, MarkedEventReceiver, Parser};
use yaml_rust::scanner::{Marker, TScalarStyle, TokenType};
use yaml_rust::Yaml;

use toy_pack::deser::Error;

use super::error::YamlError;

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Alias(usize),
    Scalar(String, TScalarStyle, Option<TokenType>),
    SequenceStart,
    SequenceEnd,
    MappingStart,
    MappingEnd,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Event::*;
        match *self {
            Alias(s) => write!(f, "Alias({:?})", s),
            Scalar(ref s, style, ref token) => write!(f, "Scalar({:?},{:?},{:?})", s, style, token),
            SequenceStart => write!(f, "SequenceStart"),
            SequenceEnd => write!(f, "SequenceEnd"),
            MappingStart => write!(f, "MappingStart"),
            MappingEnd => write!(f, "MappingEnd"),
        }
    }
}

pub struct Loader {
    events: Vec<(Event, Marker)>,
}

impl MarkedEventReceiver for Loader {
    fn on_event(&mut self, event: YamlEvent, marker: Marker) {
        let event = match event {
            YamlEvent::Nothing
            | YamlEvent::StreamStart
            | YamlEvent::StreamEnd
            | YamlEvent::DocumentStart
            | YamlEvent::DocumentEnd => return,

            YamlEvent::Alias(id) => Event::Alias(id),
            YamlEvent::Scalar(value, style, _, tag) => Event::Scalar(value, style, tag),
            YamlEvent::SequenceStart(_) => Event::SequenceStart,
            YamlEvent::SequenceEnd => Event::SequenceEnd,
            YamlEvent::MappingStart(_) => Event::MappingStart,
            YamlEvent::MappingEnd => Event::MappingEnd,
        };
        self.events.push((event, marker));
    }
}

pub struct Decoder {
    events: Vec<(Event, Marker)>,
    pos: usize,
}

impl Decoder {
    pub fn from_str(s: &str) -> Result<Decoder, YamlError> {
        let mut parser = Parser::new(s.chars());
        let mut loader = Loader { events: Vec::new() };
        parser.load(&mut loader, true)?;

        Ok(Decoder {
            events: loader.events,
            pos: 0,
        })
    }

    pub fn peek(&self) -> Result<(&Event, Marker), YamlError> {
        match self.events.get(self.pos) {
            Some(event) => Ok((&event.0, event.1)),
            None => Err(YamlError::error("end of stream")),
        }
    }

    pub fn peek_is_null(&self) -> Result<bool, YamlError> {
        let (e, _) = self.peek()?;
        match *e {
            Event::Scalar(_, _, _) => match Decoder::scalar(e) {
                Ok(v) => Ok(v.is_null()),
                Err(e) => Err(e),
            },
            _ => Ok(false),
        }
    }

    pub fn next(&mut self) -> Result<(&Event, Marker), YamlError> {
        self.opt_next().ok_or_else(|| YamlError::error("end of stream"))
    }

    fn opt_next(&mut self) -> Option<(&Event, Marker)> {
        let a = self.events.get(self.pos);
        match a {
            Some(v) => {
                self.pos += 1;
                Some((&v.0, v.1))
            }
            None => None,
        }
    }

    pub fn decode_bool(&mut self) -> Result<bool, YamlError> {
        self.next().and_then(|x| match Decoder::scalar(x.0) {
            Ok(v) => v.as_bool().ok_or_else(|| YamlError::invalid_value(x.0, "bool")),
            Err(e) => Err(e),
        })
    }

    pub fn decode_int(&mut self) -> Result<i64, YamlError> {
        self.next().and_then(|x| match Decoder::scalar(x.0) {
            Ok(v) => v.as_i64().ok_or_else(|| YamlError::invalid_value(x.0, "i64")),
            Err(e) => Err(e),
        })
    }

    pub fn decode_float(&mut self) -> Result<f64, YamlError> {
        self.next().and_then(|x| match Decoder::scalar(x.0) {
            Ok(v) => v.as_f64().ok_or_else(|| YamlError::invalid_value(x.0, "float")),
            Err(e) => Err(e),
        })
    }

    pub fn decode_string(&mut self) -> Result<String, YamlError> {
        self.next().and_then(|x| match Decoder::scalar(x.0) {
            Ok(v) => v
                .into_string()
                .ok_or_else(|| YamlError::invalid_value(format!("{{ event:{}, marker:{:?} }}", x.0, x.1), "string")),
            Err(e) => Err(e),
        })
    }

    pub fn decode_null(&mut self) -> Result<(), YamlError> {
        if self.peek_is_null()? {
            let _ = self.next()?;
            Ok(())
        } else {
            Err(YamlError::invalid_type("null"))
        }
    }

    fn scalar(ev: &Event) -> Result<Yaml, YamlError> {
        let r = match *ev {
            Event::Scalar(ref v, style, ref tag) => {
                if style != TScalarStyle::Plain {
                    Yaml::String(v.to_owned())
                } else if let Some(TokenType::Tag(ref handle, ref suffix)) = tag {
                    // XXX tag:yaml.org,2002:
                    if handle == "!!" {
                        match suffix.as_ref() {
                            "bool" => {
                                // "true" or "false"
                                match v.parse::<bool>() {
                                    Err(_) => Yaml::BadValue,
                                    Ok(v) => Yaml::Boolean(v),
                                }
                            }
                            "int" => match v.parse::<i64>() {
                                Err(_) => Yaml::BadValue,
                                Ok(v) => Yaml::Integer(v),
                            },
                            "float" => match parse_f64(&v) {
                                Some(_) => Yaml::Real(v.to_owned()),
                                None => Yaml::BadValue,
                            },
                            "null" => match v.as_ref() {
                                "~" | "null" => Yaml::Null,
                                _ => Yaml::BadValue,
                            },
                            _ => Yaml::String(v.to_owned()),
                        }
                    } else {
                        Yaml::String(v.to_owned())
                    }
                } else {
                    // Datatype is not specified, or unrecognized
                    Yaml::from_str(&v)
                }
            }
            _ => return Err(Error::invalid_type(format!("not scalar type. event:{:?}", *ev))),
        };
        Ok(r)
    }
}

fn parse_f64(v: &str) -> Option<f64> {
    match v {
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Some(f64::INFINITY),
        "-.inf" | "-.Inf" | "-.INF" => Some(f64::NEG_INFINITY),
        ".nan" | "NaN" | ".NAN" => Some(f64::NAN),
        _ => v.parse::<f64>().ok(),
    }
}
