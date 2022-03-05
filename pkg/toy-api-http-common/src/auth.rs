#[derive(Debug, Clone)]
pub struct Auth {
    user: String,
    bearer_token: Option<String>,
}

impl Auth {
    pub fn new<T: Into<String>>(user: T) -> Auth {
        Self {
            user: user.into(),
            bearer_token: None,
        }
    }

    /// credential is 'Authorization: Bearer {token}' of Http Header.
    pub fn with_bearer_token<T: Into<String>>(user: T, credential: T) -> Auth {
        Self {
            user: user.into(),
            bearer_token: Some(credential.into()),
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn bearer_token(&self) -> Option<&str> {
        self.bearer_token.as_ref().map(|x| x.as_str())
    }
}
