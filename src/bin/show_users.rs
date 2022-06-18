extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use self::models::*;
use self::diesel::prelude::*;

pub fn main() {}

pub fn show_users() {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
    }
}

pub fn get_users() -> Vec<User> {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    return results;
}