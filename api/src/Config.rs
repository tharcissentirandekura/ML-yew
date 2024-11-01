use actix_web::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(|| async { "Hello, Actix!" }));
}