use crate::{Flagment, Flagments};
use regex::Regex;
use toy_text_parser::Line;

pub struct RegexParser {
    line_regex: Regex,
    span_regex: Regex,
    time_regex: Regex,
}

impl RegexParser {
    pub fn new() -> Result<RegexParser, regex::Error> {
        let line_regex = Regex::new(
            r"(?x)
    (?P<datetime>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d{9}[+-]\d{2}:\d{2})\s+
    (?P<level>\S+)\s+
    (?P<thread_name>\S+)\s+
    (?P<thread_id>\S+)\s+
    (?P<span>.*):\s+
    (?P<target>\S+):\s+
    (?P<message>.*)
    ",
        )?;

        let span_regex = Regex::new(
            r"(?x)Task\{\s*task=(?P<task_id>\S+)\s*(graph=(?P<graph>\S+))?\s*(uri=(?P<uri>\S+))?\s*}",
        )?;
        let time_regex = Regex::new(r"(?x)time\.busy=(?P<busy>\S+)\s*time\.idle=(?P<idle>\S+)")?;

        Ok(RegexParser {
            line_regex,
            span_regex,
            time_regex,
        })
    }

    pub fn parse<'a>(&self, line: &'a Line) -> Flagments<'a> {
        let text = std::str::from_utf8(line.get(0).unwrap()).unwrap();
        let mut fl = Flagments::new();

        match self.line_regex.captures(text) {
            Some(m) => {
                fl.set_datetime(m.name("datetime").map(|x| Flagment::Datetime(x.as_str())));
                fl.set_level(m.name("level").map(|x| Flagment::Level(x.as_str())));
                fl.set_thread_name(
                    m.name("thread_name")
                        .map(|x| Flagment::ThreadName(x.as_str())),
                );
                fl.set_thread_name(m.name("thread_id").map(|x| Flagment::ThreadId(x.as_str())));

                match m.name("span") {
                    Some(m) => {
                        fl.set_span(Some(Flagment::Span(m.as_str())));
                        match self.span_regex.captures(m.as_str()) {
                            Some(m) => {
                                fl.set_task_id(
                                    m.name("task_id").map(|x| Flagment::TaskId(x.as_str())),
                                );
                                fl.set_graph(m.name("graph").map(|x| Flagment::Graph(x.as_str())));
                                fl.set_uri(m.name("uri").map(|x| Flagment::Uri(x.as_str())));
                            }
                            None => (),
                        }
                    }
                    None => (),
                }

                fl.set_target(m.name("target").map(|x| Flagment::Target(x.as_str())));

                match m.name("message") {
                    Some(m) => {
                        fl.set_message(Some(Flagment::Message(m.as_str())));
                        match self.time_regex.captures(m.as_str()) {
                            Some(m) => {
                                fl.set_node_busy_time(
                                    m.name("busy").map(|x| Flagment::NodeBusyTime(x.as_str())),
                                );
                                fl.set_node_idle_time(
                                    m.name("idle").map(|x| Flagment::NodeIdleTime(x.as_str())),
                                );
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
            }
            None => (),
        }
        fl
    }
}
