// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Uuid,
        transacted_date -> Date,
        amount -> Numeric,
        currency -> Text,
        account -> Text,
        description -> Text,
        label -> Text,
        kind -> Text,
        opts -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        identification -> Text,
    }
}

diesel::table! {
    users_accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        identification -> Text,
        kind -> Text,
        initial_amount -> Numeric,
    }
}

diesel::joinable!(users_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    transactions,
    users,
    users_accounts,
);
