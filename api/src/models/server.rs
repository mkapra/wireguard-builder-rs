//! Module that holds everything that is necessary for the `Server`
use handlebars::Handlebars;

use super::*;
use super::vpn_ip_address::VpnIpAddress;
use crate::schema::servers;
use crate::validate::is_ip_in_network;

const SERVER_CONFIG: &str = r#"# Server configuration
[Interface]
Address = {{server_address}}
ListenPort = {{listen_port}}
PrivateKey = {{server_private_key}}

# Clients
{{#each clients}}
## {{name}}
[Peer]
PublicKey = {{public_key}}
AllowedIps = {{ip_address}}/32
{{/each}}
"#;

/// Input type for a new Server
#[derive(InputObject)]
pub struct InputServer {
    pub name: String,
    pub description: Option<String>,
    /// The interface where all traffic should be forwarded to
    pub forward_interface: Option<String>,
    /// The ip address or FQDN that is used by the client to connect to the server
    pub external_ip_address: String,
    /// The id of the keypair that should be used by the server
    pub keypair_id: i32,
    /// The ip address that the server should have in the vpn network
    #[graphql(validator(ip))]
    pub ip_address: String,
    /// The id of the vpn network which the server should be associated with
    pub vpn_network_id: i32,
}

/// A `Server` that is insertable into the database
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

/// Represents the configuration for a server
#[derive(serde::Serialize)]
struct ServerConfig {
    server_address: String,
    listen_port: i32,
    server_private_key: String,
    clients: Vec<ClientServerConfig>,
}

/// Represents a client that is used for the configuration of the server
#[derive(serde::Serialize)]
pub struct ClientServerConfig {
    pub name: String,
    pub public_key: String,
    pub ip_address: String,
}

/// A server is part of the `VpnNetwork` and the endpoint for the `Client`s
#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    /// The interface where all traffic should be forwarded to
    pub forward_interface: Option<String>,
    /// The ip address or FQDN that is used by the client to connect to the server
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
    /// A wireguard configuration for the Server
    pub async fn config(&self, ctx: &Context<'_>) -> Option<String> {
        let connection = create_connection(ctx);
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(|c| c.to_string());
        handlebars.set_strict_mode(true);
        handlebars
            .register_template_string("t1", SERVER_CONFIG)
            .unwrap();

        let ip_address = self
            .ip_address(ctx)
            .await
            .expect("Server does not have an ip address");
        let vpn_network = self
            .vpn_network(ctx)
            .await
            .expect("Server is not part of vpn network");
        let keypair = self
            .keypair(ctx)
            .await
            .expect("Server does not hava a keypair");
        let clients = vpn_network.get_associated_clients(&connection);
        if let None = clients {
            return None;
        }
        let data = ServerConfig {
            server_address: format!("{}/{}", ip_address, vpn_network.subnetmask),
            listen_port: vpn_network.listen_port,
            server_private_key: keypair.private_key,
            clients: clients.unwrap(),
        };

        Some(
            handlebars
                .render("t1", &data)
                .expect("Error while generating the client configuration"),
        )
    }

    pub async fn keypair(&self, ctx: &Context<'_>) -> Result<Keypair> {
        use crate::schema::keypairs::dsl::*;
        let connection = create_connection(ctx);
        let server = Self::get_by_id(&connection, self.id)?;
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
        let client = Self::get_by_id(&connection, self.id)?;
        let ip_address = VpnIpAddress::get_by_id(&connection, client.vpn_ip_address_id);

        VpnNetwork::get_by_id(&connection, ip_address.vpn_network_id)
            .ok_or(Error::new("Could not find VPN network of client"))
    }

    /// The ip address of the server in the vpn network
    async fn ip_address(&self, ctx: &Context<'_>) -> Result<String> {
        let connection = create_connection(ctx);
        let server = Self::get_by_id(&connection, self.id)?;
        Ok(VpnIpAddress::get_by_id(&connection, server.vpn_ip_address_id).ip_address)
    }
}

impl Server {
    /// Creates a new [`Server`] in the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `server` - The [`Server`] that should be inserted into the database
    ///
    /// # Returns
    /// Returns the [`Server`] if the operation was successful. If validation of the input parameters fails an
    /// error is returned.
    pub fn create(connection: &SingleConnection, server: &InputServer) -> Result<QueryableServer> {
        // Check if keypair exists
        if let Err(_) = Keypair::get_by_id(connection, server.keypair_id) {
            return Err(Error::new(format!(
                "Keypair with id {} not found for client",
                server.keypair_id
            )));
        }

        // Check if vpn network exists
        match VpnNetwork::get_by_id(connection, server.vpn_network_id) {
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
            VpnIpAddress::create(connection, server.vpn_network_id, &server.ip_address).map_err(
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

    /// Deletes the [`Server`] with the given id from the database
    pub fn delete(connection: &SingleConnection, server_id: i32) -> Result<bool> {
        let server = Self::get_by_id(connection, server_id)?;
        diesel::delete(&server)
            .execute(connection)
            .map(|_| true)
            .map_err(Error::from)
    }

    /// Returns the [`Server`] for the given id
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `client_id` - The id of the [`Server`] that should be returned
    ///
    /// # Panics
    /// Panics if no `Server` was found
    fn get_by_id(connection: &SingleConnection, server_id: i32) -> Result<QueryableServer> {
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
}
