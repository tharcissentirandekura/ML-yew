use actix_multipart::Multipart;
use sanitize_filename;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,middleware::Logger};
use futures_util::stream::StreamExt as _;
use std::io::Write;
use actix_files::NamedFile;
use std::path::PathBuf;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse> {
    let mut message = String::from("No file uploaded");
    
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let filename = field.content_disposition().and_then(|cd| {
            cd.get_filename().map(|name| name.to_string())
        }).unwrap();

        let final_file = filename.replace(" ", "_");

        let sanitized_file = sanitize_filename::sanitize(&final_file);
        
        let file = format!("http://127.0.0.1:8080/view/{sanitized_file}");
        message = file;
        tokio::fs::create_dir_all("./uploads").await?;
        let filepath = format!("./uploads/{}", sanitized_file); // create the file path
        
        let mut f = web::block(|| std::fs::File::create(filepath)).await??; // create the file

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
        
    }
    println!("{}", message);
    Ok(HttpResponse::Ok().json(message))
}

// #[get("/")]
/**
 * A route that sends at home
 */
async fn home() -> HttpResponse {
    // open the uploads folder
    // read the file and create a link to 127.0.0.1::8080/view/{filename}
    // img src={file}
    //done.
    HttpResponse::Ok().json("hello")
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./uploads")?;
    // Set environment variables for logging
    std::env::set_var("RUST_LOG", "debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
   
    let server = "127.0.0.1:8080";
    println!("Server running at http://{}",server);
    
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger)
            .route("/upload", web::post().to(save_file))
            .route("/",web::get().to(home))
            .route("/view/{filename}", web::get().to(view_file))
            .service(actix_files::Files::new("/uploads", "./uploads").show_files_listing())
    })
    .bind(server)?
    .run()
    .await
}
