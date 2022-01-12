//! Module that holds everything that is necessary for the `VpnIpAddress`
use super::*;

use crate::schema::vpn_ip_addresses;

#[derive(Debug, Queryable)]
pub struct VpnIpAddress {
    pub id: i32,
    pub vpn_network_id: i32,
    pub ip_address: String,
}

#[derive(Debug, Insertable)]
#[table_name = "vpn_ip_addresses"]
pub struct NewVpnIpAddress<'a> {
    pub vpn_network_id: i32,
    pub ip_address: &'a str,
}

pub fn create_new_vpn_ip_address<'a>(connection: &SingleConnection, vpn_network_id: i32, ip_address: &'a str) -> Result<VpnIpAddress> {
    let new_ip = NewVpnIpAddress {
        vpn_network_id,
        ip_address,
    };

    diesel::insert_into(vpn_ip_addresses::table)
        .values(&new_ip)
        .get_result(connection)
        .map_err(Error::from)
}

/// Returns the vpn ip address for the given id
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `ip_id` - The id of the ip address that should be returned
///
/// # Panics
/// Panics if no ip address was found
pub fn get_ip_address_by_id(connection: &SingleConnection, ip_id: i32) -> VpnIpAddress {
    use crate::schema::vpn_ip_addresses::dsl::*;
    vpn_ip_addresses
        .filter(id.eq(ip_id))
        .first::<VpnIpAddress>(connection)
        .expect("Could not find ip address with given id")
}