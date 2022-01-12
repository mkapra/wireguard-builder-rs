//! Module that holds everything that is necessary for the `Server`
use super::*;

use super::keypair::get_keypair_by_id;
use super::vpn_ip_address::{create_new_vpn_ip_address, get_ip_address_by_id, VpnIpAddress};
use super::vpn_network::get_vpn_network_by_id;
use crate::schema::servers;
use crate::validate::is_ip_in_network;

#[derive(Debug, Queryable, Associations, Identifiable)]
#[table_name = "servers"]
#[belongs_to(Keypair)]
#[belongs_to(VpnIpAddress)]
pub struct QueryableServer {
    id: i32,
    name: String,
    description: Option<String>,
    forward_interface: Option<String>,
    external_ip_address: String,
    keypair_id: i32,
    vpn_ip_address_id: i32,
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Server {
    /// The id
    pub id: i32,
    /// A unique name
    pub name: String,
    /// An optional description
    pub description: Option<String>,
    /// The interface where all traffic should be forwarded to
    pub forward_interface: Option<String>,
    /// The ip address or DNS name that is used by the client to connect to the server
    pub external_ip_address: String,
}

impl From<QueryableServer> for Server {
    fn from(server: QueryableServer) -> Self {
        Server {
            id: server.id,
            name: server.name,
            description: server.description,
            forward_interface: server.forward_interface,
            external_ip_address: server.external_ip_address,
        }
    }
}

#[ComplexObject]
impl Server {
    pub async fn keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        use crate::schema::keypairs::dsl::*;
        let connection = create_connection(ctx);
        let server = get_server_by_id(&connection, self.id)?;
        keypairs
            .filter(id.eq(server.keypair_id))
            .first::<Keypair>(&connection)
            .map_err(|e| {
                Error::new(format!(
                    "Error while fetching the keypair for client '{}' ({:?})",
                    server.name, e
                ))
            })
    }

    /// The vpn network that the client is associated with
    async fn vpn_network(&self, ctx: &Context<'_>) -> Result<VpnNetwork> {
        let connection = create_connection(ctx);
        let client = get_server_by_id(&connection, self.id)?;
        let ip_address = get_ip_address_by_id(&connection, client.vpn_ip_address_id);

        get_vpn_network_by_id(&connection, ip_address.vpn_network_id)
            .ok_or(Error::new("Could not find VPN network of client"))
    }
}

/// Input type for a new server
#[derive(InputObject)]
pub struct InputServer {
    /// A unique name
    pub name: String,
    /// An optional description
    pub description: Option<String>,
    /// The interface where all traffic should be forwarded to
    pub forward_interface: Option<String>,
    /// The ip address or DNS name that is used by the client to connect to the server
    pub external_ip_address: String,
    /// The id of the keypair that should be used by the server
    pub keypair_id: i32,
    /// The ip address that the server should have in the vpn network
    #[graphql(validator(ip))]
    pub ip_address: String,
    /// The vpn network which the server should be associated with
    pub vpn_network_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "servers"]
struct InsertableServer {
    pub name: String,
    pub description: Option<String>,
    pub forward_interface: Option<String>,
    pub external_ip_address: String,
    pub keypair_id: i32,
    pub vpn_ip_address_id: i32,
}

/// Creates a new server in the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `server` - The server that should be inserted into the database
///
/// # Returns
/// Returns [`Result::Ok`] if the operation was successful. If validation of the input parameters fails an
/// [`Result::Error`] is returned.
pub fn create_server(
    connection: &SingleConnection,
    server: &InputServer,
) -> Result<QueryableServer> {
    // Check if keypair exists
    if let Err(_) = get_keypair_by_id(connection, server.keypair_id) {
        return Err(Error::new(format!(
            "Keypair with id {} not found for client",
            server.keypair_id
        )));
    }

    // Check if vpn network exists
    match get_vpn_network_by_id(connection, server.vpn_network_id) {
        Some(network) => {
            // Check if ip address is in range of vpn network
            // Unwrap here because the ip addresses are already validated
            if let false = is_ip_in_network(
                network.ip_network.parse().unwrap(),
                network.subnetmask,
                server.ip_address.parse().unwrap(),
            ) {
                return Err(Error::new(format!(
                    "IP address {} is not in range of network {}/{}",
                    server.ip_address, network.ip_network, network.subnetmask
                )));
            }
        }
        None => {
            return Err(Error::new(format!(
                "VPN network with id {} not found for client",
                server.vpn_network_id
            )))
        }
    }

    let vpn_ip_obj =
        create_new_vpn_ip_address(connection, server.vpn_network_id, &server.ip_address).map_err(
            |e| {
                Error::new(format!(
            "Could not create server. Maybe this IP address is already taken? (Error: {:?})",
            e
        ))
            },
        )?;

    let new_server = InsertableServer {
        name: server.name.clone(),
        description: server.description.clone(),
        forward_interface: server.forward_interface.clone(),
        external_ip_address: server.external_ip_address.clone(),
        keypair_id: server.keypair_id,
        vpn_ip_address_id: vpn_ip_obj.id,
    };
    diesel::insert_into(servers::table)
        .values(&new_server)
        .get_result(connection)
        .map_err(Error::from)
}

/// Deletes the server with the given id from the database
pub fn delete_server(connection: &SingleConnection, server_id: i32) -> Result<bool> {
    let server = get_server_by_id(connection, server_id)?;
    diesel::delete(&server)
        .execute(connection)
        .map(|_| true)
        .map_err(Error::from)
}

/// Returns the server for the given id
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `client_id` - The id of the server that should be returned
///
/// # Panics
/// Panics if no server was found
fn get_server_by_id(connection: &SingleConnection, server_id: i32) -> Result<QueryableServer> {
    use crate::schema::servers::dsl::*;
    servers
        .filter(id.eq(server_id))
        .first::<QueryableServer>(connection)
        .map_err(|e| {
            Error::new(format!(
                "Could not find server with id {} ({:?})",
                server_id, e
            ))
        })
}
