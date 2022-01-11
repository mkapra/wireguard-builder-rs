use async_graphql::{Context, *};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

mod keypair;
use keypair::{create_keypair, Keypair};

use crate::diesel::prelude::*;
use crate::schema::keypairs::dsl::*;

pub type GrahpQLSchema = Schema<QueryRoot, Mutation, EmptySubscription>;
pub type DatabaseConnection = Pool<ConnectionManager<PgConnection>>;
pub type SingleConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_schema(connection: Pool<ConnectionManager<PgConnection>>) -> GrahpQLSchema {
    Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(connection)
        .finish()
}

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
        create_keypair(&connection, &priv_key, &pub_key)
    }
}
