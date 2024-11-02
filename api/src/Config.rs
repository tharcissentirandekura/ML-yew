use actix_web::web;
use crate::handlers::{home};
pub fn config_app(cfg: &mut web::ServiceConfig) {

    cfg.route("/", web::get().to(|| async { "Hello, Actix!" }));
}