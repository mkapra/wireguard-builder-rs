//! Holds all the necessary things for a `User`
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn get_by_name(connection: &DatabaseConnection, name: String) -> Result<Self> {
        use crate::schema::users::dsl::*;

        users
            .filter(username.eq(username))
            .first::<User>(connection)
            .map_err(|e| {
                Error::new(format!(
                    "Could not find user with username '{}' ({:?})",
                    name, e
                ))
            })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtUser {
    pub id: i32,
    pub username: String,
}

impl From<User> for JwtUser {
    fn from(u: User) -> Self {
        JwtUser {
            id: u.id,
            username: u.username,
        }
    }
}