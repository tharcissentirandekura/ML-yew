use super::db::connect_db;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    get, post,
    web::{self, Json, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Result,
};
use futures_util::{stream::StreamExt, TryStreamExt};
use mongodb::{bson::doc, Collection, Database};
use sanitize_filename;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub path: String, // path to the image file on the file system
}

#[derive(Serialize)]
struct ClassificationResult {
    label: String,
    confidence: f32,
    path : PathBuf
}

#[derive(Deserialize)]
pub struct UploadImage {
    pub name: String,
    pub data: Vec<u8>,
}
#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().json("you are viewing the home for rustclassy website ML model")
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
async fn upload_image(mut payload: Multipart) -> Result<HttpResponse> {
    let db = connect_db().await.unwrap();
    let mut message = String::from("No file uploaded");
    std::fs::create_dir_all("./uploads")?; // Create the directory if it doesn't exist
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|name| name.to_string()))
            .unwrap();

        let final_file = filename.replace(" ", "_"); // rename the file to remove spaces

        let sanitized_file = sanitize_filename::sanitize(&final_file); // sanitize the filename to remove special characters

        let file = format!("http://127.0.0.1:8000/view/{sanitized_file}"); // create a link to the uploaded file

        message = file.clone();

        let filepath = format!("./uploads/{}", sanitized_file); // create the file path

        let mut f = web::block(|| std::fs::File::create(filepath)).await??; // create the file

        while let Some(chunk) = field.next().await {
            // write the file data to the file
            let data = chunk?; // get the chunk of data
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }

        let image = Image {
            id: 1, // This should be dynamically generated in a production environment
            name: sanitized_file.clone(),
            path: file.clone(),
        };

        match save_image(&image, &db).await {
            Ok(_) => println!("Image metadata saved to MongoDB."),
            Err(e) => {
                eprintln!("Failed to save image metadata: {}", e);
                return Ok(HttpResponse::InternalServerError().finish());
            }
        }
    }

    println!("{}", message);
    Ok(HttpResponse::Ok().json(message))
}

#[get("/view/{filename}")]
async fn view_file(req: HttpRequest) -> impl Responder {
    let folder = "./uploads";
    let file_name: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file_path = PathBuf::from(folder).join(file_name);

    if file_path.exists() {
        NamedFile::open(file_path).unwrap().into_response(&req)
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

#[get("/classify/{file_path}")]
async fn classify_image(req:HttpRequest) -> impl Responder {
    let folder = "./uploads";
    let file_name:PathBuf = req.match_info().query("file_path").parse().unwrap();
    let file_path = PathBuf::from(folder).join(file_name);

    let result = ClassificationResult {
        label: "example_label".to_string(),
        confidence: 0.95,
        path:file_path.clone(),
    };

    println!("Classified {:?}",file_path.clone());

    HttpResponse::Ok().json(result)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    //initialize the routes
    cfg.service(get_image); //get image route
    cfg.service(upload_image); //upload image route
    cfg.service(home); //home route
    cfg.service(view_file); //view file route
    cfg.service(classify_image);
                            //  cfg.route("/upload", web::route().to(upload_image));
}
