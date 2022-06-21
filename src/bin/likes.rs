extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::diesel::prelude::*;

use rocket::{routes, get};

pub fn main() {}


#[get("/like/<item_id>")]
fn like_item_req(item_id: i64) {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();

    diesel::update(items.find(item_id))
        .set(likes.eq(likes + 1))
        .execute(&connection)
        .expect("error liking image");

}

#[get("/unlike/<item_id>")]
fn unlike_item_req(item_id: i64) {
    use drp02_backend::schema::items::dsl::*;

    let connection = establish_connection();

    diesel::update(items.find(item_id))
        .set(likes.eq(likes - 1))
        .execute(&connection)
        .expect("error unliking image");
}

pub fn routes() -> Vec<rocket::Route> {
    routes![like_item_req, unlike_item_req]
}