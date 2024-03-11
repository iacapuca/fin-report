use crate::graphql::schema::{create_schema, Schema};
use actix_cors::Cors;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use dotenvy::dotenv;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

mod graphql;
mod models;

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql_handler(
    st: web::Data<std::sync::Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    let res = data.execute(&st, &pool).await;
    HttpResponse::Ok().json(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => {
            log::info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            log::error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let schema = std::sync::Arc::new(create_schema());
    let port = 8888;

    log::info!("Starting on Port: http://localhost:{}", port);
    log::info!("Playground running on: http://localhost:{}/graphiql", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .service(graphql_handler)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
