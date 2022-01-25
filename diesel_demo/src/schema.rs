table! {
    channels (id) {
        id -> Integer,
        title -> Text,
        name -> Text,
        description -> Text,
        url -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    channels,
    posts,
);
