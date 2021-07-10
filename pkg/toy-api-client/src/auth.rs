use toy_api::role_binding::Kind;

#[derive(Debug, Clone)]
pub struct Auth {
    user: String,
    kind: Kind,
    bearer_token: Option<String>,
}

impl Auth {
    pub fn new<T: Into<String>>(user: T, kind: Kind) -> Auth {
        Self {
            user: user.into(),
            kind,
            bearer_token: None,
        }
    }

    /// credential is 'Authorization: Bearer {token}' of Http Header.
    pub fn with_bearer_token<T: Into<String>>(user: T, kind: Kind, credential: T) -> Auth {
        Self {
            user: user.into(),
            kind,
            bearer_token: Some(credential.into()),
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn bearer_token(&self) -> Option<&str> {
        self.bearer_token.as_ref().map(|x| x.as_str())
    }
}
