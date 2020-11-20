table! {
    inbound_messages (id) {
        id -> Integer,
        list_id -> Integer,
        email -> Varchar,
        received -> Timestamp,
        message -> Longtext,
    }
}

table! {
    mailing_lists (id) {
        id -> Integer,
        title -> Varchar,
        email -> Varchar,
        enabled -> Bool,
    }
}

table! {
    outbound_messages (id) {
        id -> Integer,
        inbound_id -> Integer,
        send -> Timestamp,
        message -> Longtext,
    }
}

table! {
    secrets (id) {
        id -> Integer,
        list_id -> Integer,
        email -> Varchar,
        secret -> Varchar,
        valid_to -> Timestamp,
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

joinable!(outbound_messages -> inbound_messages (inbound_id));
joinable!(users -> mailing_lists (list_id));

allow_tables_to_appear_in_same_query!(
    inbound_messages,
    mailing_lists,
    outbound_messages,
    secrets,
    users,
);
