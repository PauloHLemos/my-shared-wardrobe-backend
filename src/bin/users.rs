extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use drp02_backend::models::{User, NewUser, NewUserData, NewUserAuth, UserAuth};
use drp02_backend::auth::AuthenticatedUser;

use diesel::sql_query;
use self::diesel::prelude::*;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::form::FromForm;
use rocket::{routes, post, get};

// reference: https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
// used tutorial to implement user authentication

// struct representing a valid user
// struct AuthenticatedUser {
//     uid: i64,
// }

// data received to login user
#[derive(FromForm, Deserialize)]
struct LoginData<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

// #[derive(Debug)]
// enum LoginError {
//     InvalidData,
//     UsernameDoesNotExist,
//     WrongPassword
// }

pub fn main() {}

pub fn get_users() -> Vec<User> {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    return results;
}

pub fn get_user_by_id(id: i64) -> Option<User> {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();

    let mut user: Vec<User> 
        = users.find(id)
            .load::<User>(&connection)
            .expect("Error loading user");

    if user.len() == 0 {
        None
    } else {
        Some(user.remove(0))
    }
}

pub fn get_user_by_email(email_str: String) -> Option<User> {
    use drp02_backend::schema::users::dsl::*;

    let connection = establish_connection();

    let mut user: Vec<User> 
        = users.filter(email.eq(email_str))
            .load::<User>(&connection)
            .expect("Error loading user");

    if user.len() == 0 {
        None
    } else {
        Some(user.remove(0))
    }
}

pub fn get_user_auth_by_email(email: String) -> Option<UserAuth> {

    let some_user: Option<User> = get_user_by_email(email);

    match some_user {
        Some(user) => {
            use drp02_backend::schema::users_auth::dsl::*;

            let connection = establish_connection();
            let mut user_auth = users_auth.find(user.uid)
                .load::<UserAuth>(&connection)
                .expect("Error loading user");

            if user_auth.len() == 0 {
                None
            } else {
                Some(user_auth.remove(0))
            }
        }
        None => None
    }
}

fn hash(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

// ------------------------------ user session ---------------------------------------

// TODO: potentially introduce rturn type Json<Option<i64>>
#[post("/signup", format="json", data="<signup_info>")]
fn signup(signup_info: Json<NewUserData>, cookies: &CookieJar<'_>) {    
    let new_user: NewUser = NewUser {
        name: signup_info.name.clone(),
        email: signup_info.email.clone(),
        items_liked: Vec::new(),
        users_following: Vec::new(),
        phone_no: signup_info.phone_no.clone(),
    };

    let connection = establish_connection();

    use schema::{users, users_auth};

    // insert new user meta data
    let user_meta: User = diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&connection)
        .expect("Error creating new user");

    let password_hash = hash(&String::from(signup_info.password));
    let auth_info: NewUserAuth = NewUserAuth {
        uid: user_meta.uid,
        password_hash: password_hash
    };

    // insert new user authentication data
    let _user_auth : UserAuth = diesel::insert_into(users_auth::table)
        .values(auth_info)
        .get_result(&connection)
        .expect("Error inserting user authentication data");

    cookies.add_private(Cookie::new("uid", user_meta.uid.to_string()));
}

// TODO: potentially introduce rturn type Json<Option<i64>>
#[post("/login", format="json", data="<login_info>")]
fn login(login_info: Json<LoginData>, cookies: &CookieJar<'_>) -> String {
    let some_user_auth = get_user_auth_by_email(login_info.email.to_string());
    match some_user_auth {
        Some(auth_info) => {
            let hash = hash(&String::from(login_info.password));
            if hash == auth_info.password_hash {
                cookies.add_private(Cookie::new("uid", auth_info.uid.to_string()));
                "Authentication Succeeded".to_string()
            } else {
                "Invalid email or password.".to_string()
            }
        }
        None => "Invalid email or password".to_string()
    }
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> String {
    cookies.remove_private(Cookie::named("uid"));
    "Succesfully logged out".to_string()
}

/// Retrieve the user's ID, if any.
#[get("/user_id")]
fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.get_private("uid")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

#[get("/get_user")]
fn get_user_user( auth_user: AuthenticatedUser) -> Json<Option<User>>{
    let id = auth_user.uid;
    get_user_by_id(id).into()
}

#[get("/get_user_by_id/<id>")]
fn get_other_user_by_id( id: i64, auth_user: AuthenticatedUser) -> Json<Option<User>>{
    get_user_by_id(id).into()
}

// --------------------------------------------------------------------------------------

// TODO: cascade following users array, so that if user is deleted gets automatically 
// deleted from this array
#[get("/following")]
fn following(auth_user: AuthenticatedUser) -> Json<Vec<User>> {
    use drp02_backend::schema::users::dsl::*;
    let connection = establish_connection();

    let following_ids: Vec<i64> = users.find(auth_user.uid)
        .load::<User>(&connection)
        .expect("Error loading user")
        .remove(0)
        .users_following;

    let mut following_users: Vec<User> = Vec::new();

    for following_id in following_ids {
        let following_user: User = users.find(following_id)
            .load::<User>(&connection)
            .expect("Error loading following user")
            .remove(0);
        following_users.push(following_user);
    }
    return following_users.into()
}

#[post("/follow/<following_uid>")]
fn follow(following_uid: i64, auth_user: AuthenticatedUser) {
    // user cant follow themselves
    // TODO: check that uid is valid
    if !(following_uid == auth_user.uid) {
        let connection = establish_connection();

        let uid = auth_user.uid;
        sql_query(format!("UPDATE users SET users_following = array_append(users_following,'{following_uid}') WHERE uid = {uid}"))
            .execute(&connection)
            .expect("error adding item to list of items_liked");
    }
}

#[post("/unfollow/<unfollowing_uid>")]
fn unfollow(unfollowing_uid: i64, auth_user: AuthenticatedUser) {
    // user cant follow themselves
    // TODO: check that uid is valid
    if !(unfollowing_uid == auth_user.uid) {
        let connection = establish_connection();

        let uid = auth_user.uid;
        sql_query(format!("UPDATE users SET users_following = array_remove(users_following,'{unfollowing_uid}') WHERE uid = {uid}"))
            .execute(&connection)
            .expect("error adding item to list of items_liked");
    }
}

#[get("/findfriends")]
fn find_friends() -> Json<Vec<User>>{
    get_users().into()
}

// -------------------------------------- updates --------------------------------------
#[post("/change_name/<new_name>")]
fn change_name(new_name: String, auth_user: AuthenticatedUser) {
    use drp02_backend::schema::users::dsl::*;
    let connection = establish_connection();

    let uid_ = auth_user.uid;
    diesel::update(users.find(uid_))
        .set(name.eq(new_name))
        .execute(&connection)
        .expect("error liking image");
}

#[post("/change_email/<new_email>")]
fn change_email(new_email: String, auth_user: AuthenticatedUser) {
    use drp02_backend::schema::users::dsl::*;
    let connection = establish_connection();

    let uid_ = auth_user.uid;
    diesel::update(users.find(uid_))
        .set(email.eq(new_email))
        .execute(&connection)
        .expect("error liking image");
}

// #[post("/change_phone_no/<new_phone_no>")]
// fn change_phone_no(new_phone_no: String, auth_user: AuthenticatedUser) {
//     use drp02_backend::schema::users::dsl::*;
//     let connection = establish_connection();

//     let uid_ = auth_user.uid;
//     diesel::update(users.find(uid_))
//         .set(phone_no.eq(Some(new_phone_no)))
//         .execute(&connection)
//         .expect("error liking image");
// }



pub fn routes() -> Vec<rocket::Route> {
    routes![signup, login, logout, user_id, following, follow, unfollow,
            find_friends,get_user_user,get_other_user_by_id,
            change_name, change_email]
}