extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

use drp02_backend::auth::AuthenticatedUser;
use drp02_backend::models::{Item, NewItem, NewItemUser, User};

use std::env;
use futures::executor::block_on;
// use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use rocket::serde::json::Json;
use rocket::{get, post, routes};
use meilisearch_sdk::client::*;


pub fn main() {}

#[get("/wardrobe")]
fn wardrobe(auth_user: AuthenticatedUser)  -> Json<Vec<Item>>{
    let _id: i64 = auth_user.uid;
    get_items_with_id(_id).into()
}

#[get("/see_wardrobe/<see_id>")]
fn see_wardrobe(see_id: i64, _auth_user: AuthenticatedUser)  -> Json<Vec<Item>>{
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

    let res_item: Item = diesel::insert_into(items::table)
        .values(item)
        .get_result(&connection)
        .expect("Error saving new item");
        // TODO: use .update


    // index new item in mielisearch database, to be used for searching
    block_on(async move {
        let client = Client::new(env::var("MEILI_HOST").expect("error loading host env variable"),
                             env::var("MEILI_API_KEY").expect("error loading api key env variable"));
        

        let items = client.index("items");
        items.add_documents(&[res_item], Some("item_id")).await.unwrap();
                // println!("{:?}", items.search().with_query("tshirt").execute::<Item>().await.unwrap().hits);

    })
}

#[get("/delete/<id>")]
fn delete_item(id: i64, auth_user: AuthenticatedUser) {

    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();

    diesel::delete(items.find(id))
        .filter(uid.eq(auth_user.uid))
        .execute(&connection)
        .expect("Error deleting item");
    

    // delete item from mieliesearch database
    block_on(async move {
        let client = Client::new(env::var("MEILI_HOST").expect("error loading host env variable"),
                                env::var("MEILI_API_KEY").expect("error loading api key env variable"));

        let mielie_items = client.index("items");
        mielie_items.delete_documents(&vec![id]).await.unwrap()
            .wait_for_completion(&client, None, None).await.unwrap();
    })
}

// ---------------------------------------------------------------------------------------

fn get_items_with_id(id: i64) -> Vec<Item> {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    items.filter(uid.eq(id))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items")
    
}

// ---------------------------------- update ---------------------------------------------

#[post("/set_description/<item_id_>/<new_description>")]
fn set_description(item_id_: i64, new_description: String, _auth_user: AuthenticatedUser) {
    use drp02_backend::schema::items::dsl::*;
    let connection = establish_connection();

    // TODO: ensure item id belongs to user
    diesel::update(items.find(item_id_))
        .set(description.eq(Some(new_description)))
        .execute(&connection)
        .expect("error updating description");
}

#[post("/set_type/<item_id_>/<new_type>")]
fn set_type(item_id_: i64, new_type: String, _auth_user: AuthenticatedUser) {
    use drp02_backend::schema::items::dsl::*;
    let connection = establish_connection();

    // TODO: ensure item id belongs to user
    diesel::update(items.find(item_id_))
        .set(type_.eq(new_type))
        .execute(&connection)
        .expect("error updating type");
}



pub fn routes() -> Vec<rocket::Route> {
    routes![wardrobe, feed, see_wardrobe,
        insert_item, delete_item, discover, set_description, set_type]
}