#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

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

pub mod bin;
use bin::show_items::get_items;

#[get("/wardrobe")]
fn wardrobe() -> String {
    let mut stuff_str = String::new();

        for item in get_items() {
            stuff_str.push_str(&item.name);
            stuff_str.push_str(",");
        }
    return stuff_str;
}

use bin::insert_item::insert_item;

#[get("/insert/<item>")]
fn insert(item: &str) {
    let mut lastindex = 1;
    for item in get_items() {
        if lastindex < item.id {
            lastindex = item.id;
        };
    }
    insert_item(&(lastindex + 1), item);
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,hello,wardrobe,insert])
}