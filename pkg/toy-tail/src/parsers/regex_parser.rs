use crate::parsers::Parser;
use crate::{Flagment, Flagments};
use regex::Regex;
use toy_text_parser::Line;

pub struct RegexParser {
    regex: Regex,
}

impl RegexParser {
    pub fn new(pattern: &str) -> Result<RegexParser, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(RegexParser { regex })
    }
}

impl Parser for RegexParser {
    fn parse<'a>(&self, line: &'a Line) -> Flagments<'a> {
        let mut fl = Flagments::new();
        let bytes = match line.get(0) {
            Some(d) => d,
            None => return fl,
        };

        let text = std::str::from_utf8(bytes).unwrap();

        match self.regex.captures(text) {
            Some(m) => {
                fl.set_datetime(m.name("datetime").map(|x| Flagment::Datetime(x.as_str())));
                fl.set_level(m.name("level").map(|x| Flagment::Level(x.as_str())));
                fl.set_thread_name(
                    m.name("thread_name")
                        .map(|x| Flagment::ThreadName(x.as_str())),
                );
                fl.set_thread_id(m.name("thread_id").map(|x| Flagment::ThreadId(x.as_str())));
                fl.set_task_id(m.name("task_id").map(|x| Flagment::TaskId(x.as_str())));
                fl.set_graph(m.name("graph").map(|x| Flagment::Graph(x.as_str())));
                fl.set_uri(m.name("uri").map(|x| Flagment::Uri(x.as_str())));
                fl.set_target(m.name("target").map(|x| Flagment::Target(x.as_str())));
                fl.set_message(m.name("message").map(|x| Flagment::Message(x.as_str())));
                fl.set_node_busy_time(m.name("busy").map(|x| Flagment::NodeBusyTime(x.as_str())));
                fl.set_node_idle_time(m.name("idle").map(|x| Flagment::NodeIdleTime(x.as_str())));
            }
            None => (),
        };
        fl
    }
}
