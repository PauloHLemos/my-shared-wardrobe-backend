extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::models::*;
use self::diesel::prelude::*;

pub fn main() {}

pub fn get_items() -> Vec<Item> {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    let results = items.filter(uid.eq(1))
        .limit(100)
        .load::<Item>(&connection)
        .expect("Error loading items");

    return results;
}