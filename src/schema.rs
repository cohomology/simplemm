table! {
    mailing_lists (id) {
        id -> Integer,
        title -> Varchar,
        email -> Varchar,
        enabled -> Bool,
    }
}

table! {
    messages (id) {
        id -> Integer,
        list_id -> Integer,
        email -> Varchar,
        received -> Timestamp,
        message -> Longtext,
        send -> Nullable<Timestamp>,
    }
}

table! {
    users (list_id, email) {
        list_id -> Integer,
        email -> Varchar,
        password -> Varchar,
        enabled -> Bool,
        secret -> Varchar,
    }
}

joinable!(users -> mailing_lists (list_id));

allow_tables_to_appear_in_same_query!(
    mailing_lists,
    messages,
    users,
);
