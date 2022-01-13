//! Module that holds everything that is necessary for the `VpnIpAddress`
use super::*;
use crate::schema::vpn_ip_addresses;

/// A [`VpnIpAddress`] that is insertable into the database
#[derive(Debug, Insertable)]
#[table_name = "vpn_ip_addresses"]
pub struct NewVpnIpAddress<'a> {
    pub vpn_network_id: i32,
    pub ip_address: &'a str,
}

/// A `VpnIpAddress` represents a unique ip address that is associated with a `VpnNetwork`
#[derive(Debug, Queryable)]
pub struct VpnIpAddress {
    pub id: i32,
    pub vpn_network_id: i32,
    pub ip_address: String,
}

impl VpnIpAddress {
    /// Creates a new [`VpnIpAddress`]
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `vpn_network_id` - The id of the `VpnNetwork` with that the ip address should be associated with
    /// * `ip_address` - The ip address that should be created
    ///
    /// # Returns
    /// Returns the created [`VpnIpAddress`] or an error from the database
    pub fn create<'a>(
        connection: &SingleConnection,
        vpn_network_id: i32,
        ip_address: &'a str,
    ) -> Result<VpnIpAddress> {
        let new_ip = NewVpnIpAddress {
            vpn_network_id,
            ip_address,
        };

        diesel::insert_into(vpn_ip_addresses::table)
            .values(&new_ip)
            .get_result(connection)
            .map_err(Error::from)
    }

    /// Returns the [`VpnIpAddress`] for the given id
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `ip_id` - The id of the [`VpnIpAddress`] that should be returned
    ///
    /// # Panics
    /// Panics if no ip address was found
    pub fn get_by_id(connection: &SingleConnection, ip_id: i32) -> VpnIpAddress {
        use crate::schema::vpn_ip_addresses::dsl::*;
        vpn_ip_addresses
            .filter(id.eq(ip_id))
            .first::<VpnIpAddress>(connection)
            .expect("Could not find ip address with given id")
    }
}
