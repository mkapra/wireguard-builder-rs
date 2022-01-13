//! Provides some functions for validating content
use std::net::Ipv4Addr;
use ipaddress::IPAddress;

/// Validates if an ip address is in the given network or not
pub fn is_ip_in_network(ip_network: Ipv4Addr, subnetmask: i32, ip_address: Ipv4Addr) -> bool {
    let ip_address = IPAddress::parse(format!("{}/{}", ip_address, subnetmask)).unwrap();
    ip_address.network().to_s().parse::<Ipv4Addr>().unwrap() == ip_network
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
