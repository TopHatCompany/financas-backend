use actix_web::{error, get, web, HttpResponse, Responder};

use crate::{db::utils::DbPool, extractors::Claims};

#[get("")]
pub async fn get(claims: Claims, pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let accounts = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        crate::db::models::UsersAccount::get_all(claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", accounts.len())))
        .json(accounts))
}

#[get("/{id}")]
pub async fn get_one(
    claims: Claims,
    id: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let account = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        crate::db::models::UsersAccount::get_one(*id, claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(account))
}

#[get("/{id}/transactions")]
pub async fn get_transactions(
    claims: Claims,
    id: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        crate::db::models::UsersAccount::get_transactions(*id, claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", transactions.len())))
        .json(transactions))
}
