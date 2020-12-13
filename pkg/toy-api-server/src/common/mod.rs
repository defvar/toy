pub mod body;
pub mod error;
pub mod reply;

pub mod constants {
    pub static GRAPHS_KEY_PREFIX: &'static str = "/graphs";

    pub fn graph_key(part: String) -> String {
        format!("{}/{}", GRAPHS_KEY_PREFIX, part)
    }
}
