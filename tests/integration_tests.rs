#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use bytes::Bytes;
    use pixify::{api::routes, http::multipart};

    #[actix_web::test]
    async fn it_gets_index() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let req = test::TestRequest::get().uri("/api/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello world!"));
    }

    #[actix_web::test]
    async fn it_uploads_image() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let pic = include_bytes!("pic.png").to_vec();

        let (header, payload) = multipart::build("file", &pic);
        let req = test::TestRequest::post()
            .uri("/api/upload")
            .insert_header(header)
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("content-type").unwrap(), "image/png");
        assert_eq!(
            resp.headers().get("content-disposition").unwrap(),
            "attachment; filename=pic.png"
        );
        let body = test::read_body(resp).await;
        assert!(!body.is_empty());
    }

    #[actix_web::test]
    async fn it_uploads_image_in_wrong_field() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let pic = include_bytes!("pic.png").to_vec();

        let (header, payload) = multipart::build("wrong_field", &pic);
        let req = test::TestRequest::post()
            .uri("/api/upload")
            .insert_header(header)
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
        let body = test::read_body(resp).await;
        assert_eq!(
            body,
            Bytes::from_static(b"\"Invalid payload. Expected 'file' to be provided\"")
        );
    }
}
