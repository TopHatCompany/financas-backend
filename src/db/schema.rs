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
