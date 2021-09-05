use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Debug)]
pub struct TcpLogger {
    tcp: Option<TcpStream>,
    addr: Vec<SocketAddr>,
}

impl TcpLogger {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, std::io::Error> {
        let st = TcpStream::connect(&addr)?;
        Ok(Self {
            tcp: Some(st),
            addr: addr.to_socket_addrs()?.collect(),
        })
    }
}

impl std::io::Write for TcpLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.tcp.as_ref().map(|mut x| x.write(buf)).unwrap_or(Ok(0))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.tcp.as_ref().map(|mut x| x.flush()).unwrap_or(Ok(()))
    }
}

impl MakeWriter for TcpLogger {
    type Writer = TcpLogger;

    fn make_writer(&self) -> Self::Writer {
        match TcpStream::connect(self.addr.as_slice()) {
            Ok(st) => TcpLogger {
                tcp: Some(st),
                addr: self.addr.clone(),
            },
            Err(e) => {
                eprintln!("error, create TcpLogger{:?}", e);
                TcpLogger {
                    tcp: None,
                    addr: self.addr.clone(),
                }
            }
        }
    }
}
