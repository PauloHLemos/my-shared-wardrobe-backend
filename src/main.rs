#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod bin;

// use rocket::Request;
use rocket::serde::json::Json;
use rocket_auth::{Users, Error, Auth, Signup, Login};


use drp02_backend::models::{Item, NewItem};
use bin::show_items::get_items;
use bin::insert_item::{insert_item, insert_item_plain};
use bin::delete_item::delete_item;

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

// ------------------------------ user session ---------------------------------------

#[post("/signup", data="
")] 
fn signup(form: Form, mut auth: Auth) {
   auth.signup(&form);
}

#[post("/login", data="")] 
fn login(form: Form, mut auth: Auth) {
   auth.login(&form);
}

#[get("/logout")] 
fn logout(mut auth: Auth) {
   auth.logout();
}

// --------------------------------------------------------------------------------------

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,wardrobe,new_item_plain,wardrobe_plain,
            new_item,delete_item_req])
}