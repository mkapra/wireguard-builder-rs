//! Module that holds everything that is necessary for the `Client`
use std::collections::BTreeMap;

use handlebars::Handlebars;

use super::dns_server::get_dns_server_by_id;
use super::keypair::get_keypair_by_id;
use super::vpn_ip_address::{create_new_vpn_ip_address, get_ip_address_by_id};
use super::vpn_network::get_vpn_network_by_id;
use super::*;
use crate::schema::clients;
use crate::schema::vpn_ip_addresses;
use crate::schemas::vpn_ip_address::VpnIpAddress;
use crate::validate::is_ip_in_network;

const CLIENT_CONFIG: &str = r#"[Interface]
PrivateKey = {{clientPrivateKey}}
Address = {{clientIp}}
DNS = {{dnsServerIp}}

[Peer]
PublicKey = {{serverPublicKey}}
AllowedIPs = 0.0.0.0/0
Endpoint = {{endpoint}}
PersistentKeepalive = {{keepalive}}"#;

#[derive(Debug, Queryable, Associations, Identifiable)]
#[table_name = "clients"]
#[belongs_to(DnsServer)]
#[belongs_to(Keypair)]
pub struct QueryableClient {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub keepalive_interval: i32,
    pub dns_server_id: i32,
    pub keypair_id: i32,
    pub vpn_ip_address_id: i32,
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
    async fn config(&self, ctx: &Context<'_>) -> Option<String> {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(|c| c.to_string());
        handlebars
            .register_template_string("t1", CLIENT_CONFIG)
            .unwrap();
        let mut data = BTreeMap::new();

        let server = self
            .server(ctx)
            .await
            .expect("Error while fetching server data");
        let vpn_network = self.vpn_network(ctx).await.ok()?;
        // Query server that should be used by the client
        match server {
            Some(server) => {
                // Get keypair of server
                let server_keypair = server
                    .keypair(ctx)
                    .await
                    .expect("Error while getting keypair of server");
                data.insert("serverPublicKey", server_keypair.public_key);

                // Endpoint
                data.insert(
                    "endpoint",
                    format!("{}:{}", server.external_ip_address, vpn_network.listen_port),
                );
            }
            None => return None,
        }
        // Get keypair of client
        let keypair = self.keypair(ctx).await.ok()?;
        data.insert("clientPrivateKey", keypair.private_key);
        // Get the dns server
        let dns_server = self.dns_server(ctx).await.ok()?;
        data.insert("dnsServerIp", dns_server.ip_address);
        // Get the clients ip address
        let vpn_ip = self.ip_address(ctx).await.ok()?;
        data.insert("clientIp", format!("{}/{}", vpn_ip, vpn_network.subnetmask));
        // Keepalive Interval
        data.insert("keepalive", self.keepalive_interval.to_string());

        Some(
            handlebars
                .render("t1", &data)
                .expect("Error while generating the client configuration"),
        )
    }

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

    /// The ip address of the client in the vpn network
    async fn ip_address(&self, ctx: &Context<'_>) -> Result<String> {
        let connection = create_connection(ctx);
        let client = get_client_by_id(&connection, self.id)?;
        Ok(get_ip_address_by_id(&connection, client.vpn_ip_address_id).ip_address)
    }

    async fn server(&self, ctx: &Context<'_>) -> Option<Server> {
        use crate::schema::servers::dsl::*;
        let connection = create_connection(ctx);
        let vpn_network = self
            .vpn_network(ctx)
            .await
            .ok()
            .expect("Did not find vpn network of client");
        let result: Result<(QueryableServer, VpnIpAddress), _> = servers
            .filter(vpn_ip_addresses::vpn_network_id.eq(vpn_network.id))
            .inner_join(vpn_ip_addresses::table)
            .first(&connection);

        result.ok().map(|(server, _)| Server::from(server))
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
