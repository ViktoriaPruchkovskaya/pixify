use actix_multipart::{Field, MultipartError};
use actix_web::http;
use futures_util::StreamExt;

pub async fn get_bytes(mut field: Field) -> Result<Vec<u8>, MultipartError> {
    let mut bytes: Vec<u8> = vec![];
    while let Some(chunk) = field.next().await {
        let mut vec = chunk?.clone().to_vec();
        bytes.append(&mut vec);
    }
    Ok(bytes)
}

pub fn build(field: &str, file_content: &Vec<u8>) -> ((http::header::HeaderName, String), Vec<u8>) {
    const BOUNDARY: &str = "12345";
    let mut payload: Vec<u8> = format!(
        "--{BOUNDARY}\r\n\
        Content-Disposition: form-data; name=\"{field}\"; filename=\"pic.png\"\r\n\
        Content-Type: image/png\r\n\
        Content-Length: {}\r\n\r\n\
        ",
        file_content.len()
    )
    .as_bytes()
    .to_vec();
    payload.extend(file_content);
    payload.extend(
        format!(
            "\r\n\
        --{BOUNDARY}--\r\n"
        )
        .as_bytes(),
    );
    let header: (http::header::HeaderName, String) = (
        http::header::CONTENT_TYPE,
        format!("multipart/form-data; boundary={BOUNDARY}"),
    );
    (header, payload)
}
