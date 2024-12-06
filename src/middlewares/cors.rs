use actix_cors::Cors;
use actix_web::http::{header, Method};

pub fn cors(client_origin_url: &str) -> Cors {
    Cors::default()
        .allowed_origin(client_origin_url)
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
