use crate::db::utils::DbPool;
use crate::model::transaction::TransactionKind;
use actix_web::web::Path;
use actix_web::{error, HttpResponse, Responder};
use actix_web::{
    get, post, delete,
    web::{self, Json},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Trasaction {
    label: String,
}

#[derive(Deserialize)]
pub struct NewTransactionRequest {
    pub transacted_date: chrono::NaiveDate,
    pub amount: bigdecimal::BigDecimal,
    pub description: String,
    pub label: String,
    pub kind: TransactionKind,
}

#[get("/transactions")]
pub async fn get_transactions(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        crate::db::models::Transaction::all(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(transactions))
}

#[post("/transactions")]
pub async fn create_transaction(
    pool: web::Data<DbPool>,
    request: Json<NewTransactionRequest>,
) -> actix_web::Result<impl Responder> {
    let new_transaction = crate::db::models::NewTransaction {
        transacted_date: request.transacted_date.clone(),
        amount: request.amount.clone(),
        description: request.description.clone(),
        label: request.label.clone(),
        kind: request.kind.clone().to_string(),
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
