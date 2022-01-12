//! Module that holds everything that is necessary for the `Client`
use super::*;

use super::dns_server::get_dns_server_by_id;
use super::keypair::get_keypair_by_id;
use super::vpn_ip_address::{create_new_vpn_ip_address, get_ip_address_by_id};
use super::vpn_network::get_vpn_network_by_id;
use crate::schema::clients;
use crate::validate::is_ip_in_network;

#[derive(Debug, Queryable, Associations, Identifiable)]
#[table_name = "clients"]
#[belongs_to(DnsServer)]
#[belongs_to(Keypair)]
pub struct QueryableClient {
    id: i32,
    name: String,
    description: Option<String>,
    keepalive_interval: i32,
    dns_server_id: i32,
    keypair_id: i32,
    vpn_ip_address_id: i32,
}

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Client {
    /// The id
    id: i32,
    /// A unique name
    name: String,
    /// An optional description
    description: Option<String>,
    /// The interval in seconds where the client should reconnect to the server
    keepalive_interval: i32,
}

impl From<QueryableClient> for Client {
    fn from(client: QueryableClient) -> Self {
        Client {
            id: client.id,
            name: client.name,
            description: client.description,
            keepalive_interval: client.keepalive_interval,
        }
    }
}

#[ComplexObject]
impl Client {
    // TODO: The custom resolvers should use a `DataLoader` to reduce the amount of queries to the database
    /// The keypair that is used by the client
    async fn keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        use crate::schema::keypairs::dsl::*;
        let connection = create_connection(ctx);
        let client = get_client_by_id(&connection, self.id)?;
        keypairs
            .filter(id.eq(client.keypair_id))
            .first::<Keypair>(&connection)
            .map_err(|e| {
                Error::new(format!(
                    "Error while fetching the keypair for client '{}' ({:?})",
                    client.name, e
                ))
            })
    }

    /// The dns server that is used by the client
    async fn dns_server(&self, ctx: &Context<'_>) -> Result<DnsServer> {
        use crate::schema::dns_servers::dsl::*;
        let connection = create_connection(ctx);
        let client = get_client_by_id(&connection, self.id)?;
        dns_servers
            .filter(id.eq(client.dns_server_id))
            .first::<DnsServer>(&connection)
            .map_err(|e| {
                Error::new(format!(
                    "Error while fetching the DNS server for client '{}' ({:?})",
                    client.name, e
                ))
            })
    }

    /// The vpn network that the client is associated with
    async fn vpn_network(&self, ctx: &Context<'_>) -> Result<VpnNetwork> {
        let connection = create_connection(ctx);
        let client = get_client_by_id(&connection, self.id)?;
        let ip_address = get_ip_address_by_id(&connection, client.vpn_ip_address_id);

        get_vpn_network_by_id(&connection, ip_address.vpn_network_id)
            .ok_or(Error::new("Could not find VPN network of client"))
    }
}

/// Input type for a new client
#[derive(InputObject)]
pub struct InputClient {
    /// The name of the client
    pub name: String,
    /// An optional description for the client
    pub description: Option<String>,
    /// The ip address of the client in the vpn network
    #[graphql(validator(ip))]
    pub ip_address: String,
    /// The vpn network of that the client should be a part of
    pub vpn_network_id: i32,
    /// The interval in seconds where the client should reconnect to the server
    #[graphql(default = 25)]
    pub keepalive_interval: i32,
    /// The id of the dns server that should be used by the client
    pub dns_server_id: i32,
    /// The id of the keypair that should be used by the client
    pub keypair_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "clients"]
struct InsertableClient {
    pub name: String,
    pub description: Option<String>,
    pub keepalive_interval: i32,
    pub dns_server_id: i32,
    pub keypair_id: i32,
    pub vpn_ip_address_id: i32,
}

/// Creates a new client in the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `client` - The client that should be inserted into the database
///
/// # Returns
/// Returns [`Result::Ok`] if the operation was a success. If validation of the input parameters fails an
/// [`Result::Error`] is returned.
pub fn create_client(
    connection: &SingleConnection,
    client: &InputClient,
) -> Result<QueryableClient> {
    // Check if dns server exists
    if let None = get_dns_server_by_id(connection, client.dns_server_id) {
        return Err(Error::new(format!(
            "DNS Server with id {} not found for client",
            client.dns_server_id
        )));
    }

    // Check if keypair exists
    if let Err(_) = get_keypair_by_id(connection, client.keypair_id) {
        return Err(Error::new(format!(
            "Keypair with id {} not found for client",
            client.keypair_id
        )));
    }

    // Check if vpn network exists
    match get_vpn_network_by_id(connection, client.vpn_network_id) {
        Some(network) => {
            // Check if ip address is in range of vpn network
            // Unwrap here because the ip addresses are already validated
            if let false = is_ip_in_network(
                network.ip_network.parse().unwrap(),
                network.subnetmask,
                client.ip_address.parse().unwrap(),
            ) {
                return Err(Error::new(format!(
                    "IP address {} is not in range of network {}/{}",
                    client.ip_address, network.ip_network, network.subnetmask
                )));
            }
        }
        None => {
            return Err(Error::new(format!(
                "VPN network with id {} not found for client",
                client.vpn_network_id
            )))
        }
    }

    let vpn_ip_obj =
        create_new_vpn_ip_address(connection, client.vpn_network_id, &client.ip_address).map_err(
            |e| {
                Error::new(format!(
            "Could not create client. Maybe this IP address is already taken? (Error: {:?})",
            e
        ))
            },
        )?;

    let new_client = InsertableClient {
        name: client.name.clone(),
        description: client.description.clone(),
        keepalive_interval: client.keepalive_interval,
        dns_server_id: client.dns_server_id,
        keypair_id: client.keypair_id,
        vpn_ip_address_id: vpn_ip_obj.id,
    };

    diesel::insert_into(clients::table)
        .values(&new_client)
        .get_result(connection)
        .map_err(Error::from)
}

pub fn delete_client(connection: &SingleConnection, client_id: i32) -> Result<bool> {
    let client = get_client_by_id(connection, client_id)?;
    diesel::delete(&client)
        .execute(connection)
        .map(|_| true)
        .map_err(Error::from)
}

/// Returns the client for the given id
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `client_id` - The id of the cclient that should be returned
///
/// # Panics
/// Panics if no client was found
fn get_client_by_id(connection: &SingleConnection, client_id: i32) -> Result<QueryableClient> {
    use crate::schema::clients::dsl::*;
    clients
        .filter(id.eq(client_id))
        .first::<QueryableClient>(connection)
        .map_err(|e| {
            Error::new(format!(
                "Could not find client with id {} ({:?})",
                client_id, e
            ))
        })
}
