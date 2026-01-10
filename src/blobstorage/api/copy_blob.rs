use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::time::SystemTime;

pub fn copy_blob(source_url: &str, destination_url: String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        HeaderName::from_static("x-ms-date"),
        HeaderValue::from_str(&httpdate::fmt_http_date(SystemTime::now()))
            .expect("Failed to apply timestamp."),
    );
    headers.insert(
        HeaderName::from_static("x-ms-copy-source"),
        HeaderValue::from_str(source_url).expect("Failed to insert source file name."),
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .put(destination_url)
        .body("")
        .headers(headers)
        .send()
        .expect("Failed to copy file.");

    let status = response.status();

    if !status.is_success() {
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to parse response body."));

        panic!("Non-successful HTTP status: {}, {}", status, body_text)
    }
}
