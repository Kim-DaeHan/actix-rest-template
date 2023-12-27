// @generated automatically by Diesel CLI.

diesel::table! {
    newtable (id) {
        id -> Varchar,
        post_title -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    newtable,
    posts,
);
