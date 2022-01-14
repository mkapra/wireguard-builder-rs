//! Module that holds everything that is necessary for the `DnsServer`
use async_graphql::*;

use crate::database::DatabaseConnection;
use crate::diesel::prelude::*;
use crate::schema::dns_servers;

/// A [`DnsServer`] that is insertable into the database
#[derive(Insertable)]
#[table_name = "dns_servers"]
pub struct NewDnsServer<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub ip_address: &'a str,
}

/// Input type for creating a new DnsServer
#[derive(InputObject)]
pub struct InputDnsServer {
    pub name: String,
    pub description: Option<String>,
    #[graphql(validator(ip))]
    pub ip_address: String,
}

/// A DnsServer that is used by the Client
#[derive(Debug, SimpleObject, Queryable, Identifiable, AsChangeset)]
pub struct DnsServer {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub ip_address: String,
}

impl DnsServer {
    /// Creates a new [`DnsServer`] in the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `dns_server` - The [`DnsServer`] that should be created
    ///
    /// # Returns
    /// Returns the created [`DnsServer`] or returns the error that was thrown by the database
    ///
    /// These errors can be:
    ///
    /// * Duplicate `name`
    /// * Duplicate `ip_address`
    pub fn create(
        connection: &DatabaseConnection,
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

    /// Updates a [`DnsServer`] in the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `server_id` - The id of the [`DnsServer`] that should be updated
    /// * `dns_server` - The attributes of the updated [`DnsServer`] that will be applied
    ///
    /// # Returns
    /// The update may return an error if the new values violate uniqe constraints in the database. Otherwise the
    /// updated [`DnsServer`] is returned.
    pub fn update(
        connection: &DatabaseConnection,
        server_id: i32,
        dns_server: &InputDnsServer,
    ) -> Result<DnsServer> {
        if let Some(server) = Self::get_by_id(connection, server_id) {
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

    /// Deletes the [`DnsServer`] with the given id from the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `server_id` - The id of the [`DnsServer`] that should be deleted
    ///
    /// # Returns
    /// An empty result if the element was deleted or an error if the [`DnsServer`] was not found or could not be
    /// deleted
    pub fn delete(connection: &DatabaseConnection, server_id: i32) -> Result<()> {
        match Self::get_by_id(connection, server_id) {
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

    /// Retrieves the [`DnsServer`] with the given id from the database
    ///
    /// # Arguments
    /// * `connection` - A connection to the database
    /// * `server_id` - The id of the [`DnsServer`] that should be returned
    ///
    /// # Returns
    /// The [`DnsServer`] if found or [`Option::None`]
    pub fn get_by_id(connection: &DatabaseConnection, server_id: i32) -> Option<DnsServer> {
        use crate::schema::dns_servers::dsl::*;

        let mut servers = dns_servers
            .filter(id.eq(server_id))
            .load::<DnsServer>(connection)
            .expect("Could not query the database");
        servers.pop()
    }
}
