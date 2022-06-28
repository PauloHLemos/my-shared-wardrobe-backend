use rocket::serde::{Serialize, Deserialize};
use rocket::form::FromForm;
use chrono::NaiveDateTime;
use super::schema::{items, users, users_auth};

// ------------------------------------ user ------------------------------------------

// extract info from users table
#[derive(Queryable, Serialize)]
pub struct User {
    pub uid: i64,
    pub name: String,
    pub email: String,
    pub items_liked: Vec<i64>,
    pub users_following: Vec<i64>,
    pub phone_no: Option<String>,
}

// extract info from users_auth table
#[derive(Queryable, Serialize)]
pub struct UserAuth {
    pub uid: i64,
    pub password_hash: String,
}

// data received to create new user
#[derive(FromForm, Deserialize)]
pub struct NewUserData<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub phone_no: Option<String>,
}

// data passed to create new user in users table
#[derive(Insertable, FromForm, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub items_liked: Vec<i64>,
    pub users_following: Vec<i64>,
    pub phone_no: Option<String>,
}

// data passed to users_auth table
#[derive(Insertable, Deserialize)]
#[table_name="users_auth"]
pub struct NewUserAuth {
    pub uid: i64,
    pub password_hash: String,
}


// ------------------------------------ item ------------------------------------------

// extract info from items table
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Item {
    pub item_id: i64,
    pub uid: i64,
    pub type_: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pics: Vec<String>,
    pub likes: i64,
    pub creation_time: NaiveDateTime,
}

// data passed to create new item in items table
#[derive(Deserialize)]
pub struct NewItem<'a> {
    pub type_: &'a str,
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub pics: Vec<&'a str>,
    pub likes: i64,
    pub creation_time: NaiveDateTime,
}

// data passed to create new item in items table
#[derive(Insertable, Deserialize)]
#[table_name="items"]
pub struct NewItemUser<'a> {
    pub uid: i64,
    pub type_: &'a str,
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub pics: Vec<&'a str>,
    pub likes: i64,
    pub creation_time: NaiveDateTime,
}