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

#[derive(Debug)]
pub struct MultipartBuilder<'a> {
    files: Vec<(String, String, &'a Vec<u8>)>,
    texts: Vec<(String, String)>,
}

impl<'a> MultipartBuilder<'a> {
    pub fn new() -> Self {
        MultipartBuilder {
            files: vec![],
            texts: vec![],
        }
    }

    pub fn add_file(&mut self, field: &str, filename: &str, file_content: &'a Vec<u8>) {
        self.files
            .push((field.into(), filename.into(), file_content));
    }

    pub fn add_text(&mut self, field: &str, text: impl ToString) {
        self.texts.push((field.into(), text.to_string()))
    }

    pub fn build(self) -> ((http::header::HeaderName, String), Vec<u8>) {
        const BOUNDARY: &str = "12345";
        let mut payload: Vec<u8> = vec![];
        for (field, filename, file_content) in self.files {
            payload.extend(
                format!(
                    "--{BOUNDARY}\r\n\
        Content-Disposition: form-data; name=\"{field}\"; filename=\"{filename}\"\r\n\
        Content-Type: image/png\r\n\
        Content-Length: {}\r\n\r\n\
        ",
                    file_content.len()
                )
                .as_bytes()
                .to_vec(),
            );
            payload.extend(file_content);
            payload.extend("\r\n".as_bytes());
        }

        for (field, content) in self.texts {
            payload.extend(
                format!(
                    "--{BOUNDARY}\r\n\
        Content-Disposition: form-data; name=\"{field}\"\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\r\n\
        ",
                    content.len()
                )
                .as_bytes()
                .to_vec(),
            );
            payload.extend(content.as_bytes());
            payload.extend("\r\n".as_bytes());
        }

        payload.extend(format!("--{BOUNDARY}--\r\n").as_bytes());

        let header: (http::header::HeaderName, String) = (
            http::header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        );
        (header, payload)
    }
}

impl<'a> Default for MultipartBuilder<'a> {
    fn default() -> Self {
        MultipartBuilder::new()
    }
}
