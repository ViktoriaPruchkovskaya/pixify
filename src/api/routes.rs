use actix_web::web;
use crate::api;

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(api::image::index).service(api::image::upload));
}