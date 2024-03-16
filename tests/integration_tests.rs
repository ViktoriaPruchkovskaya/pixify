mod tests {
    use actix_web::{test, App, http};
    use bytes::Bytes;
    use pixify::api::routes;

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
        let pic = include_bytes!("pic.png").to_vec();
        let app = test::init_service(App::new().configure(routes::services)).await;

        let (header, payload) = build_multipart("file", &pic);
        let req = test::TestRequest::post().uri("/api/upload")
            .insert_header(header).set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.headers().get("content-type").unwrap(), "image/png");
        let body = test::read_body(resp).await;
        assert!(!body.is_empty());
    }

    #[actix_web::test]
    async fn it_uploads_image_in_wrong_field() {
        let pic = include_bytes!("pic.png").to_vec();
        let app = test::init_service(App::new().configure(routes::services)).await;

        let (header, payload) = build_multipart("wrong_field", &pic);
        let req = test::TestRequest::post().uri("/api/upload")
            .insert_header(header).set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"\"Invalid payload. Expected 'file'\""));
    }

    fn build_multipart(field: &str, file_content: &Vec<u8>) -> ((http::header::HeaderName, String), Vec<u8>) {
        const BOUNDARY: &str = "12345";
        let mut payload: Vec<u8> = format!("--{BOUNDARY}\r\n\
        Content-Disposition: form-data; name=\"{field}\"; filename=\"pic.png\"\r\n\
        Content-Type: image/png\r\n\
        Content-Length: {}\r\n\r\n\
        ", file_content.len()).as_bytes().to_vec();
        payload.extend(file_content);
        payload.extend(format!("\r\n\
        --{BOUNDARY}--\r\n").as_bytes());
        let header: (http::header::HeaderName, String) = (http::header::CONTENT_TYPE, format!("multipart/form-data; boundary={BOUNDARY}"));
        (header, payload)
    }
}
