#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod bin;

use rocket::data::ToByteUnit;
use rocket::fairing::AdHoc;
// use rocket::Request;
use rocket::serde::json::{Json, json};
use rocket::{get, post, form::Form, routes};
use rocket_auth::{Users, Error, Auth, Signup, Login};


use drp02_backend::models::{Item, NewItem, NewUserData};
use bin::items::{get_items, insert_item, insert_item_plain, delete_item};
use crate::bin::users::{get_users, create_user};
use bin::likes::{like_item, unlike_item};
use aws_sdk_s3::{Client, Error as AWSError};
use rocket::Data;
use rocket::response::status::NotFound;
use rocket::State;

const BUCKET_NAME: &str = "drpbucket"; 

#[get("/")]
fn index() -> &'static str {
  "<Hello WORLD>"
}

// ---------------------------------- items ----------------------------------

#[get("/wardrobe")]
fn wardrobe() -> Json<Vec<Item>> {
    get_items().into()
}

#[get("/wardrobe_plain")]
fn wardrobe_plain() -> String {
    let mut items_str = String::new();

        for item in get_items() {
            items_str.push_str("Item no: ");
            items_str.push_str(&item.id.to_string());
            items_str.push_str(":");
            // TODO: add belonging to uid...
            items_str.push_str(&item.name);
            items_str.push_str(", ");
            if item.description.is_some() {
                items_str.push_str(&item.description.expect("no description"));
                items_str.push_str(", ");
            }
            if item.tags.is_some() {
                items_str.push_str("tags: ");
                let tags: Vec<String> = item.tags.expect("no tags");
                let tags_str: String  = tags.iter().map( |id| id.to_string() + ",").collect(); 
                items_str.push_str(&tags_str);
            }
            items_str.push_str("\n");
        }
    return items_str;
}

#[post("/insert_item", format = "json", data = "<item>")]
fn new_item(item: Json<NewItem>) {
    insert_item(&item);
}

#[get("/insert_item_plain/<type_>/<name>")]
fn new_item_plain(type_: &str, name: &str) {
    insert_item_plain(type_, name);
}

#[get("/delete_item/<item_id>")]
fn delete_item_req(item_id: i64) {
    delete_item(item_id);
}

#[get("/like_item/<item_id>")]
fn like_item_req(item_id: i64) {
    like_item(item_id);
}

#[get("/unlike_item/<item_id>")]
fn unlike_item_req(item_id: i64) {
    unlike_item(item_id);
}

// ------------------------------ user session ---------------------------------------

#[post("/signup", format="json", data="<data>")]
fn signup(data: Json<NewUserData>) {
    create_user(&data);
}

// #[post("/login", data="<form>")]
// async fn login(form: rocket::serde::json::Json<Login>, auth: Auth<'_>) -> Result<&'static str, Error> {
//     auth.login(&form).await?;
//     Ok("You're logged in.")
// }

// #[get("/logout")]
// fn logout(auth: Auth<'_>) {
//     auth.logout();
// }

// #[get("/see-user/<id>")]
// async fn see_user(id: i32, users: &State<Users>) -> String {
//     let user = users.get_by_id(id).await.unwrap();
//     format!("{}", json!(user))
// }

// #[post("/signup", data="<form>")]
// async fn signup(form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
//     auth.signup(&form).await?;
//     auth.login(&form.into());
//     Ok("You signed up.")
// }

// #[post("/login", data="<form>")]
// async fn login(form: rocket::serde::json::Json<Login>, auth: Auth<'_>) -> Result<&'static str, Error> {
//     auth.login(&form).await?;
//     Ok("You're logged in.")
// }

// #[get("/logout")]
// fn logout(auth: Auth<'_>) {
//     auth.logout();
// }

// #[get("/see-user/<id>")]
// async fn see_user(id: i32, users: &State<Users>) -> String {
//     let user = users.get_by_id(id).await.unwrap();
//     format!("{}", json!(user))
// }

// --------------------------------------------------------------------------------------

pub async fn upload_object(
    client: &Client,
    data: Vec<u8>,
    key: &str,
) -> Result<(), AWSError> {
    client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .body(data.into())
        .send()
        .await?;

    println!("Uploaded file: {}", key);
    Ok(())
}

#[post("/image/<post_id>", format = "image/jpeg", data = "<data>")]
async fn set_post_image(
    post_id: &str,
    data: Data<'_>,
    client: &State<Client>,
) -> Result<String, NotFound<String>> {
    let path = format!("tmp/{}.jpeg", post_id);

    let data = data
        .open(2u32.mebibytes())
        .into_bytes()
        .await
        .map_err(|e| NotFound(e.to_string()))?;

    image::load_from_memory(data.as_slice()).map_err(|e| NotFound(e.to_string()))?;

    upload_object(  
        client,
        data.value,
        &format!("{}.jpeg", post_id),
    )
    .await
    .map_err(|e| NotFound(e.to_string()))?;

    Ok(path)
}

async fn initialize_variables() -> Client{
    let config = aws_config::from_env().load().await;
    let client = Client::new(&config);
    return client;
}


#[launch]
async fn rocket() -> _ {
    
    let users = get_users();
    rocket::build()
            .mount("/", routes![index, 
                                wardrobe, wardrobe_plain,
                                new_item_plain, new_item,
                                delete_item_req,
                                like_item_req, unlike_item_req,
                                set_post_image,
                                //signup, login, logout, see_user
                                signup])
            .manage(users)
            .attach(AdHoc::on_ignite("Liftoff Message", |r| {
                Box::pin(async move {
                    let client = initialize_variables().await;
                    r.manage(client)
                })
            }))
}