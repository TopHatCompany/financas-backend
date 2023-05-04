// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Uuid,
        transacted_date -> Date,
        amount -> Numeric,
        description -> Text,
        label -> Text,
        kind -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
