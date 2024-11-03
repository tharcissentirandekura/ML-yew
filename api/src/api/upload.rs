use super::db::connect_db;

use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    HttpResponse, Responder,
};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub data: Vec<u8>, // image data is stored as a byte array
}

#[get("/image")]
pub async fn get_image() -> impl Responder {
    HttpResponse::Ok().body("Image")
}

#[derive(Deserialize)]
pub struct UploadImage {
    pub name: String,
    pub data: Vec<u8>,
}
async fn save_image(image: &Image, db: &Database) -> mongodb::error::Result<()> {
    let collection: Collection<Image> = db.collection("images");
    collection.insert_one(image).await?;
    Ok(())
}
#[post("/upload")]
// we use Json<UploadImage> to automatically deserialize the request body into the UploadImage struct
async fn upload_image(image: Json<UploadImage>) -> impl Responder {
    let db = connect_db().await.unwrap(); // Connect to the database first of all

    let image = Image {
        //define the image struct object
        id: 1,
        name: image.name.clone(),
        data: image.data.clone(),
    };

    match save_image(&image, &db).await {
        //save the image to the database
        Ok(_) => HttpResponse::Ok().json("Image uploaded successfully"),
        Err(e) => {
            eprintln!("Failed to upload image: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn init_routes(cfg: &mut ServiceConfig) {
    //initialize the routes
    cfg.service(get_image); //get image route
    cfg.service(upload_image); //upload image route
}
