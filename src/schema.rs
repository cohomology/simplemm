table! {
    mailing_lists (id) {
        id -> Integer,
        title -> Varchar,
        email -> Varchar,
        enabled -> Bool,
    }
}

table! {
    subscriptions (uuid) {
        uuid -> Binary,
        list_id -> Integer,
        email -> Varchar,
        timestamp -> Timestamp,
        request -> Text,
    }
}

table! {
    users (list_id, email) {
        list_id -> Integer,
        email -> Varchar,
        password -> Varchar,
        enabled -> Bool,
    }
}

joinable!(subscriptions -> mailing_lists (list_id));
joinable!(users -> mailing_lists (list_id));

allow_tables_to_appear_in_same_query!(
    mailing_lists,
    subscriptions,
    users,
);
