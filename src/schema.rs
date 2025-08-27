// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        reply -> Nullable<Varchar>,
        upvotes -> Int4,
        got_replies -> Int4,
        userid -> Varchar,
        wiki_entry -> Varchar,
    }
}
