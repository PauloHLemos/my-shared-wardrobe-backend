#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod bin;

use rocket::serde::Serialize;
use rocket::Request;
use rocket::serde::json::Json;

use drp02_backend::models::Item;
use bin::show_items::get_items;
use bin::insert_item::insert_item;

#[get("/")]
fn index() -> &'static str {
  "<Hello WORLD>"
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/wardrobe")]
fn wardrobe() -> String {
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

#[get("/insert/<type_>/<name>")]
fn insert(type_: &str, name: &str) {
    let mut lastindex = 1;
    for item in get_items() {
        if lastindex < item.id {
            lastindex = item.id;
        };
    }
    insert_item(&(lastindex + 1), type_, name);
}

#[get("/users")]
fn route() -> Json<Item> {
    Item {
        id: 69,
        uid: 1,
        type_: String::from("tshirt"),
        name: String::from("old red tshirt"),
        description: Some(String::from("This is a tshirt i dont like")),
        tags: Some(vec![String::from("old")]),
        pics: vec![String::from("oldtshirt.com")],
    }.into()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,wardrobe,insert,route])
}