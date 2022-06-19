use rocket::serde::{Serialize, Deserialize};
use rocket::form::FromForm;
use super::schema::{items, users, users_auth};

// ------------------------------------ user ------------------------------------------

// reference: https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
// used tutorial to implement user authentication

#[derive(Queryable, Serialize)]
pub struct User {
    pub uid: i64,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Insertable, Deserialize)]
#[table_name="users_auth"]
pub struct AuthInfo {
    pub uid: i64,
    pub password_hash: String,
}

#[derive(FromForm, Deserialize)]
struct CreateInfo {
    name: String,
    // email: String,
    // age: i32,
    // password: String
}

#[derive(FromForm, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}


// ------------------------------------ item ------------------------------------------

#[derive(Queryable, Serialize, Debug)]
pub struct Item {
    pub id: i64,
    pub uid: i64,
    pub type_: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pics: Vec<String>,
    pub likes: i64,
}

#[derive(Insertable, Deserialize)]
#[table_name="items"]
pub struct NewItem<'a> {
    // pub item_id: &'a i64,
    pub uid: i64,
    pub type_: &'a str,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub pics: Vec<&'a str>,
    pub likes: i64,
}