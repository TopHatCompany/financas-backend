use bigdecimal;
use chrono::naive;
use serde::{Serialize, Deserialize};
use uuid;
use strum::{EnumString, Display};

#[derive(Clone, Deserialize, Serialize, EnumString, Display, Eq, PartialEq)]
pub enum TransactionKind {
    Entrada,
    Saida,
    Sandro,
    Emily,
    Transfer,
}

#[derive(Serialize)]
pub struct Transaction {
    pub id: Option<uuid::Uuid>,
    pub kind: TransactionKind,
    pub categoria: String,
    pub data: Option<naive::NaiveDate>,
    pub amount: bigdecimal::BigDecimal,
    pub account: String,
    pub label: String,
    pub description: String,
}
