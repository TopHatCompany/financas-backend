use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
// use diesel::query_dsl::methods::LimitDsl;
use diesel::PgConnection;
use serde::Serialize;
use uuid::Uuid;

use super::schema::transactions;

// Queryable, Identifiable, AsChangeset, Deserialize
#[derive(Clone, Debug, Queryable, Selectable, Serialize, Identifiable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    #[diesel(deserialize_as = uuid::Uuid)]
    pub id: Uuid,
    pub transacted_date: NaiveDate,
    pub amount: BigDecimal,
    pub currency: String,
    pub account: String,
    pub description: String,
    pub label: String,
    pub kind: String,
    pub opts: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub(crate) struct _Filter {
    sort: String,
    filter: Option<String>,
    range: String,
}

impl Transaction {
    pub(crate) fn all(conn: &mut PgConnection) -> diesel::QueryResult<Vec<Transaction>> {
        use crate::db::schema::transactions::dsl::*;
        // normal diesel operations
        let result: Result<Vec<Transaction>, diesel::result::Error> = transactions.load(conn);
        result
    }

    pub(crate) fn month(
        lower_transacted: NaiveDate,
        upper_transacted: NaiveDate,
        range: (i64, i64),
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<Vec<Transaction>> {
        use crate::db::schema::transactions::dsl::*;

        transactions
            .order_by(transacted_date.desc())
            .limit(range.1)
            .offset(range.0)
            .load(conn)
    }

    pub(crate) fn one(
        id_to_find: Uuid,
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<Transaction> {
        use crate::db::schema::transactions::dsl::*;
        // normal diesel operation
        transactions.find(id_to_find).first(conn)
    }

    pub(crate) fn erase(id_to_erase: Uuid, conn: &mut PgConnection) -> diesel::QueryResult<usize> {
        use crate::db::schema::transactions::dsl::*;
        // normal diesel operation
        diesel::delete(transactions.filter(id.eq(id_to_erase))).execute(conn)
    }

    pub(crate) fn count(conn: &mut PgConnection) -> diesel::QueryResult<i64> {
        use crate::db::schema::transactions::dsl::*;
        // normal diesel operation
        transactions.count().first(conn)
    }
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub transacted_date: NaiveDate,
    pub amount: BigDecimal,
    pub currency: String,
    pub account: String,
    pub description: String,
    pub label: String,
    pub kind: String,
    pub opts: String,
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
