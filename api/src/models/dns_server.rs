//! Module that holds everything that is necessary for the `DnsServer`
use async_graphql::*;

use crate::diesel::prelude::*;
use crate::schema::dns_servers;
use crate::models::SingleConnection;

/// A dns server that is used by the client
#[derive(SimpleObject, Queryable, Identifiable, AsChangeset, Debug)]
pub struct DnsServer {
    /// The id of the dns server
    pub id: i32,
    /// A unique name of the dns server
    pub name: String,
    /// A optional description of the dns server
    pub description: Option<String>,
    /// The ip address of the dns server
    pub ip_address: String,
}

#[derive(Insertable)]
#[table_name = "dns_servers"]
pub struct NewDnsServer<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub ip_address: &'a str,
}

/// Input type for creating a new dns server
#[derive(InputObject)]
pub struct InputDnsServer {
    /// The unique name of the dns server
    pub name: String,
    /// A optional description of the dns server
    pub description: Option<String>,
    /// The ip address of the dns server
    #[graphql(validator(ip))]
    pub ip_address: String,
}

/// Creates a new dns server in the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `dns_server` - The dns server that should be created
///
/// # Returns
/// Returns the created dns server or returns the error that was thrown by the database
///
/// These errors can be:
///
/// * Duplicate `name`
/// * Duplicate `ip_address`
pub fn create_dns_server(
    connection: &SingleConnection,
    dns_server: &InputDnsServer,
) -> Result<DnsServer> {
    let new_dns_server = NewDnsServer {
        name: &dns_server.name,
        description: dns_server.description.as_deref(),
        ip_address: &dns_server.ip_address,
    };

    diesel::insert_into(dns_servers::table)
        .values(&new_dns_server)
        .get_result(connection)
        .map_err(Error::from)
}

/// Updates a dns server in the database
///
/// # Arguments
/// * `connection` - The connection to the database
/// * `server_id` - The id of the server that should be updated
/// * `dns_server` - The attributes of the updated dns server that will be applied to the dns server with the id `server_id`
///
/// # Returns
/// The update may return an error if the new values violate uniqe constraints in the database. Otherwise the updated
/// dns server is returned.
pub fn update_dns_server(
    connection: &SingleConnection,
    server_id: i32,
    dns_server: &InputDnsServer,
) -> Result<DnsServer> {
    if let Some(server) = get_dns_server_by_id(connection, server_id) {
        let updated_server = DnsServer {
            id: server.id,
            name: dns_server.name.clone(),
            description: dns_server.description.clone(),
            ip_address: dns_server.ip_address.clone(),
        };

        return diesel::update(&server)
            .set(&updated_server)
            .get_result(connection)
            .map_err(Error::from);
    }

    return Err(Error::new(format!(
        "DNS Server with id {} not found",
        server_id
    )));
}

/// Delete the dns server with the given id from the database
///
/// # Arguments
/// * `connection` - The connection to the database
/// * `server_id` - The id of the server that should be deleted
///
/// # Returns
/// An empty result if the element was deleted or an error if the server was not found or could not be
/// deleted
pub fn delete_dns_server(connection: &SingleConnection, server_id: i32) -> Result<()> {
    match get_dns_server_by_id(connection, server_id) {
        Some(server) => {
            if let Err(e) = diesel::delete(&server).execute(connection) {
                Err(Error::from(e))
            } else {
                Ok(())
            }
        }
        None => Err(Error::new(format!(
            "DNS Server with id {} not found",
            server_id
        ))),
    }
}

/// Retrieves the dns server with the given id from the database
///
/// # Arguments
/// * `connection` - The connection to the database
/// * `server_id` - The id of the server that should be returned
///
/// # Returns
/// The server if found or [`Option::None`]
pub fn get_dns_server_by_id(connection: &SingleConnection, server_id: i32) -> Option<DnsServer> {
    use crate::schema::dns_servers::dsl::*;

    let mut servers = dns_servers
        .filter(id.eq(server_id))
        .load::<DnsServer>(connection)
        .expect("Could not query the database");
    servers.pop()
}
