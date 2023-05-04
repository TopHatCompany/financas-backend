use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::Serialize;
use uuid::Uuid;

use super::schema::transactions;

// Queryable, Identifiable, AsChangeset, Deserialize
#[derive(Clone, Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    #[diesel(deserialize_as = uuid::Uuid)]
    pub id: Uuid,
    pub transacted_date: NaiveDate,
    pub amount: BigDecimal,
    pub description: String,
    pub label: String,
    pub kind: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Transaction {
    pub(crate) fn all(conn: &mut PgConnection) -> diesel::QueryResult<Vec<Transaction>> {
        use crate::db::schema::transactions::dsl::*;
        // normal diesel operations
        let result: Result<Vec<Transaction>, diesel::result::Error> = transactions.load(conn);
        result
    }
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub transacted_date: NaiveDate,
    pub amount: BigDecimal,
    pub description: String,
    pub label: String,
    pub kind: String,
}

impl NewTransaction {

    pub(crate) fn create(self, conn: &mut PgConnection) -> diesel::QueryResult<Transaction> {
        use crate::db::schema::transactions::dsl::*;
        // normal diesel operations
        let result: Result<Transaction, diesel::result::Error> = diesel::insert_into(transactions)
            .values(&self)
            .get_result::<Transaction>(conn);
        result
    }

}
