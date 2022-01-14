//! Provides some functions for validating content
use ipaddress::IPAddress;
use std::net::Ipv4Addr;

use crate::models::{Client, Server, SingleConnection};

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
pub fn is_keypair_used(connection: &SingleConnection, keypair_id: i32) -> bool {
    let mut used_keypairs =
        Client::get_keypair_ids(connection).expect("Error while querying the database");
    used_keypairs
        .extend(Server::get_keypair_ids(connection).expect("Error while querying the database"));

    used_keypairs.contains(&keypair_id)
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
