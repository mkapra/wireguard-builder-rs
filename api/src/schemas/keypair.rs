use async_graphql::*;
use diesel::{Insertable, Queryable};

use crate::diesel::prelude::*;
use crate::schema::keypairs;
use crate::schemas::SingleConnection;

#[derive(SimpleObject, Queryable, Debug)]
pub struct Keypair {
    /// The id of the keypair
    id: i32,
    /// The private key of the keypair
    private_key: String,
    /// The public key of the keypair
    public_key: String,
}

#[derive(Insertable)]
#[table_name = "keypairs"]
pub struct NewKeypair<'a> {
    pub private_key: &'a str,
    pub public_key: &'a str,
}

pub fn create_keypair<'a>(connection: &SingleConnection, private_key: &'a str, public_key: &'a str) -> Keypair {
    let new_keypair = NewKeypair {
        private_key,
        public_key,
    };

    diesel::insert_into(keypairs::table)
        .values(&new_keypair)
        .get_result(connection)
        .expect("Error saving generated keypair")
}
