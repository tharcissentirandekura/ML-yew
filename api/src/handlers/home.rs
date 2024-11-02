use actix_web::{web, Error, HttpResponse};
use crate::common::Image;

pub async fn get_image(_query: web::Query<Option<Image>>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
pub async fn upload_image(_new_product: web::Json<Image>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

pub async fn home(_query: web::Query<String>) -> Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().finish())
}