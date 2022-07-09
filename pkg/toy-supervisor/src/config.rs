pub trait SupervisorConfig {
    fn heart_beat_interval_secs(&self) -> u64;

    fn cert_path(&self) -> String;

    fn key_path(&self) -> String;

    fn pub_path(&self) -> String;
}
