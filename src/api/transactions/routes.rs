use actix_web::{web, Scope};

use super::handlers::*;

pub fn routes() -> Scope {
    web::scope("/transactions")
        .service(get_all)
        .service(create)
        .service(retrieve)
        .service(erase)
}
