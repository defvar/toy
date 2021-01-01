#![allow(unused)]

// common
pub const JWK_URL: &'static str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
pub const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
pub const REVOKE_TOKEN_URL: &'static str = "https://accounts.google.com/o/oauth2/revoke?token=";

pub const TOKEN_REQUEST_GRANT_TYPE: &'static str =
    "urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer";

// credential
pub const ENV_KEY_GOOGLE_API_CREDENTIALS: &'static str = "TOY_GOOGLE_API_CREDENTIALS";

// firebase
pub const FIREBASE_VALID_ISS_PREFIX: &'static str = "https://securetoken.google.com";
pub const ENV_KEY_FIREBASE_ID: &'static str = "TOY_FIREBASE_PROJECT_ID";
