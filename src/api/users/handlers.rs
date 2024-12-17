use actix_web::{error, get, web, HttpResponse, Responder};

use crate::{db::utils::DbPool, extractors::Claims};

#[get("")]
pub async fn get_all(claims: Claims, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let users = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        crate::db::models::User::get_all_by_sub(claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", users.len())))
        .json(users))
}
