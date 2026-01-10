use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::time::SystemTime;

pub fn delete_blob(url: &str) {
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

    let client = reqwest::blocking::Client::new();

    let response = client
        .delete(url)
        .headers(headers)
        .send()
        .expect("Failed to delete blob.");

    let status = response.status();

    if !status.is_success() {
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to parse response body."));

        panic!("Non-successful HTTP status: {}, {}", status, body_text)
    }
}
