use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Trasaction {
    label: String,
}

#[derive(Deserialize)]
pub struct NewTransactionRequest {
    pub transacted_date: chrono::NaiveDate,
    pub amount: bigdecimal::BigDecimal,
    pub currency: String,
    pub account_id: uuid::Uuid,
    pub description: String,
    pub label: String,
    pub kind: String,
    pub opts: String,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub sort: Option<String>,
    pub filter: Option<String>,
    pub range: Option<String>,
}
