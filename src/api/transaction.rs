use std::cmp::Reverse;

use crate::db::utils::DbPool;
use actix_web::web::Path;
use actix_web::{
    delete, get, post,
    web::{self, Json},
};
use actix_web::{error, HttpResponse, Responder};
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime};
use log::debug;
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
        let group = result.entry(t.account.to_lowercase()).or_default();
        group.push(t);
    });
    Ok(HttpResponse::Ok().json(result))
}

#[derive(Deserialize, Debug)]
struct Info {
    sort: Option<String>,
    filter: Option<String>,
    range: Option<String>,
}

#[get("/transactions")]
pub async fn get_transactions(
    info: web::Query<Info>,
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    let mut a = 0;
    let mut b = 10;

    if let Some(range) = &info.range {
        let trimmed = &range[1..range.len() - 1];
        let parts: Vec<&str> = trimmed.split(',').collect();
        a = parts[0].parse().unwrap();
        b = parts[1].parse().unwrap();
    }

    // Split by comma and parse
    let today: NaiveDateTime = chrono::Local::now().naive_local();
    let first_day: NaiveDate = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
        .unwrap()
        .checked_sub_months(Months::new(48))
        .unwrap();
    let last_day: NaiveDate = first_day
        .checked_add_months(Months::new(48))
        .unwrap()
        .checked_sub_days(Days::new(1))
        .unwrap();
    debug!(
        "today: {}\t\tfirst_day: {}\t\tlast_day: {}",
        today, first_day, last_day
    );
    debug!("query string info: {:?} {} {}", info, a, b);
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let qtd = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        crate::db::models::Transaction::count(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    let mut transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        crate::db::models::Transaction::month(first_day, last_day, (a, b), &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    transactions.sort_by_key(|a| Reverse(a.transacted_date));
    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", qtd)))
        .json(transactions))
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
