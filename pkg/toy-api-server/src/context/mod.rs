use crate::authentication::AuthUser;

pub mod server;

#[derive(Debug, Clone)]
pub struct Context {
    user: AuthUser,
    resource: String,
    verb: String,
}

impl Context {
    pub fn new<T: Into<String>>(user: AuthUser, resource: T, verb: T) -> Self {
        Context {
            user,
            resource: resource.into(),
            verb: verb.into(),
        }
    }

    pub fn user(&self) -> &AuthUser {
        &self.user
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn verb(&self) -> &str {
        &self.verb
    }
}
