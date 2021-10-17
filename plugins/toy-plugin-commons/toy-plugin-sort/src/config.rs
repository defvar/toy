use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toy_pack::Schema;

pub const fn default_capacity() -> u32 {
    10000
}

/// What to do when the buffer is full
#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub enum BufferFullStrategy {
    /// Execute a sort with only the data in the buffer,
    /// send the data to the next service, and then clear the buffer.
    Flush,

    /// When the buffer is full, it will save the contents to a file and clear the buffer.
    /// After the data reaches the end, it retrieves the data again from the saved contents and sends the data to the next service after sorting.
    Persist { temp_path: PathBuf },
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub enum SortKey {
    /// Use the value itself as a key. This is useful when the payload is a number or character.
    Value,

    /// When the payload is a map structure, the value associated with the specified key is used as the key.
    /// This can be used when you want to sort by a specific field value as a key.
    Name(String),

    /// When the payload is an array structure, the value stored in the specified index will be used as the key.
    /// This can be used when you want to sort by a specific value as a key.
    Index(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct SortConfig {
    #[serde(default = "default_capacity")]
    buffer_capacity: u32,
    #[serde(default)]
    buffer_full_strategy: BufferFullStrategy,
    #[serde(default)]
    sort_key: SortKey,
}

impl SortConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(
        buffer_capacity: u32,
        buffer_full_strategy: BufferFullStrategy,
        sort_key: SortKey,
    ) -> Self {
        Self {
            buffer_capacity,
            buffer_full_strategy,
            sort_key,
            ..Default::default()
        }
    }

    pub fn buffer_capacity(&self) -> u32 {
        self.buffer_capacity
    }

    pub fn buffer_full_strategy(&self) -> &BufferFullStrategy {
        &self.buffer_full_strategy
    }

    pub fn sort_key(&self) -> &SortKey {
        &self.sort_key
    }
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            buffer_capacity: default_capacity(),
            buffer_full_strategy: BufferFullStrategy::default(),
            sort_key: SortKey::default(),
        }
    }
}

impl Default for BufferFullStrategy {
    fn default() -> Self {
        BufferFullStrategy::Flush
    }
}

impl Default for SortKey {
    fn default() -> Self {
        SortKey::Value
    }
}
