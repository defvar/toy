use toy_pack::{Pack, Unpack};

#[derive(Clone, Copy, Debug, Pack, Unpack)]
pub enum Kind {
    User,
    ServiceAccount,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Subject {
    kind: Kind,
    name: String,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct RoleBinding {
    name: String,
    role: String,
    subjects: Vec<Subject>,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct RoleBindingList {
    items: Vec<RoleBinding>,
    count: u32,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::User
    }
}

impl Subject {
    pub fn new(kind: Kind, name: String) -> Self {
        Self { kind, name }
    }
}

impl RoleBinding {
    pub fn new(name: String, role: String, subjects: Vec<Subject>) -> Self {
        Self {
            name,
            role,
            subjects,
        }
    }
}

impl RoleBindingList {
    pub fn new(items: Vec<RoleBinding>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}
