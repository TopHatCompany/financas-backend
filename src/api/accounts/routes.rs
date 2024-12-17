use actix_web::{web, Scope};

use super::handlers::*;

pub fn routes() -> Scope {
    web::scope("/accounts")
        .service(get)
        .service(get_one)
        .service(get_transactions)
}
