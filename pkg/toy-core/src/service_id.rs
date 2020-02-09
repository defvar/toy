#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId {
    id: String,
}

impl From<&str> for ServiceId {
    fn from(v: &str) -> Self {
        ServiceId { id: v.to_string() }
    }
}

impl From<String> for ServiceId {
    fn from(v: String) -> Self {
        ServiceId { id: v }
    }
}

impl From<&String> for ServiceId {
    fn from(v: &String) -> Self {
        ServiceId { id: v.to_string() }
    }
}
