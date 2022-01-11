use dotenv::dotenv;
use std::env;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};
use rocket::{response::content, routes, State};

mod schemas;
use schemas::{GrahpQLSchema, create_schema};

#[macro_use]
extern crate diesel;
mod schema;

fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(&database_url);
    Pool::new(manager).expect("Could not connect to database")
}

#[rocket::get("/")]
fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<GrahpQLSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<GrahpQLSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema).await
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().manage(create_schema(establish_connection())).mount(
        "/",
        routes![graphql_query, graphql_request, graphql_playground],
    )
}
