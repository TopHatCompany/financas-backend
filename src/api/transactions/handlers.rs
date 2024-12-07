use super::types::*;
use std::cmp::Reverse;

use crate::api::transactions::types::NewTransactionRequest;
use crate::db::utils::DbPool;
use crate::extractors::Claims;
use actix_web::web::Path;
use actix_web::{
    delete, get, post,
    web::{self, Json},
};
use actix_web::{error, HttpResponse, Responder};
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime};
use log::debug;

#[get("")]
pub async fn get_all(
    info: web::Query<Info>,
    pool: web::Data<DbPool>,
    claims: Claims,
) -> actix_web::Result<impl Responder> {
    let mut a = 0;
    let mut b = 10;
    let pool = pool.clone();
    // Split by comma and parse
    if let Some(range) = &info.range {
        let trimmed = &range[1..range.len() - 1];
        let parts: Vec<&str> = trimmed.split(',').collect();
        a = parts[0].parse().unwrap();
        b = parts[1].parse().unwrap();
    }

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

    debug!("claim: {:?}", claims.sub);
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let qtd = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        crate::db::models::Transaction::count(claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    let mut transactions = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        crate::db::models::Transaction::month(first_day, last_day, (a, b), claims.sub, &mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;
    transactions.sort_by_key(|a| Reverse(a.transacted_date));
    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", qtd)))
        .json(transactions))
}

#[post("")]
pub async fn create(
    pool: web::Data<DbPool>,
    request: Json<NewTransactionRequest>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
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

#[get("/{transaction_id}")]
pub async fn retrieve(
    pool: web::Data<DbPool>,
    transaction_id: Path<uuid::Uuid>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
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

#[delete("/{transaction_id}")]
pub async fn erase(
    pool: web::Data<DbPool>,
    transaction_id: Path<uuid::Uuid>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();
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
