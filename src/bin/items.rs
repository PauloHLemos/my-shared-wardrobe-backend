extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

use drp02_backend::auth::AuthenticatedUser;
use drp02_backend::models::{Item, NewItem};

// use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use rocket::serde::json::Json;
use rocket::{get, post, routes};


pub fn main() {}

#[get("/wardrobe")]
fn wardrobe(auth_user: AuthenticatedUser)  -> Json<Vec<Item>>{
    let _id: i64 = auth_user.uid;
    get_items_with_id(_id).into()
}

#[get("/feed")]
fn feed(auth_user: AuthenticatedUser) -> Json<Vec<Item>>{
    let _id: i64 = auth_user.uid;
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    
    items.filter(uid.ne(_id))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items")
        .into()    
}

// ------------------------------------------------------------------------------------

#[post("/insert", format = "json", data = "<new_item>")]
fn new_item(new_item: Json<NewItem>, auth_user: AuthenticatedUser) {
    insert_item(&new_item);
}

fn insert_item(new_item: &NewItem) {
    let connection = establish_connection();

    use schema::items;

    let _item: Item = diesel::insert_into(items::table)
        .values(new_item)
        .get_result(&connection)
        .expect("Error saving new item");
        // TODO: use .update
}

#[get("/delete/<id>")]
fn delete_item(id: i64) {

    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    diesel::delete(items.find(id))
        .execute(&connection)
        .expect("Error deleting item");
}

// ---------------------------------------------------------------------------

fn get_items_with_id(id: i64) -> Vec<Item> {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    items.filter(uid.eq(id))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items")
    
}

pub fn routes() -> Vec<rocket::Route> {
    routes![wardrobe, feed,
        new_item, delete_item]
}