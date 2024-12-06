use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SummaryAccount {
    pub account: String,
    pub transactions: Vec<crate::db::models::Transaction>,
}
