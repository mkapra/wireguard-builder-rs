//! The GraphQL schema
use async_graphql::{Context, *};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use crate::diesel::prelude::*;

mod keypair;
use keypair::Keypair;
mod dns_server;
use dns_server::{DnsServer, InputDnsServer};
mod vpn_network;
use vpn_network::{InputVpnNetwork, VpnNetwork};
mod client;
use client::{Client, InputClient, QueryableClient};
mod server;
mod vpn_ip_address;
use server::{InputServer, QueryableServer, Server};

/// Represents the schema that is created by [`create_schema()`]
pub type GrahpQLSchema = Schema<QueryRoot, Mutation, EmptySubscription>;
/// Represents a pool of connections to the database
pub type DatabaseConnection = Pool<ConnectionManager<PgConnection>>;
/// Represents a single connection to the database
pub type SingleConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Creates a new schema with a connection pool for communicating with the database as context
///
/// # Arguments
/// * `connection` - A pool for PostgreSQL connections
///
/// # Returns
/// Returns a schema that can be used by the web framework
pub fn create_schema(connection: DatabaseConnection) -> GrahpQLSchema {
    Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(connection)
        .finish()
}

/// The root of the Query type
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Returns all the keypairs from the database
    async fn keypairs(&self, ctx: &Context<'_>) -> Vec<Keypair> {
        use crate::schema::keypairs::dsl::*;
        keypairs.load::<Keypair>(&create_connection(ctx)).unwrap()
    }

    /// Returns all unused keypairs
    async fn unused_keypairs(&self, ctx: &Context<'_>) -> Vec<Keypair> {
        use crate::schema::keypairs::dsl::*;

        let connection = create_connection(ctx);
        let mut used_keypairs =
            Client::get_keypair_ids(&connection).expect("Failed to query keypairs");
        let keypair_ids_server =
            Server::get_keypair_ids(&connection).expect("Failed to query keypairs");
        used_keypairs.extend(keypair_ids_server);

        keypairs
            .filter(id.ne_all(&used_keypairs))
            .load::<Keypair>(&connection)
            .expect("Could not query database")
    }

    /// Returns all the dns servers from the database
    async fn dns_servers(&self, ctx: &Context<'_>) -> Vec<DnsServer> {
        use crate::schema::dns_servers::dsl::*;
        dns_servers
            .load::<DnsServer>(&create_connection(ctx))
            .unwrap()
    }

    /// Returns all the vpn networks from the database
    async fn vpn_networks<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<VpnNetwork> {
        use crate::schema::vpn_networks::dsl::*;
        vpn_networks
            .load::<VpnNetwork>(&create_connection(ctx))
            .unwrap()
    }

    /// Returns all the clients from the database
    async fn clients(&self, ctx: &Context<'_>) -> Vec<Client> {
        use crate::schema::clients::dsl::*;
        clients
            .load::<QueryableClient>(&create_connection(ctx))
            .unwrap()
            .into_iter()
            .map(Client::from)
            .collect()
    }

    /// Returns all the servers from the database
    async fn servers(&self, ctx: &Context<'_>) -> Vec<Server> {
        use crate::schema::servers::dsl::*;
        servers
            .load::<QueryableServer>(&create_connection(ctx))
            .unwrap()
            .into_iter()
            .map(Server::from)
            .collect()
    }
}

/// The root of the mutation type
pub struct Mutation;

#[Object]
impl Mutation {
    /// Generates a keypair
    async fn generate_keypair(&self, ctx: &Context<'_>) -> Keypair {
        let (priv_key, pub_key) = Keypair::generate_keypair();
        Keypair::create(&create_connection(ctx), &pub_key, &priv_key)
    }

    /// Creates a new dns server
    async fn create_dns_server(
        &self,
        ctx: &Context<'_>,
        dns_server: InputDnsServer,
    ) -> Result<DnsServer> {
        DnsServer::create(&create_connection(ctx), &dns_server)
    }

    /// Updates an existing dns server
    async fn update_dns_server(
        &self,
        ctx: &Context<'_>,
        server_id: i32,
        dns_server: InputDnsServer,
    ) -> Result<DnsServer> {
        DnsServer::update(&create_connection(ctx), server_id, &dns_server)
    }

    /// Deletes a dns server
    async fn delete_dns_server(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The id of the server that should be deleted")] server_id: i32,
    ) -> Result<bool> {
        DnsServer::delete(&create_connection(ctx), server_id).map(|_| true)
    }

    /// Creates a vpn network
    async fn create_vpn_network(
        &self,
        ctx: &Context<'_>,
        vpn_network: InputVpnNetwork,
    ) -> Result<VpnNetwork> {
        VpnNetwork::create(&create_connection(ctx), &vpn_network)
    }

    /// Updates an existing vpn network
    async fn update_vpn_network(
        &self,
        ctx: &Context<'_>,
        net_id: i32,
        vpn_network: InputVpnNetwork,
    ) -> Result<VpnNetwork> {
        VpnNetwork::update(&create_connection(ctx), net_id, &vpn_network)
    }

    /// Deletes a vpn network
    async fn delete_vpn_network(&self, ctx: &Context<'_>, network_id: i32) -> Result<bool> {
        VpnNetwork::delete(&create_connection(ctx), network_id)
    }

    /// Creates a client
    async fn create_client(&self, ctx: &Context<'_>, client: InputClient) -> Result<Client> {
        Client::create(&create_connection(ctx), &client).map(Client::from)
    }

    /// Deletes a client
    async fn delete_client(&self, ctx: &Context<'_>, client_id: i32) -> Result<bool> {
        Client::delete(&create_connection(ctx), client_id)
    }

    /// Creates a server
    async fn create_server(&self, ctx: &Context<'_>, server: InputServer) -> Result<Server> {
        Server::create(&create_connection(ctx), &server).map(Server::from)
    }

    /// Deletes a server
    async fn delete_server(&self, ctx: &Context<'_>, server_id: i32) -> Result<bool> {
        Server::delete(&create_connection(ctx), server_id)
    }
}

/// Retrieves a single database connection from the database connection pool and returns it
///
/// # Arguments
/// * `ctx` - The context of the graphql request that includes the database connection pool
fn create_connection(ctx: &Context<'_>) -> SingleConnection {
    ctx.data::<DatabaseConnection>()
        .expect("Could not retrieve connection from context")
        .get()
        .expect("Recieved no connection from pool")
}
