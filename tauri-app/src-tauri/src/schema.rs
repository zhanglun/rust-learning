table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        link -> Text,
        description -> Text,
        author -> Text,
        content -> Text,
        category -> Integer,
        comments -> Text,
        pub_date -> Timestamp,
        create_date -> Timestamp,
        update_date -> Timestamp,
        has_read -> Integer,
        is_like -> Integer,
        channel_id -> Text,
    }
}

table! {
    channels (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        feed_url -> Text,
        link -> Text,
        ttl -> Integer,
        favicon -> Text,
        category -> Text,
        tag -> Text,
        create_date -> Timestamp,
        update_date -> Timestamp,
        last_sync_date -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    channels,
);
