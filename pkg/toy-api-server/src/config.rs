//! Config for api server.

/// The traits that the config for api server implements.
pub trait ServerConfig {
    fn cert_path(&self) -> String;

    fn key_path(&self) -> String;

    fn pub_path(&self) -> String;
}
