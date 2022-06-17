extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

pub fn main() {}

pub fn like_item(id: i64) {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();

    diesel::update(items.find(id))
        .set(likes.eq(likes + 1))
        .execute(&connection)
        .expect("error liking image");

}

pub fn unlike_item(id: i64) {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();

    diesel::update(items.find(id))
        .set(likes.eq(likes - 1))
        .execute(&connection)
        .expect("error unliking image");
}