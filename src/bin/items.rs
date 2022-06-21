extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use drp02_backend::models::{Item, NewItem};
use self::diesel::prelude::*;

use chrono::{NaiveDateTime, NaiveDate, NaiveTime};


pub fn main() {}

pub fn insert_item_plain(type_: &str, name: &str) {
    // TODO: user id currently hardcoded to 1

    let new_item = NewItem {
        uid: 1,
        type_: type_,
        name: name,
        description: Some("sample description"), // description
        tags: None,
        pics: vec!["dummy_url.com"],
        likes: 0,
        creation_time: NaiveDateTime::new(NaiveDate::from_ymd(2022, 6, 20), NaiveTime::from_hms_milli(00, 00, 00, 000)),
    };

    insert_item(&new_item);
}

pub fn insert_item(new_item: &NewItem) -> Item {
    let connection = establish_connection();

    use schema::items;

    diesel::insert_into(items::table)
        .values(new_item)
        .get_result(&connection)
        .expect("Error saving new item")
        // TODO: use .update
}

pub fn delete_item(id: i64) {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    diesel::delete(items.find(id))
        .execute(&connection)
        .expect("Error deleting item");
}

pub fn get_items() -> Vec<Item> {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    items.filter(uid.eq(1))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items")

}

pub fn get_items_with_id(id: i64) -> Vec<Item> {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    items.filter(uid.eq(id))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items")
    
}

pub fn get_my_feed(id: i64) -> Vec<Item> {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    
    items.filter(uid.ne(id))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items")
    
}