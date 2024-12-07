use super::accounts;
use super::summary;
use super::transactions;
use super::users;
use actix_web::web;
use actix_web::Scope;

pub fn routes() -> Scope {
    web::scope("/api")
        .service(accounts::routes())
        .service(summary::routes())
        .service(transactions::routes())
        .service(users::routes())
}
