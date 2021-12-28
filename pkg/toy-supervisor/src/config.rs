use serde::{Deserialize, Serialize};

const fn default_heart_beat_interval_secs() -> u64 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorConfig {
    #[serde(default = "default_heart_beat_interval_secs")]
    heart_beat_interval_secs: u64,
}

impl SupervisorConfig {
    pub fn heart_beat_interval_secs(&self) -> u64 {
        self.heart_beat_interval_secs
    }

    pub fn with_heart_beat_interval_secs(self, secs: u64) -> Self {
        Self {
            heart_beat_interval_secs: secs,
            ..self
        }
    }
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        SupervisorConfig {
            heart_beat_interval_secs: default_heart_beat_interval_secs(),
        }
    }
}
