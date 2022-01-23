//! Holds all the necessary things for a `User`
use super::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

impl User {
    /// Creates a new [`GraphQLUser`] in the database
    pub fn create(connection: &DatabaseConnection, username: String) -> Result<GraphQLUser> {
        let password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let hashed_password = bcrypt::hash(&password, 8).map_err(Error::from)?;
        let new_user = GraphQLUser {
            username,
            password: hashed_password,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(connection)
            .map(|r: User| GraphQLUser::new(r.username, password))
            .map_err(Error::from)
    }

    pub fn get_by_name(connection: &DatabaseConnection, name: String) -> Result<Self> {
        use crate::schema::users::dsl::*;

        users
            .filter(username.eq(&name))
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

#[derive(Debug, SimpleObject, Insertable)]
#[graphql(name = "User")]
#[table_name = "users"]
pub struct GraphQLUser {
    pub username: String,
    pub password: String,
}

impl GraphQLUser {
    fn new(username: String, password: String) -> Self {
        GraphQLUser { username, password }
    }
}
