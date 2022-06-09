table! {
    Items (item_id) {
        item_id -> Int8,
        uid -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    Users (uid) {
        uid -> Int8,
        name -> Varchar,
    }
}

joinable!(Items -> Users (uid));

allow_tables_to_appear_in_same_query!(
    Items,
    Users,
);
