extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

use drp02_backend::auth::AuthenticatedUser;
use drp02_backend::models::{Item, NewItem, NewItemUser, User};

// use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use rocket::serde::json::Json;
use rocket::{get, post, routes};


pub fn main() {}

#[get("/wardrobe")]
fn wardrobe(auth_user: AuthenticatedUser)  -> Json<Vec<Item>>{
    let _id: i64 = auth_user.uid;
    get_items_with_id(_id).into()
}

#[get("/see_wardrobe/<see_id>")]
fn see_wardrobe(see_id: i64, auth_user: AuthenticatedUser)  -> Json<Vec<Item>>{
    get_items_with_id(see_id).into()
}

#[get("/feed")]
fn feed(auth_user: AuthenticatedUser) -> Json<Vec<Item>>{
    use drp02_backend::schema::users::dsl::*;
    let connection = establish_connection();

    let following_ids: Vec<i64> = users.find(auth_user.uid)
        .load::<User>(&connection)
        .expect("Error loading user")
        .remove(0)
        .users_following;

    let mut follower_wardrobes: Vec<Item> = Vec::new();

    for following_id in following_ids {
        let following_wardrobe: Vec<Item> = get_items_with_id(following_id);
        for item in following_wardrobe{
            follower_wardrobes.push(item);
        } 
    }
    follower_wardrobes.into()
}


#[get("/discover")]
fn discover(auth_user: AuthenticatedUser) -> Json<Vec<Item>>{
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

#[post("/insert", format = "json", data = "<item>")]
fn insert_item(item: Json<NewItem>, auth_user: AuthenticatedUser) {
    let new_item_user = NewItemUser {uid: auth_user.uid, 
                                    type_: item.type_, 
                                    name: item.name, 
                                    description: item.description, 
                                    tags: item.tags.clone(), 
                                    pics: item.pics.clone(), 
                                    likes: item.likes, 
                                    creation_time: item.creation_time };
    new_item(&new_item_user);
}

fn new_item(item: &NewItemUser) {
    let connection = establish_connection();

    use schema::items;

    let _item: Item = diesel::insert_into(items::table)
        .values(item)
        .get_result(&connection)
        .expect("Error saving new item");
        // TODO: use .update
}

#[get("/delete/<id>")]
fn delete_item(id: i64, auth_user: AuthenticatedUser) {

    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();

    diesel::delete(items.find(id))
        .filter(uid.eq(auth_user.uid))
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
    routes![wardrobe, feed, see_wardrobe,
        insert_item, delete_item, discover]
}