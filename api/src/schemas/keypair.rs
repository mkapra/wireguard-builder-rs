//! Module that holds everything that is necessary for the `Keypairs`
use async_graphql::*;
use diesel::{Insertable, Queryable};
use std::process::{Command, Stdio};
use std::{io::Write, str};

use crate::diesel::prelude::*;
use crate::schema::keypairs;
use crate::schemas::SingleConnection;

/// A keypair that is used by a client or server
#[derive(SimpleObject, Queryable, Debug)]
pub struct Keypair {
    /// The id of the keypair
    pub id: i32,
    /// The public key of the keypair
    pub public_key: String,
    /// The private key of the keypair
    pub private_key: String,
}

#[derive(Insertable)]
#[table_name = "keypairs"]
pub struct NewKeypair<'a> {
    pub public_key: &'a str,
    pub private_key: &'a str,
}

// Creates a new keypair in the database
//
// # Arguments
// * `connection` - A connection to the database
// * `public_key` - The public key
// * `private_key` - The private key
//
// # Returns
// Returns the created keypair
//
// # Panics
// Panics if an error occured while saving to the database.
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

/// Returns the keypair for the given id
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `net_id` - The id of the keypair that should be returned
///
/// # Panics
/// If the keypair was not found in the database
pub fn get_keypair_by_id(connection: &SingleConnection, keypair_id: i32) -> Result<Keypair> {
    use crate::schema::keypairs::dsl::*;

    keypairs
        .filter(id.eq(keypair_id))
        .first(connection)
        .map_err(|e| {
            Error::new(format!(
                "Could not query keypair with id {} ({})",
                keypair_id, e
            ))
        })
}
