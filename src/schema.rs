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
    }
}

table! {
    users (uid) {
        uid -> Int8,
        name -> Varchar,
    }
}

joinable!(items -> users (uid));

allow_tables_to_appear_in_same_query!(
    items,
    users,
);
