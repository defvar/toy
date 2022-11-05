//! Model for rbac/roleBinding api.

use crate::common::{KVObject, ListOption, ListOptionLike, SelectionCandidate};
use crate::selection::candidate::CandidateMap;
use crate::selection::selector::Selector;
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

impl SelectionCandidate for RoleBinding {
    fn candidate_fields() -> &'static [&'static str] {
        &[]
    }

    fn candidate_map(&self) -> CandidateMap {
        CandidateMap::empty()
    }
}

impl KVObject for RoleBinding {
    fn key(&self) -> &str {
        &self.name
    }
}

impl RoleBindingList {
    pub fn new(items: Vec<RoleBinding>) -> Self {
        let count = items.len() as u32;
        Self { items, count }
    }
}

//////////////////////////////////
// Option
//////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleBindingListOption {
    #[serde(flatten)]
    common: ListOption,
}

impl RoleBindingListOption {
    pub fn new() -> Self {
        Self {
            common: ListOption::new(),
        }
    }
}

impl ListOptionLike for RoleBindingListOption {
    fn common(&self) -> &ListOption {
        &self.common
    }

    fn selection(&self) -> &Selector {
        self.common.selection()
    }
}
