table! {
    items (item_id) {
        item_id -> Int8,
        uid -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        tags -> Nullable<Array<Text>>,
        pics -> Array<Text>,
        likes -> Int8,
        creation_time -> Timestamp,
    }
}

table! {
    users (uid) {
        uid -> Int8,
        name -> Varchar,
        email -> Varchar,
    }
}

table! {
    users_auth (uid) {
        uid -> Int8,
        password_hash -> Varchar,
    }
}

joinable!(items -> users (uid));
joinable!(users_auth -> users (uid));

allow_tables_to_appear_in_same_query!(
    items,
    users,
    users_auth,
);
