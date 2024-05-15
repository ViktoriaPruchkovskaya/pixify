#[derive(serde::Deserialize)]
struct CanvasResponse {
    pub embroidery: Vec<Vec<[u8; 3]>>,
    pub palette: Vec<Palette>,
}

#[derive(serde::Deserialize)]
#[allow(unused)]
struct Palette {
    pub identifier: String,
    pub color: Color,
    pub thread_length: usize,
}

#[derive(serde::Deserialize)]
#[allow(unused)]
struct Color {
    pub name: String,
    pub rgb: [u8; 3],
}

#[cfg(test)]
mod tests {
    use crate::CanvasResponse;
    use actix_web::{test, web::Bytes, App};
    use pixify::api::routes;
    use pixify::http::multipart::MultipartBuilder;

    #[actix_web::test]
    async fn it_uploads_image() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let pic = include_bytes!("pic.png").to_vec();

        let mut multipart = MultipartBuilder::new();
        multipart.add_file("file", "pic.png", &pic);
        multipart.add_text("n_colors", 5);
        multipart.add_text("n_cells_in_width", 10);
        let (header, payload) = multipart.build();
        let req = test::TestRequest::post()
            .uri("/api/upload")
            .insert_header(header)
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: CanvasResponse = test::read_body_json(resp).await;
        assert_eq!(body.palette.len(), 5);
        assert_eq!(body.embroidery[0].len(), 10); // check embroidery row length
    }

    #[actix_web::test]
    async fn it_uploads_image_in_wrong_field() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let pic = include_bytes!("pic.png").to_vec();

        let mut multipart = MultipartBuilder::new();
        multipart.add_file("wrong_field", "pic.png", &pic);
        let (header, payload) = multipart.build();
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
            Bytes::from_static(b"\"Missing value. Expected 'file' to be provided\"")
        );
    }

    #[actix_web::test]
    async fn it_uploads_image_exceeding_color_limit() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let pic = include_bytes!("pic.png").to_vec();

        let mut multipart = MultipartBuilder::new();
        multipart.add_file("file", "pic.png", &pic);
        multipart.add_text("n_colors", 300);
        let (header, payload) = multipart.build();
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
            Bytes::from_static(
                b"\"Invalid value in 'n_colors'. Value should be within 2 and 200\""
            )
        );
    }

    #[actix_web::test]
    async fn it_exports_image() {
        let app = test::init_service(App::new().configure(routes::services)).await;
        let pic = include_bytes!("pic.png").to_vec();

        let mut multipart = MultipartBuilder::new();
        multipart.add_file("file", "pic.png", &pic);
        multipart.add_text("n_colors", 5);
        multipart.add_text("n_cells_in_width", 10);
        let (header, payload) = multipart.build();
        let req = test::TestRequest::post()
            .uri("/api/export")
            .insert_header(header)
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert!(resp.headers().contains_key("content-type"));
        assert_eq!(resp.headers().get("content-type").unwrap(), "image/png");
        assert!(resp.headers().contains_key("content-disposition"));
        assert_eq!(
            resp.headers().get("content-disposition").unwrap(),
            "attachment; filename=pic.png"
        )
    }
}
