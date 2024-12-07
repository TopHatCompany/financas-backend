use crate::db::schema::transactions;
use crate::db::schema::users;
use crate::db::schema::users_accounts;
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
}

impl UsersAccount {
    pub fn get_all(sub: String, conn: &mut PgConnection) -> diesel::QueryResult<Vec<UsersAccount>> {
        let user = User::get_by_sub(sub, conn).expect("expect to get an existent user");
        UsersAccount::belonging_to(&user).load::<UsersAccount>(conn)
    }

    pub fn get_one(id_to_find: Uuid, conn: &mut PgConnection) -> diesel::QueryResult<UsersAccount> {
        // use crate::db::schema::transactions::dsl::*;
        // use crate::db::schema::users::dsl::*;
        use crate::db::schema::users_accounts::dsl::*;

        users_accounts.find(id_to_find).first(conn)
    }
}

impl Transaction {
    pub(crate) fn all(
        _sub: String,
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<Vec<Transaction>> {
        use crate::db::schema::transactions::dsl::*;
        // use crate::db::schema::users::dsl::*;
        // use crate::db::schema::users_accounts::dsl::*;
        // normal diesel operations
        let result: Result<Vec<Transaction>, diesel::result::Error> = transactions
            // .inner_join(users_accounts.on(transactions.account.eq(users_accounts.identification)))
            // .inner_join(users.on(users_accounts.user_id.eq(users.id)))
            // .filter(users.identification.eq(sub))
            .load(conn);
        result
    }

    pub(crate) fn month(
        lower_transacted: NaiveDate,
        upper_transacted: NaiveDate,
        range: (i64, i64),
        sub: String,
        conn: &mut PgConnection,
    ) -> diesel::QueryResult<Vec<Transaction>> {
        use crate::db::schema::transactions::dsl::*;

        // users::table
        // .filter(users::identification.eq(sub))
        // .inner_join(users_accounts::table)
        // .inner_join(transactions::table)
        // .order_by(transactions::transacted_date.desc())
        // .limit(range.1)
        // .offset(range.0)
        // .select(transactions::as_select())
        // .load::<Transaction>(conn)

        // .filter(transactions::account_id.eq(sub))
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

    pub(crate) fn count(_sub: String, conn: &mut PgConnection) -> diesel::QueryResult<i64> {
        use crate::db::schema::transactions::dsl::*;
        // use crate::db::schema::users::dsl::*;
        // use crate::db::schema::users_accounts::dsl::*;
        // normal diesel operation
        transactions
            // .inner_join(users_accounts.on(transactions.account.eq(users_accounts.identification)))
            // .inner_join(users.on(users_accounts.user_id.eq(users.user_id)))
            // .filter(users.eq(sub))
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
