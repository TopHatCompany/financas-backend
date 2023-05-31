use std::cmp::Reverse;

use crate::db::utils::DbPool;
use crate::model::transaction::TransactionKind;
use actix_web::web::Path;
use actix_web::{
    delete, get, post,
    web::{self, Json},
};
use actix_web::{error, HttpResponse, Responder};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Trasaction {
    label: String,
}

#[derive(Deserialize)]
pub struct NewTransactionRequest {
    pub transacted_date: chrono::NaiveDate,
    pub amount: bigdecimal::BigDecimal,
    pub currency: String,
    pub account: String,
    pub description: String,
    pub label: String,
    pub kind: String,
    pub opts: String,
}

#[derive(Debug, Serialize)]
pub struct SummaryAccount {
    pub account: String,
    pub transactions: Vec<crate::db::models::Transaction>,
}

#[get("/accounts")]
pub async fn get_summary(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::db::models::Transaction::all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    transactions.sort_by_key(|a| Reverse(a.transacted_date));

    let mut result: HashMap<String, Vec<crate::db::models::Transaction>> = HashMap::new();

    transactions.into_iter().for_each(|t| {
        let group = result.entry(t.account.to_lowercase()).or_insert(vec![]);
        group.push(t);
    });
    Ok(HttpResponse::Ok().json(result))
}

#[get("/transactions")]
pub async fn get_transactions(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::db::models::Transaction::all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    transactions.sort_by_key(|a| Reverse(a.transacted_date));
    Ok(HttpResponse::Ok().json(transactions))
}

#[post("/transactions")]
pub async fn create_transaction(
    pool: web::Data<DbPool>,
    request: Json<NewTransactionRequest>,
) -> actix_web::Result<impl Responder> {
    let new_transaction = crate::db::models::NewTransaction {
        transacted_date: request.transacted_date.to_owned(),
        amount: request.amount.to_owned(),
        description: request.description.to_owned(),
        label: request.label.to_owned(),
        kind: request.kind.to_owned().to_string(),
        currency: request.currency.to_owned(),
        account: request.account.to_owned(),
        opts: request.opts.to_owned(),
    };

    let transaction = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        new_transaction.create(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(transaction))
}

#[get("/transactions/{transaction_id}")]
pub async fn get_transaction(
    pool: web::Data<DbPool>,
    transaction_id: Path<uuid::Uuid>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::db::models::Transaction::one(transaction_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/transactions/{transaction_id}")]
pub async fn delete_transaction(
    pool: web::Data<DbPool>,
    transaction_id: Path<uuid::Uuid>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::db::models::Transaction::erase(transaction_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
