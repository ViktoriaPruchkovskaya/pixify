#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::routes;
    use bytes::Bytes;

    #[actix_web::test]
    async fn test() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let req = test::TestRequest::get().uri("/api/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello world!"));
    }
}