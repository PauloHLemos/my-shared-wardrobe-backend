#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Item, NewItem};

pub fn add_item<'a>(conn: &PgConnection, new_item: &NewItem) -> Item {
                         // description: &'a Option<&str>
    use schema::items;

    diesel::insert_into(items::table)
        .values(new_item)
        .get_result(conn)
        .expect("Error saving new post")
        // TODO: use .update
}