use async_graphql::{Context, *};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use std::process::{Command, Stdio};
use std::{io::Write, str};

mod keypair;
use keypair::{create_keypair, Keypair};

use crate::diesel::prelude::*;
use crate::schema::keypairs::dsl::*;

pub type GrahpQLSchema = Schema<QueryRoot, Mutation, EmptySubscription>;
pub type DatabaseConnection = Pool<ConnectionManager<PgConnection>>;
pub type SingleConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_schema(connection: Pool<ConnectionManager<PgConnection>>) -> GrahpQLSchema {
    Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(connection)
        .finish()
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns all the keypairs from the database
    async fn keypairs<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Keypair> {
        let connection = ctx
            .data::<DatabaseConnection>()
            .expect("Could not retrieve connection from context")
            .get()
            .expect("Recieved no connection from pool");

        keypairs.load::<Keypair>(&connection).unwrap()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    /// Generates a keypair
    async fn generate_keypair<'ctx>(&self, ctx: &Context<'ctx>) -> Keypair {
        let connection = ctx
            .data::<DatabaseConnection>()
            .expect("Could not retrieve connection from context")
            .get()
            .expect("Recieved no connection from pool");

        // Generate private key
        let command_privkey = Command::new("wg")
            .arg("genkey")
            .output()
            .expect("Failed to execute command")
            .stdout;
        let priv_key = str::from_utf8(&command_privkey)
            .expect("Could not parse private key")
            .replace("\n", "");

        // Generate public key
        let pubkey_command = Command::new("wg")
            .arg("pubkey")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to generate public key");
        let stdin = pubkey_command
            .stdin
            .as_ref()
            .unwrap()
            .write_all(priv_key.as_bytes());
        if let Err(e) = stdin {
            panic!("Could not read: {}", e);
        }
        // let pubkey_output = pubkey_command.wait_with_output().expect("Did not get a response from wg pubkey");
        // let pub_key = String::from_utf8(pubkey_output.stdout).expect("Could not parse public key");

        create_keypair(&connection, &priv_key, &priv_key)
    }
}
