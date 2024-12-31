use super::types::*;

use crate::api::transactions::types::NewTransactionRequest;
use crate::db;
use crate::db::models::UsersAccount;
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
    let (mut a, mut b) = (0, 10);

    let mut sort_column = "id".to_string();
    let mut sort_order = "ASC".to_string();

    // Split by comma and parse
    if let Some(range) = &info.range {
        if let Some(trimmed) = range.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            let parts: Vec<&str> = trimmed.split(',').collect();
            if parts.len() == 2 {
                a = parts[0].parse().unwrap_or(0);
                b = parts[1].parse().unwrap_or(10);
            }
        }
    }

    // Parse sort parameter if provided
    if let Some(sort) = &info.sort {
        if let Some(trimmed) = sort.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            let parts: Vec<&str> = trimmed.split(',').map(|s| s.trim_matches('"')).collect();
            if parts.len() == 2 {
                sort_column = parts[0].to_string();
                sort_order = parts[1].to_uppercase();
                if !["ASC", "DESC"].contains(&sort_order.as_str()) {
                    sort_order = "ASC".to_string(); // Default to ASC if invalid
                }
            }
        }
    }

    let today: NaiveDateTime = chrono::Local::now().naive_local();
    let first_day: NaiveDate = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
        .and_then(|date| date.checked_sub_months(Months::new(48)))
        .ok_or_else(|| error::ErrorInternalServerError("Invalid first day calculation"))?;
    let last_day: NaiveDate = first_day
        .checked_add_months(Months::new(48))
        .and_then(|date| date.checked_sub_days(Days::new(1)))
        .ok_or_else(|| error::ErrorInternalServerError("Invalid last day calculation"))?;

    debug!(
        "today: {}\t\tfirst_day: {}\t\tlast_day: {}",
        today, first_day, last_day
    );
    debug!("query string info: {:?} {} {}", info, a, b);
    debug!("claim: {:?}", &claims.sub);

    let qtd = web::block({
        let pool = pool.clone();
        let sub = claims.sub.clone();
        move || {
            // Obtaining a connection from the pool is also a potentially blocking operation.
            // So, it should be called within the `web::block` closure, as well.
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            crate::db::models::Transaction::count(&sub, &mut conn)
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let transactions = web::block({
        let pool = pool.clone();
        let sub = claims.sub.clone();
        let sort_column = sort_column.clone();
        let sort_order = sort_order.clone();

        move || {
            // Obtaining a connection from the pool is also a potentially blocking operation.
            // So, it should be called within the `web::block` closure, as well.
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            crate::db::models::Transaction::month(
                first_day,
                last_day,
                (a, b),
                &sort_column,
                &sort_order,
                &sub,
                &mut conn,
            )
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .insert_header(("Access-Control-Expose-Headers", "X-Total-Count"))
        .insert_header(("X-Total-Count", format!("{}", qtd)))
        .json(transactions))
}

#[post("")]
pub async fn create(
    pool: web::Data<DbPool>,
    request: Json<NewTransactionRequest>,
    claims: Claims,
) -> actix_web::Result<impl Responder> {
    let pool = pool.clone();

    let transaction = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        let account: UsersAccount =
            db::models::UsersAccount::get_one(request.account_id, claims.sub, &mut conn).unwrap();
        let new_transaction = crate::db::models::NewTransaction {
            transacted_date: request.transacted_date.to_owned(),
            amount: request.amount.to_owned(),
            description: request.description.to_owned(),
            label: request.label.to_owned(),
            kind: request.kind.to_owned().to_string(),
            currency: request.currency.to_owned(),
            account_id: account.account_id,
            opts: request.opts.to_owned(),
        };

        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.

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
