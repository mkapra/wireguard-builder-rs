//! Module that holds everything that is necessary for the `DnsServer`
use async_graphql::*;

use crate::diesel::prelude::*;
use crate::schema::dns_servers;
use crate::schemas::SingleConnection;

/// A dns server that is used by the client
#[derive(SimpleObject, Queryable, Debug)]
pub struct DnsServer {
    /// The id of the dns server
    id: i32,
    /// A unique name of the dns server
    name: String,
    /// A optional description of the dns server
    description: Option<String>,
    /// The ip address of the dns server
    ip_address: String,
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
    pub ip_address: String
}

/// Creates a new dns server in the database
///
/// # Arguments
/// * `connection` - A connection to the database
/// * `dns_server` - The dns server that should be created
///
/// # Returns
/// Returns the created dns server or returns the error that was thrown by the database.
/// These errors can be:
///
/// * Duplicate `name`
/// * Duplicate `ip_address`
pub fn create_dns_server<'a>(
    connection: &SingleConnection,
    dns_server: &InputDnsServer
) -> Result<DnsServer> {
    let new_dns_server = NewDnsServer {
        name: &dns_server.name,
        description: dns_server.description.as_deref(),
        ip_address: &dns_server.ip_address,
    };

    diesel::insert_into(dns_servers::table)
        .values(&new_dns_server)
        .get_result(connection)
        .map_err(|e| Error::from(e))
}