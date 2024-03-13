use crate::api;

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    api::routes::services(cfg);
}