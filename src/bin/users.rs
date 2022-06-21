extern crate drp02_backend;
extern crate diesel;

use self::drp02_backend::*;
use drp02_backend::models::{User, NewUser, NewUserData, NewUserAuth, UserAuth};

use self::diesel::prelude::*;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rocket::http::{Status, Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::form::FromForm;
use rocket::{Request, routes, post, get};
use rocket::request::{FromRequest, Outcome};

// reference: https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
// used tutorial to implement user authentication

// struct representing a valid user
struct AuthenticatedUser {
    uid: i64,
}

// data received to login user
#[derive(FromForm, Deserialize)]
struct LoginData<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug)]
enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = LoginError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<AuthenticatedUser, LoginError> {
        let email = request.headers().get_one("email");
        let password = request.headers().get_one("password");

        match (email, password) {
            (Some(e), Some(p)) => {
                let user_auth = get_user_auth_by_email(e.to_string());

                match user_auth {
                    Some(auth_info) => {
                        let hash = hash(&String::from(p));
                        if hash == auth_info.password_hash {
                            Outcome::Success(AuthenticatedUser{uid: auth_info.uid})
                        }
                        else {
                            Outcome::Failure((Status::Forbidden, LoginError::WrongPassword))
                        }
                    }
                    None => Outcome::Failure((Status::NotFound, LoginError::UsernameDoesNotExist))
                }
            },
            _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData))
        }
    }
}

// ------------------------------ user session ---------------------------------------

// TODO: potentially introduce rturn type Json<Option<i64>>
#[post("/signup", format="json", data="<signup_info>")]
fn signup(signup_info: Json<NewUserData>, cookies: &CookieJar<'_>) {    
    let new_user: NewUser = NewUser {
        name: signup_info.name.clone(),
        email: signup_info.email.clone()
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

// --------------------------------------------------------------------------------------


pub fn routes() -> Vec<rocket::Route> {
    routes![signup, login, logout, user_id]
}