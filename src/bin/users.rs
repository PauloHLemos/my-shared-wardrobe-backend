extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use drp02_backend::models::{User, NewUser, NewUserData, NewUserAuth};
use self::diesel::prelude::*;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn main() {}

pub fn insert_user(new_user: &NewUser) -> User {
    let connection = establish_connection();

    use schema::users;

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&connection)
        .expect("Error saving new user")
        // TODO: use .update
}

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

pub fn create_user(data: &NewUserData) -> User {
    
    let new_user: NewUser = NewUser {
        name: data.name.clone()
    };

    let connection = establish_connection();

    use schema::{users, users_auth};

    // insert new user meta data
    let user_meta: User = diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&connection)
        .expect("Error creating new user");

    let password_hash = hash(&String::from(data.password));
    let auth_info: NewUserAuth = NewUserAuth {
        uid: user_meta.uid,
        password_hash: password_hash
    };

    // insert new user authentication data
    diesel::insert_into(users_auth::table)
        .values(auth_info)
        .get_result(&connection)
        .expect("Error inserting user authentication data")
    
}


fn hash(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

// fn hash(password: &String) -> String {
//     let mut hasher = Sha3::sha3_256();
//     hasher.input_str(password);
//     hasher.result_str()
// }