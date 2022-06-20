use rocket::serde::{Serialize, Deserialize};
use rocket::form::FromForm;
use super::schema::{items, users, users_auth};

// ------------------------------------ user ------------------------------------------

// reference: https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
// used tutorial to implement user authentication

// extract info from users table
#[derive(Queryable, Serialize)]
pub struct User {
    pub uid: i64,
    pub name: String,
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
    pub password: &'a str,
}

// data passed to create new user in users table
#[derive(Insertable, FromForm, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
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

// data passed to create new item in items table
#[derive(Insertable, Deserialize)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub uid: i64,
    pub type_: &'a str,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub pics: Vec<&'a str>,
    pub likes: i64,
}