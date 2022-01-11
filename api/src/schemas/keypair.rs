use async_graphql::*;
use diesel::{Insertable, Queryable};
use std::process::{Command, Stdio};
use std::{io::Write, str};

use crate::diesel::prelude::*;
use crate::schema::keypairs;
use crate::schemas::SingleConnection;

#[derive(SimpleObject, Queryable, Debug)]
pub struct Keypair {
    /// The id of the keypair
    id: i32,
    /// The public key of the keypair
    public_key: String,
    /// The private key of the keypair
    private_key: String,
}

#[derive(Insertable)]
#[table_name = "keypairs"]
pub struct NewKeypair<'a> {
    pub public_key: &'a str,
    pub private_key: &'a str,
}

pub fn create_keypair<'a>(
    connection: &SingleConnection,
    public_key: &'a str,
    private_key: &'a str,
) -> Keypair {
    let new_keypair = NewKeypair {
        public_key,
        private_key,
    };

    diesel::insert_into(keypairs::table)
        .values(&new_keypair)
        .get_result(connection)
        .expect("Error saving generated keypair")
}

impl Keypair {
    /// Generates a new keypair and returns it
    pub fn generate_keypair() -> (String, String) {
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
            .stdout(Stdio::piped())
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

        let pubkey_output = pubkey_command
            .wait_with_output()
            .expect("Did not get a response from wg pubkey");
        let pub_key = str::from_utf8(&pubkey_output.stdout)
            .expect("Could not parse public key")
            .replace("\n", "");

        (priv_key, pub_key)
    }
}
