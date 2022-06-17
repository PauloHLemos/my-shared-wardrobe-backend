extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use drp02_backend::models::{Item, NewItem};
use self::diesel::prelude::*;


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
    };

    insert_item(&new_item);
}

pub fn insert_item(new_item: &NewItem) -> Item {
    let connection = establish_connection();

    use schema::items;

    diesel::insert_into(items::table)
        .values(new_item)
        .get_result(&connection)
        .expect("Error saving new post")
        // TODO: use .update
}