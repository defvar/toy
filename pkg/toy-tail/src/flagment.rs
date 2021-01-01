use std::fmt;

#[derive(Clone, Debug)]
pub enum Flagment<'a> {
    Datetime(&'a str),
    Level(&'a str),
    ThreadName(&'a str),
    ThreadId(&'a str),
    Span(&'a str),
    TaskId(&'a str),
    Graph(&'a str),
    Uri(&'a str),
    Target(&'a str),
    Message(&'a str),
    NodeBusyTime(&'a str),
    NodeIdleTime(&'a str),
}

#[derive(Clone, Debug)]
pub struct Flagments<'a> {
    datetime: Option<Flagment<'a>>,
    level: Option<Flagment<'a>>,
    thread_name: Option<Flagment<'a>>,
    thread_id: Option<Flagment<'a>>,
    span: Option<Flagment<'a>>,
    task_id: Option<Flagment<'a>>,
    graph: Option<Flagment<'a>>,
    uri: Option<Flagment<'a>>,
    target: Option<Flagment<'a>>,
    message: Option<Flagment<'a>>,
    node_busy_time: Option<Flagment<'a>>,
    node_idle_time: Option<Flagment<'a>>,

    is_some: bool,
}

impl<'a> Flagment<'a> {
    pub fn value(&self) -> &'a str {
        match *self {
            Flagment::Datetime(v) => v,
            Flagment::Level(v) => v,
            Flagment::ThreadName(v) => v,
            Flagment::ThreadId(v) => v,
            Flagment::Span(v) => v,
            Flagment::TaskId(v) => v,
            Flagment::Graph(v) => v,
            Flagment::Uri(v) => v,
            Flagment::Target(v) => v,
            Flagment::Message(v) => v,
            Flagment::NodeBusyTime(v) => v,
            Flagment::NodeIdleTime(v) => v,
        }
    }
}

macro_rules! getter {
    ($name: ident) => {
        pub fn $name(&self) -> Option<&'a str> {
            self.$name.as_ref().map(|x| x.value())
        }
    };
}

macro_rules! setter {
    ($name: ident, $field: ident, $variant: ident) => {
        pub fn $name(&mut self, flagment: Option<Flagment<'a>>) {
            match flagment {
                Some(fl) => match fl {
                    Flagment::$variant(_) => {
                        self.$field = Some(fl);
                        self.is_some = true;
                    }
                    _ => (),
                },
                None => (),
            }
        }
    };
}

impl<'a> Flagments<'a> {
    pub fn new() -> Self {
        Self {
            datetime: None,
            level: None,
            thread_name: None,
            thread_id: None,
            span: None,
            task_id: None,
            graph: None,
            uri: None,
            target: None,
            message: None,
            node_busy_time: None,
            node_idle_time: None,
            is_some: false,
        }
    }

    pub fn is_some(&self) -> bool {
        self.is_some
    }

    getter!(datetime);
    getter!(level);
    getter!(thread_name);
    getter!(thread_id);
    getter!(span);
    getter!(task_id);
    getter!(graph);
    getter!(uri);
    getter!(target);
    getter!(message);
    getter!(node_busy_time);
    getter!(node_idle_time);

    setter!(set_datetime, datetime, Datetime);
    setter!(set_level, level, Level);
    setter!(set_thread_name, thread_name, ThreadName);
    setter!(set_thread_id, thread_id, ThreadId);
    setter!(set_span, span, Span);
    setter!(set_task_id, task_id, TaskId);
    setter!(set_graph, graph, Graph);
    setter!(set_uri, uri, Uri);
    setter!(set_target, target, Target);
    setter!(set_message, message, Message);
    setter!(set_node_busy_time, node_busy_time, NodeBusyTime);
    setter!(set_node_idle_time, node_idle_time, NodeIdleTime);
}

impl fmt::Display for Flagment<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Flagment::Datetime(v) => f.write_fmt(format_args!("datetime=\"{}\"", v)),
            Flagment::Level(v) => f.write_fmt(format_args!("level={}", v)),
            Flagment::ThreadName(v) => f.write_fmt(format_args!("thread_name=\"{}\"", v)),
            Flagment::ThreadId(v) => f.write_fmt(format_args!("thread_id=\"{}\"", v)),
            Flagment::Span(v) => f.write_fmt(format_args!("span=\"{}\"", v)),
            Flagment::TaskId(v) => f.write_fmt(format_args!("task_id=\"{}\"", v)),
            Flagment::Graph(v) => f.write_fmt(format_args!("graph={}", v)),
            Flagment::Uri(v) => f.write_fmt(format_args!("uri={}", v)),
            Flagment::Target(v) => f.write_fmt(format_args!("target={}", v)),
            Flagment::Message(v) => f.write_fmt(format_args!("message=\"{}\"", v)),
            Flagment::NodeBusyTime(v) => f.write_fmt(format_args!("node_busy_time=\"{}\"", v)),
            Flagment::NodeIdleTime(v) => f.write_fmt(format_args!("node_idle_time=\"{}\"", v)),
        }
    }
}

fn fmt_flagment(
    f: &mut fmt::Formatter<'_>,
    flagment: Option<&Flagment<'_>>,
    need_delimiter: &mut bool,
) -> std::fmt::Result {
    if flagment.is_some() {
        if *need_delimiter {
            f.write_str(",")?;
        } else {
            *need_delimiter = true;
        }
        f.write_fmt(format_args!("{}", flagment.as_ref().unwrap()))?;
    }
    Ok(())
}

impl fmt::Display for Flagments<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut need_delimiter = false;
        f.write_str("{")?;
        fmt_flagment(f, self.level.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.graph.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.target.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.uri.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.datetime.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.thread_name.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.thread_id.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.task_id.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.message.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.node_busy_time.as_ref(), &mut need_delimiter)?;
        fmt_flagment(f, self.node_idle_time.as_ref(), &mut need_delimiter)?;
        f.write_str("}")
    }
}
