use actix_cors::Cors;
use actix_web::http::header::HeaderMap;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::middleware::Logger;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use std::env;

mod database;
use database::Database;
mod models;
use models::{create_schema, GrahpQLSchema};
mod validate;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
mod schema;
mod crypto;
use crypto::SecretKey;

/// Runs all migrations for the database
fn run_migrations(db: &Database) {
    let connection = db.get();
    embedded_migrations::run(&connection).expect("Migrations could not be applied successfully");
}

/// The `Token` that was sent by the client for authentication
#[derive(Debug)]
pub struct Token(Option<String>);

impl Token {
    pub fn get_token(&self) -> Option<&String> {
        self.0.as_ref()
    }
}

/// Retrieves the `Token` Header from the request and returns a `Token` object that can be passed as context
fn get_token_from_headers(headers: &HeaderMap) -> Token {
    match headers.get("Token") {
        Some(token_value) => Token(Some(
            token_value
                .to_str()
                .expect("Could not parse token to string")
                .to_string(),
        )),
        None => Token(None),
    }
}

embed_migrations!();

async fn gql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn index(
    schema: web::Data<GrahpQLSchema>,
    secret_key: web::Data<SecretKey>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    request = request
        .data(get_token_from_headers(req.headers()))
        .data(secret_key.into_inner());
    schema.execute(request).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = SecretKey(env::var("SECRET_KEY").expect("SECRET_KEY must be set"));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Server listening on http://localhost:8000");
    HttpServer::new(move || {
        let db = Database::new(&database_url);
        run_migrations(&db);

        let cors = Cors::permissive();
        // .allowed_methods(vec!["GET", "POST"])
        // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        // .allowed_header(header::CONTENT_TYPE)
        // .max_age(3600);

        App::new()
            .app_data(web::Data::new(create_schema(db)))
            .app_data(web::Data::new(secret_key.clone()))
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i Code(%s) URL(%U)"))
            .service(web::resource("/").guard(guard::Get()).to(gql_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
