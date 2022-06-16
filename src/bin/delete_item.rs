extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

pub fn main() {}

pub fn delete_item(id: i64) {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();
    diesel::delete(items.find(id))
        .execute(&connection)
        .expect("Error deleting item");
}