use super::db::connect_db;
use actix_multipart::Multipart;
use futures_util::{stream::StreamExt, TryStreamExt};
use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    HttpResponse, Responder,
};
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::path::PathBuf;



#[derive(Debug,Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub path: String, // path to the image file on the file system
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
    <form action='http://127.0.0.1:8080/upload' method='post' enctype='multipart/form-data'>
        <input type='file' name='file' />
        <input type='submit' value='Upload' />
    </form>
    


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
    let db = connect_db().await.unwrap();
    let collection: Collection<Image> = db.collection("images");
    let filter = doc! {}; // Retrieve all images
    let cursor = collection.find(filter).await.unwrap();
    let images: Vec<Image> = cursor.try_collect().await.unwrap();

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
async fn upload_image(mut payload: Multipart) -> Result<HttpResponse, actix_web::error::Error> {
    let db = connect_db().await.unwrap();
    let mut image_name = String::new();
    let upload_dir = "./uploads/"; // Directory to save uploaded files

    tokio::fs::create_dir_all(upload_dir).await?; // Create the directory if it doesn't exist

    // Process the multipart payload
    while let Some(field) = payload.next().await {
        let mut field = match field {
            Ok(field) => field,
            Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string())),
        };

        // Check if this is the file field
        if field.name() == Some("file") {
            image_name = field
                .content_disposition()
                .and_then(|cd| cd.get_filename())
                .unwrap_or("image")
                .to_string();
            
            let file_path = PathBuf::from(format!("{}{}", upload_dir, image_name));
            println!("File path: {:#?}", file_path);

            // Create a file at the specified path and write the field data to it
            let mut file = tokio::fs::File::create(&file_path).await?;
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string())),
                };
                file.write_all(&chunk).await?;
            }

            // Save the file path in MongoDB instead of the binary data
            let image = Image {
                id: 1, // This should be dynamically generated in a production environment
                name: image_name.clone(),
                path: file_path.to_string_lossy().to_string(), // Convert the path to a String
            };

            // Store the image metadata (name and path) in MongoDB
            println!("Image saved successfully,{:#?}", image);
            match save_image(&image, &db).await {
                
                Ok(_) => return Ok(HttpResponse::Ok().json("Image saved successfully")),
                Err(e) => {
                    eprintln!("Failed to upload image: {}", e);
                    return Ok(HttpResponse::InternalServerError().finish());
                }
            }
        }
    }

    Ok(HttpResponse::BadRequest().body("No file found in request"))
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    //initialize the routes
    cfg.service(get_image); //get image route
    cfg.service(upload_image); //upload image route
    cfg.service(home); //home route
    //  cfg.route("/upload", web::route().to(upload_image));
}
