#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

pub mod bin;

use rocket::data::ToByteUnit;
use rocket::fairing::AdHoc;
use rocket::{get, post, routes};
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
    
    rocket::build()
            .mount("/", routes![index, set_post_image])
            .mount("/user", bin::users::routes())
            .mount("/items", bin::items::routes())
            .mount("/likes", bin::likes::routes())
            .attach(AdHoc::on_ignite("Liftoff Message", |r| {
                Box::pin(async move {
                    let client = initialize_variables().await;
                    r.manage(client)
                })
            }))
}