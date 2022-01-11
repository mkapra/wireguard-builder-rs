//! The GraphQL schema
use async_graphql::{Context, *};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

mod keypair;
use keypair::{create_keypair, Keypair};

use crate::diesel::prelude::*;
use crate::schema::keypairs::dsl::*;

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
    async fn keypairs<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Keypair> {
        let connection = ctx
            .data::<DatabaseConnection>()
            .expect("Could not retrieve connection from context")
            .get()
            .expect("Recieved no connection from pool");

        keypairs.load::<Keypair>(&connection).unwrap()
    }
}

/// The root of the mutation type
pub struct Mutation;

#[Object]
impl Mutation {
    /// Generates a keypair
    async fn generate_keypair<'ctx>(&self, ctx: &Context<'ctx>) -> Keypair {
        let connection = ctx
            .data::<DatabaseConnection>()
            .expect("Could not retrieve connection from context")
            .get()
            .expect("Recieved no connection from pool");

        let (priv_key, pub_key) = Keypair::generate_keypair();
        create_keypair(&connection, &pub_key, &priv_key)
    }
}
