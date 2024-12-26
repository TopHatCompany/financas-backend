use std::{cmp::Reverse, collections::HashMap};

use actix_web::{error, get, web, HttpResponse, Responder};

use crate::{db::utils::DbPool, extractors::Claims};

#[get("")]
pub async fn get(pool: web::Data<DbPool>, claims: Claims) -> actix_web::Result<impl Responder> {
    let mut transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::db::models::Transaction::all(&claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    transactions.sort_by_key(|a| Reverse(a.transacted_date));

    let mut result: HashMap<String, Vec<crate::db::models::Transaction>> = HashMap::new();

    transactions.into_iter().for_each(|t| {
        let group = result.entry(t.account_id.to_string()).or_default();
        group.push(t);
    });
    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", 1)))
        .json(result))
}
