mod api;
mod db;
mod model;

use actix_web::{middleware, web::Data, App, HttpServer};
use dotenvy::dotenv;
use std::env;

use crate::api::transaction::create_transaction;
use crate::api::transaction::get_transactions;
use crate::db::utils::get_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = get_pool(&db_url);

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    println!("Hello, world!");

    HttpServer::new(move || {
        let logger = middleware::Logger::default();
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::NormalizePath::default())
            .wrap(logger)
            .service(get_transactions)
            .service(create_transaction)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
