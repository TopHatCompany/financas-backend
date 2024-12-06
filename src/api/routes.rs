use super::summary;
use super::transactions;
use actix_web::web;
use actix_web::Scope;

pub fn routes() -> Scope {
    web::scope("/api")
        .service(transactions::routes())
        .service(summary::routes())
}
