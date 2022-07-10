mod logger;
pub mod tcp;

pub use logger::{
    console, console_with_addr, file, tcp, LogGuard, LogRotation, CONSOLE_DEFAULT_IP,
    CONSOLE_DEFAULT_PORT,
};
