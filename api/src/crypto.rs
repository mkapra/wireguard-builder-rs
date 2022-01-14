//! Module that represents the crypto part in this crate
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::models::JwtUser;

/// The secret key for `JWT`s
#[derive(Clone, Debug)]
pub struct SecretKey(pub String);

impl Display for SecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Data that is encoded into the `JWT`
#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub user: JwtUser,
}

impl Claims {
    pub fn new(user: &JwtUser) -> Self {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
            user: user.clone(),
        }
    }
}
