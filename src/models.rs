#[derive(Queryable)]
pub struct User {
    pub uid: i64,
    pub name: String,
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub uid: i64,
    pub item: String,
}

use super::schema::Items;
#[derive(Insertable)]
#[table_name="Items"]
pub struct NewItem<'a> {
    pub item_id: &'a i64,
    pub uid: &'a i64,
    pub type_: &'a str,
}