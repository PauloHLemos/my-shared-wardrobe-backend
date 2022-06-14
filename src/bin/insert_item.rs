extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;

pub fn main() {}

pub fn insert_item(item_id: &i64, type_: &str, name: &str) {
    let connection = establish_connection();
    // TODO: user id currently hardcoded to 1
    // TODO: so is item name
    let _item = add_item(&connection, item_id, &1, type_, name);
}