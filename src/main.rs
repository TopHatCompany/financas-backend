mod api;
mod db;
mod model;

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use log::info;
use std::env;

use dotenv::dotenv;
use crate::api::transaction::*;
use crate::db::utils::get_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("CLIENT_HOST", "http://localhost:5173");
    env_logger::init();

    info!("Hello, world!");
    HttpServer::new(move || {
        let logger = middleware::Logger::default();
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .wrap(logger)
            .service(get_summary)
            .service(get_transactions)
            .service(get_transaction)
            .service(delete_transaction)
            .service(create_transaction)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
