use crate::api;
use actix_web::web;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(api::image::index)
            .service(api::image::upload)
            .service(api::image::export),
    );
}
