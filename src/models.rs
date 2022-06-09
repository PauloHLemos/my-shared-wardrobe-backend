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