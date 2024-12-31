use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct UsersAccount {
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub identification: String,
    pub kind: String,
    pub initial_amount: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
