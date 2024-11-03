use super::db::connect_db;
use futures_util::stream::StreamExt;
use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    HttpResponse, Responder,
};
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub data: Vec<u8>, // image data is stored as a byte array
}


#[derive(Deserialize)]
pub struct UploadImage {
    pub name: String,
    pub data: Vec<u8>,
}
#[get("/")]
pub async  fn home() -> impl Responder {
    let data = String::from("
    <h1> ******************* Rustcv API *******************</h1>
    
    <ul>
        <li>Get Image  <a href='http://127.0.0.1:8080/image'>/image</a></li>
        <li>Upload Image  <a href='http://127.0.0.1:8080/upload'>/upload</a></li>
    </ul>


    <style>
    body {
        font-family: Arial, sans-serif;
        margin: 0;
        padding: 0;
        background-color: #f4f4f4;
    }
    li {
        font-size: 2.2em;
        margin: 10px;
    }
    </style>

    "

    

);
    HttpResponse::Ok().body(data)
}

#[get("/image")]
pub async fn get_image() -> impl Responder {
    let db = connect_db().await.unwrap(); // Connect to the database first of all
    let collection: Collection<Image> = db.collection("images"); // Get the images collection
    let filter = doc! {}; // filter all images
    let mut cursor = collection.find(filter).await.unwrap(); // Get all images from the database
    let mut images: Vec<Image> = Vec::new(); // Create a vector to store the images

    while let Some(result) = cursor.next().await { // Loop through the images
        match result {
            Ok(image) => images.push(image),
            Err(e) => {
                eprintln!("Failed to fetch image: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    if images.is_empty() {
        return HttpResponse::Ok().body("No images found");
    }
    HttpResponse::Ok().json(images)
}

// update in the future, instead of posting the image data, we will replace the existing image with a new one to avoid storing images that users delete for each classification.
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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    //initialize the routes
    cfg.service(get_image); //get image route
    cfg.service(upload_image); //upload image route
    cfg.service(home); //home route
    //  cfg.route("/upload", web::route().to(upload_image));
}
