//! Provides some functions for validating content
use async_graphql::*;
use ipaddress::IPAddress;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::net::Ipv4Addr;
use std::sync::Arc;

use crate::crypto::{Claims, SecretKey};
use crate::database::DatabaseConnection;
use crate::models::{Client, Server};
use crate::Token;

/// Validates if an ip address is in the given network or not
///
/// # Arguments
/// * `ip_network` - The ip network in that the ip addresss should be in
/// * `subnetmask` - The subnetmask of the `ip_network` in CIDR format
/// * `ip_address` - The ip address that should be validated
pub fn is_ip_in_network(ip_network: Ipv4Addr, subnetmask: i32, ip_address: Ipv4Addr) -> bool {
    let ip_address = IPAddress::parse(format!("{}/{}", ip_address, subnetmask)).unwrap();
    ip_address.network().to_s().parse::<Ipv4Addr>().unwrap() == ip_network
}

/// Verifies if the given id of a `Keypair` is already used by another `Client` or `Server`
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `keypair_id` - The id of the `Keypair` that should be checked
pub fn is_keypair_used(connection: &DatabaseConnection, keypair_id: i32) -> bool {
    let mut used_keypairs =
        Client::get_keypair_ids(connection).expect("Error while querying the database");
    used_keypairs
        .extend(Server::get_keypair_ids(connection).expect("Error while querying the database"));

    used_keypairs.contains(&keypair_id)
}

/// Verifies that the given token can be decoded with the secret_key.
///
/// # Arguments
/// * `secret_key` - The key that should be used to decrypt the token
/// * `token` - The token that should be decrypted
///
/// # Returns
/// `true` if the decryption was successful, `false` otherwise
pub fn is_valid_token(secret_key: &Arc<SecretKey>, token: &str) -> bool {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.to_string().as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).is_ok()
}

pub struct UserGuard;

#[async_trait::async_trait]
impl Guard for UserGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let secret_key = ctx
            .data::<Arc<SecretKey>>()
            .expect("Could not find secret key");
        if let Ok(token) = ctx.data::<Token>() {
            if let Some(t) = token.get_token() {
                if is_valid_token(secret_key, &t) {
                    return Ok(());
                }
            }
        }

        Err(Error::new("Forbidden"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ip_in_network() {
        assert!(is_ip_in_network(
            "192.168.0.0".parse().unwrap(),
            24,
            "192.168.0.2".parse().unwrap()
        ));
        assert_eq!(
            is_ip_in_network(
                "192.168.0.0".parse().unwrap(),
                24,
                "192.168.50.2".parse().unwrap()
            ),
            false
        );
    }
}
