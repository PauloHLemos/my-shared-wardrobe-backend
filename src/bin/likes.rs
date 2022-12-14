extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

use drp02_backend::auth::AuthenticatedUser;
use drp02_backend::models::{User, Item};

use diesel::sql_query;
use diesel::dsl::any;
use rocket::{routes, get, post};
use rocket::serde::json::Json;

pub fn main() {}


#[post("/like/<_item_id>")]
fn like(_item_id: i64, auth_user: AuthenticatedUser) {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();

    let id: i64 = auth_user.uid;
    let mut user: Vec<User> = users.find(id)
        .load::<User>(&connection)
        .expect("Error loading user");

    // TODO: implement error handling if user not found (should never be the case)
    let liked_items = user.remove(0).items_liked;

    if !liked_items.contains(&_item_id) {
        use drp02_backend::schema::items::dsl::*;

        diesel::update(items.find(_item_id))
            .set(likes.eq(likes + 1))
            .execute(&connection)
            .expect("error liking image");

        // diesel::update(users.find(id))
        //     .set(items_liked.eq(items_liked))
        //     .execute(&connection)
        //     .expect("error liking image");

        sql_query(format!("UPDATE users SET items_liked = array_append(items_liked,'{_item_id}') WHERE uid = {id}"))
            .execute(&connection)
            .expect("error adding item to list of items_liked");
    }
}

#[post("/unlike/<_item_id>")]
fn unlike(_item_id: i64, auth_user: AuthenticatedUser) {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();

    let id: i64 = auth_user.uid;
    let mut user: Vec<User> = users.find(id)
        .load::<User>(&connection)
        .expect("Error loading user");

    // TODO: implement error handling if user not found (should never be the case)
    let liked_items = user.remove(0).items_liked;

    if liked_items.contains(&_item_id) {
        use drp02_backend::schema::items::dsl::*;

        diesel::update(items.find(_item_id))
            .set(likes.eq(likes - 1))
            .execute(&connection)
            .expect("error liking image");

        sql_query(format!("UPDATE users SET items_liked = array_remove(items_liked,'{_item_id}') WHERE uid = {id}"))
            .execute(&connection)
            .expect("error removing item from list of items_liked"); 
    }
}

#[get("/liked_items")]
fn get_liked_items(auth_user: AuthenticatedUser) -> Json<Vec<Item>> {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();

    let id: i64 = auth_user.uid;
    let liked_ids = users.find(id)
        .load::<User>(&connection)
        .expect("Error loading user")
        .remove(0)
        .items_liked;

    use drp02_backend::schema::items::dsl::*;

    items.filter(item_id.eq(any(liked_ids)))
        .load::<Item>(&connection)
        .expect("Error loading liked items")
        .into()
}

#[get("/liked_items/<id>")]
fn get_liked_items_with_id(id : i64, _auth_user: AuthenticatedUser) -> Json<Vec<Item>> {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();

    //let id: i64 = auth_user.uid;
    let liked_ids = users.find(id)
        .load::<User>(&connection)
        .expect("Error loading user")
        .remove(0)
        .items_liked;

    use drp02_backend::schema::items::dsl::*;

    items.filter(item_id.eq(any(liked_ids)))
        .load::<Item>(&connection)
        .expect("Error loading liked items")
        .into()
}


pub fn routes() -> Vec<rocket::Route> {
    routes![like, unlike, get_liked_items,get_liked_items_with_id]
}