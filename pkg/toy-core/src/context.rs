pub trait Context {
    type Config;

    fn set_config(&mut self, config: Self::Config);
}
