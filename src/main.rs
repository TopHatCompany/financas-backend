mod api;
mod db;
mod extractors;
mod middlewares;
mod model;
mod types;

use std::sync::Arc;

use actix_web::{http::header, middleware, web, App, HttpServer};
use awc::{Client, Connector};
use log::debug;
use rustls::{ClientConfig, RootCertStore};

use crate::db::utils::get_pool;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let client_tls_config = Arc::new(rustls_config());
    let config = types::Config::default();
    let auth0_config = extractors::Auth0Config::default();
    let pool = get_pool(&config.database_url);
    debug!("{:?}", config.clone());
    debug!("{:?}", auth0_config.clone());

    HttpServer::new(move || {
        // create client _inside_ `HttpServer::new` closure to have one per worker thread
        let http_client = Client::builder()
            // Wikipedia requires a User-Agent header to make requests
            .add_default_header((header::USER_AGENT, "financas-rust/0.0.1"))
            // a "connector" wraps the stream into an encrypted connection
            .connector(Connector::new().rustls_0_23(Arc::clone(&client_tls_config)))
            .finish();

        App::new()
            .app_data(web::Data::new(auth0_config.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(http_client.clone()))
            .wrap(middleware::NormalizePath::trim())
            .wrap(middlewares::cors(&config.client_host))
            .wrap(middlewares::err_handlers())
            .wrap(middlewares::security_headers())
            .wrap(middlewares::logger())
            .service(api::routes())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}

/// Create simple rustls client config from root certificates.
fn rustls_config() -> ClientConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.to_owned());

    rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
