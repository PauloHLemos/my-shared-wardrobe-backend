#[derive(Queryable)]
pub struct User {
    pub uid: i64,
    pub name: String,
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub uid: i64,
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pics: Vec<String>,
}

use super::schema::items;
#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub item_id: &'a i64,
    pub uid: &'a i64,
    pub type_: &'a str,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub pics: Vec<&'a str>,
}