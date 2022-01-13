//! Module that holds everything that is necessary for the `VpnNetwork`
use async_graphql::*;

use crate::diesel::prelude::*;
use crate::schema::vpn_networks;
use crate::models::SingleConnection;

#[derive(SimpleObject, Queryable, Identifiable, AsChangeset, Debug)]
pub struct VpnNetwork {
    /// The id
    pub id: i32,
    /// A unique name
    pub name: String,
    /// An optional description
    pub description: Option<String>,
    /// The ip_network that the object represents
    pub ip_network: String,
    /// The subnetmask of the ip_network in CIDR format
    pub subnetmask: i32,
    /// The port where the vpn server is listening on
    pub listen_port: i32,
    /// The name of the interface (e.g. wg0)
    pub interface_name: String,
}

#[derive(Insertable)]
#[table_name = "vpn_networks"]
pub struct NewVpnNetwork<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub ip_network: &'a str,
    pub subnetmask: i32,
    pub listen_port: i32,
    pub interface_name: &'a str,
}

#[derive(InputObject)]
pub struct InputVpnNetwork {
    /// A unique name
    pub name: String,
    /// An optional description
    pub description: Option<String>,
    /// The ip_network that the object represents
    #[graphql(validator(ip))]
    pub ip_network: String,
    /// The subnetmask of the ip_network in CIDR format
    #[graphql(default = 24)]
    pub subnetmask: i32,
    /// The port where the vpn server is listening on
    pub listen_port: i32,
    /// The name of the interface (e.g. wg0)
    pub interface_name: String,
}

/// Creates a new vpn network in the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `vpn_network` - The vpn network that should be created
///
/// # Returns
/// Returns the created vpn server or an error that was thrown by the database
pub fn create_vpn_network(
    connection: &SingleConnection,
    vpn_network: &InputVpnNetwork,
) -> Result<VpnNetwork> {
    let new_vpn_network = NewVpnNetwork {
        name: &vpn_network.name,
        description: vpn_network.description.as_deref(),
        ip_network: &vpn_network.ip_network,
        subnetmask: vpn_network.subnetmask,
        listen_port: vpn_network.listen_port,
        interface_name: &vpn_network.interface_name,
    };

    diesel::insert_into(vpn_networks::table)
        .values(&new_vpn_network)
        .get_result(connection)
        .map_err(Error::from)
}

/// Updates a vpn network in the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `net_id` - The id of the vpn network that should be updated
/// * `vpn_network` - The new vpn network object that should replace the old one
///
/// # Returns
/// The update may return an error if the new values violate unique constraints in the database. Otherwise the
/// updated vpn network is returned.
pub fn update_vpn_network(
    connection: &SingleConnection,
    net_id: i32,
    vpn_network: &InputVpnNetwork,
) -> Result<VpnNetwork> {
    if let Some(net) = get_vpn_network_by_id(connection, net_id) {
        let updated_net = VpnNetwork {
            id: net.id,
            name: vpn_network.name.clone(),
            description: vpn_network.description.clone(),
            ip_network: vpn_network.ip_network.clone(),
            subnetmask: vpn_network.subnetmask,
            listen_port: vpn_network.listen_port,
            interface_name: vpn_network.interface_name.clone(),
        };
        return diesel::update(&net)
            .set(&updated_net)
            .get_result(connection)
            .map_err(Error::from);
    }

    Err(Error::new(format!(
        "VPN Network with id {} not found",
        net_id
    )))
}

/// Deletes a vpn network from the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `net_id` - The id of the vpn network that should be deleted
///
/// # Returns
/// Returns true if the delete action was successful an error otherwise
pub fn delete_vpn_network(connection: &SingleConnection, net_id: i32) -> Result<bool> {
    match get_vpn_network_by_id(connection, net_id) {
        Some(net) => match diesel::delete(&net).execute(connection) {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::from(e)),
        },
        None => Err(Error::new(format!(
            "VPN Network with id {} not found",
            net_id
        ))),
    }
}

/// Returns the vpn network for the given id
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `net_id` - The id of the vpn network that should be returned
///
/// # Returns
/// If a vpn network was found a [`Option::Some`] will be returned [`Option::None`] otherwise
pub fn get_vpn_network_by_id(connection: &SingleConnection, net_id: i32) -> Option<VpnNetwork> {
    use crate::schema::vpn_networks::dsl::*;

    vpn_networks
        .filter(id.eq(net_id))
        .load::<VpnNetwork>(connection)
        .expect("Could not query the database")
        .pop()
}
