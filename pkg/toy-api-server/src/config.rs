//! Config for api server.

/// The traits that the config for api server implements.
pub trait ServerConfig {
    fn cert_path(&self) -> String;

    fn key_path(&self) -> String;

    fn pub_path(&self) -> String;

    fn tls_secret_key(&self) -> String;

    fn dispatch_interval_mills(&self) -> u64;

    fn clean_supervisor_interval_mills(&self) -> u64;
}
