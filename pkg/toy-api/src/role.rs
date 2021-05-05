use toy_pack::{Pack, Unpack};

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Rule {
    resources: Vec<String>,
    verbs: Vec<String>,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct Role {
    name: String,
    note: Option<String>,
    rules: Vec<Rule>,
}

#[derive(Clone, Debug, Pack, Unpack)]
pub struct RoleList {
    items: Vec<Role>,
    count: u32,
}

impl Rule {
    pub fn new(resources: Vec<String>, verbs: Vec<String>) -> Self {
        Self { resources, verbs }
    }
}

impl Role {
    pub fn new(name: String, note: Option<String>, rules: Vec<Rule>) -> Self {
        Self { name, note, rules }
    }
}

impl RoleList {
    pub fn new(items: Vec<Role>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}
