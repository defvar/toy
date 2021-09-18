use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Kind {
    User,
    ServiceAccount,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    kind: Kind,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleBinding {
    name: String,
    role: String,
    subjects: Vec<Subject>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn name(&self) -> &str {
        &self.name
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn subjects(&self) -> &Vec<Subject> {
        &self.subjects
    }
}

impl RoleBindingList {
    pub fn new(items: Vec<RoleBinding>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

pub struct RoleBindingBuilder {
    e: RoleBinding,
}

impl RoleBindingBuilder {
    pub fn new<P: Into<String>>(name: P) -> RoleBindingBuilder {
        RoleBindingBuilder {
            e: RoleBinding {
                name: name.into(),
                role: "".to_owned(),
                subjects: vec![],
            },
        }
    }

    pub fn role<P: Into<String>>(mut self, role: P) -> RoleBindingBuilder {
        self.e.role = role.into();
        self
    }

    pub fn service_account<P: Into<String>>(mut self, name: P) -> RoleBindingBuilder {
        self.e
            .subjects
            .push(Subject::new(Kind::ServiceAccount, name.into()));
        self
    }

    pub fn user<P: Into<String>>(mut self, name: P) -> RoleBindingBuilder {
        self.e.subjects.push(Subject::new(Kind::User, name.into()));
        self
    }

    pub fn subject(mut self, subject: Subject) -> RoleBindingBuilder {
        self.e.subjects.push(subject);
        self
    }

    pub fn subjects(mut self, subjects: Vec<Subject>) -> RoleBindingBuilder {
        self.e.subjects = subjects;
        self
    }

    pub fn build(self) -> RoleBinding {
        self.e
    }
}
