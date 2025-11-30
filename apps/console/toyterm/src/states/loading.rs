use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadingState {
    index: i8,
    last_tick: std::time::Instant,
    tick_rate: std::time::Duration,
    symbols: Vec<String>,
    empty: String,
}

impl Default for LoadingState {
    fn default() -> Self {
        Self {
            index: 0,
            last_tick: std::time::Instant::now(),
            tick_rate: std::time::Duration::from_millis(250),
            symbols: vec![
                "|".to_owned(),
                "/".to_owned(),
                "-".to_owned(),
                "\\".to_owned(),
            ],
            empty: "".to_owned(),
        }
    }
}

impl LoadingState {
    pub fn index(&self) -> i8 {
        self.index
    }

    pub fn symbols(&self) -> &[String] {
        self.symbols.as_slice()
    }

    pub fn empty(&self) -> String {
        self.empty.clone()
    }

    pub fn tick(&mut self) {
        if self.last_tick.elapsed() >= self.tick_rate {
            self.calc_step(1);
            self.last_tick = std::time::Instant::now();
        }
    }

    pub fn calc_step(&mut self, step: i8) {
        self.index = if step == 0 {
            let mut rng = rand::rng();
            rng.random()
        } else {
            self.index.checked_add(step).unwrap_or(0)
        }
    }

    pub fn normalize(&mut self) {
        let len = self.symbols.len() as i8;
        if len <= 0 {
            //ng but it's not used, so it stays.
        } else {
            self.index %= len;
            if self.index < 0 {
                // Negative numbers are indexed from the tail
                self.index += len;
            }
        }
    }
}
