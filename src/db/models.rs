use crate::db::schema::transactions;
use crate::db::schema::users;
use crate::db::schema::users_accounts;
use actix_web::web;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Queryable, Selectable, Identifiable, Serialize, PartialEq)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = users)]
pub struct User {
    // #[diesel(deserialize_as = uuid::Uuid)]
    #[serde(rename = "id")]
    pub user_id: Uuid,
    pub identification: String,
    pub username: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Queryable, Selectable, Identifiable, Associations, Serialize, PartialEq)]
#[diesel(primary_key(account_id))]
#[diesel(table_name = users_accounts)]
#[diesel(belongs_to(User))]
pub struct UsersAccount {
    // #[diesel(deserialize_as = uuid::Uuid)]
    #[serde(rename = "id")]
    pub account_id: Uuid,
    // #[diesel(deserialize_as = uuid::Uuid)]
    pub user_id: Uuid,
    pub identification: String,
    pub kind: String,
    pub initial_amount: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Queryable, Selectable, Identifiable, Associations, Serialize, PartialEq)]
#[diesel(table_name = transactions)]
#[diesel(belongs_to(UsersAccount, foreign_key = account_id))]
pub struct Transaction {
    #[diesel(deserialize_as = uuid::Uuid)]
    pub id: Uuid,
    pub transacted_date: NaiveDate,
    pub amount: BigDecimal,
    pub currency: String,
    // pub account: UsersAccount,
    pub account_id: Uuid,
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

impl User {
    pub fn get_by_sub(sub: String, conn: &mut PgConnection) -> diesel::QueryResult<User> {
        use crate::db::schema::users::dsl::*;
        users.filter(identification.eq(sub)).first::<User>(conn)
    }

    pub fn get_all_by_sub(sub: String, conn: &mut PgConnection) -> diesel::QueryResult<Vec<User>> {
        use crate::db::schema::users::dsl::*;
        users.filter(identification.eq(sub)).load(conn)
    }
}

impl UsersAccount {
    pub fn get_all(sub: String, conn: &mut PgConnection) -> diesel::QueryResult<Vec<UsersAccount>> {
        let user = User::get_by_sub(sub, conn).expect("expect to get an existent user");
        UsersAccount::belonging_to(&user).load::<UsersAccount>(conn)
    }

    pub fn get_one(
        id_to_find: uuid::Uuid,
        _sub: String,
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<UsersAccount> {
        use crate::db::schema::users_accounts::dsl::*;

        // let user = User::get_by_sub(sub, conn).expect("expect to get an existent user");
        users_accounts.find(id_to_find).first(conn)
    }

    pub fn get_transactions(
        id: uuid::Uuid,
        sub: String,
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<Vec<Transaction>> {
        let account = UsersAccount::get_one(id, sub, conn).expect("expect to get an existent user");
        Transaction::belonging_to(&account).load(conn)
    }
}

impl Transaction {
    pub(crate) fn all(sub: &str, conn: &mut PgConnection) -> diesel::QueryResult<Vec<Transaction>> {
        // use crate::db::schema::transactions::dsl::*;
        use crate::db::schema::users::dsl::*;
        // use crate::db::schema::users_accounts::dsl::*;
        // normal diesel operations
        Transaction::belonging_to(
            &UsersAccount::belonging_to(&users.filter(identification.eq(sub)).first::<User>(conn)?)
                .load::<UsersAccount>(conn)?,
        )
        .load(conn)
    }

    pub(crate) fn month(
        _lower_transacted: NaiveDate,
        _upper_transacted: NaiveDate,
        range: (i64, i64),
        sort_column: &str,
        sort_order: &str,
        sub: &str,
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<Vec<Transaction>> {
        use crate::db::schema::transactions::dsl::*;
        use crate::db::schema::users::dsl::*;

        let user_accounts =
            UsersAccount::belonging_to(&users.filter(identification.eq(sub)).first::<User>(conn)?)
                .load::<UsersAccount>(conn)?;

        // Determine sorting column and direction
        let mut query = Transaction::belonging_to(&user_accounts).into_boxed();

        // Apply sorting dynamically
        query = match sort_column {
            "id" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(id.desc())
                } else {
                    query.order(id.asc())
                }
            }
            "transacted_date" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(transacted_date.desc())
                } else {
                    query.order(transacted_date.asc())
                }
            }
            "amount" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(amount.desc())
                } else {
                    query.order(amount.asc())
                }
            }
            "currency" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(currency.desc())
                } else {
                    query.order(currency.asc())
                }
            }
            "description" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(description.desc())
                } else {
                    query.order(description.asc())
                }
            }
            "label" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(label.desc())
                } else {
                    query.order(label.asc())
                }
            }
            "kind" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(kind.desc())
                } else {
                    query.order(kind.asc())
                }
            }
            "opts" => {
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(opts.desc())
                } else {
                    query.order(opts.asc())
                }
            }
            "created_at" => {
                use crate::db::schema::transactions;
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(transactions::created_at.desc()) // Fully qualified
                } else {
                    query.order(transactions::created_at.asc()) // Fully qualified
                }
            }
            "updated_at" => {
                use crate::db::schema::transactions;
                if sort_order.eq_ignore_ascii_case("DESC") {
                    query.order(transactions::updated_at.desc()) // Fully qualified
                } else {
                    query.order(transactions::updated_at.asc()) // Fully qualified
                }
            }
            _ => {
                // Default to transacted_date ASC
                query.order(transacted_date.desc())
            }
        };
        query.limit(range.1).offset(range.0).load(conn)
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

    pub(crate) fn count(sub: &str, conn: &mut PgConnection) -> diesel::QueryResult<i64> {
        use crate::db::schema::users::dsl::*;
        // normal diesel operation
        Transaction::belonging_to(
            &UsersAccount::belonging_to(&users.filter(identification.eq(sub)).first::<User>(conn)?)
                .load::<UsersAccount>(conn)?,
        )
        .count()
        .first(conn)
    }
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub transacted_date: NaiveDate,
    pub amount: BigDecimal,
    pub currency: String,
    pub account_id: uuid::Uuid,
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
