// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Uuid,
        transacted_date -> Date,
        amount -> Numeric,
        currency -> Text,
        account_id -> Uuid,
        description -> Text,
        label -> Text,
        kind -> Text,
        opts -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        identification -> Text,
        username -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users_accounts (account_id) {
        account_id -> Uuid,
        user_id -> Uuid,
        identification -> Text,
        kind -> Text,
        initial_amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(transactions -> users_accounts (account_id));
diesel::joinable!(users_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(transactions, users, users_accounts,);
