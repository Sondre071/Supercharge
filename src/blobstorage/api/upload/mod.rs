use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use std::fs;
use std::io::{self, Read};
use std::time::SystemTime;

/// Performs a simple upload for files smaller than the threshold
pub fn simple_upload(client: &Client, url: &str, file_path: &std::path::Path) {
    let file_content = fs::read(file_path).expect("Failed to parse file content.");

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
        HeaderName::from_static("x-ms-blob-type"),
        HeaderValue::from_static("blockblob"),
    );

    println!("Uploading file in single request...");

    let response = client
        .put(url)
        .body(file_content)
        .headers(headers)
        .send()
        .expect("Failed to upload file.");

    let status = response.status();

    if !status.is_success() {
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to parse response body."));

        panic!("Non-successful HTTP status: {}, {}", status, body_text)
    }

    println!("Upload complete!");
}

/// Performs a chunked upload for larger files
pub fn chunked_upload(client: &Client, url: &str, file_path: &std::path::Path, file_size: usize) {
    let block_size: usize = 4 * 1024 * 1024;
    
    // Calculate total chunks beforehand for accurate progress reporting
    let total_chunks = (file_size + block_size - 1) / block_size;
    
    println!("Uploading file in {} chunks...", total_chunks);

    let f = fs::File::open(file_path).expect("Failed to open file.");
    let mut reader = io::BufReader::new(f);

    let mut buffer = vec![0u8; block_size];
    let mut block_ids: Vec<String> = Vec::new();
    let mut block_index: u32 = 0;

    loop {
        let n = reader.read(&mut buffer).expect("Failed to read file.");
        if n == 0 {
            break;
        }

        upload_block(client, url, &buffer[..n], block_index, total_chunks);
        
        let raw_id = format!("block-{:08}", block_index);
        let block_id_b64 = general_purpose::STANDARD.encode(raw_id.as_bytes());
        block_ids.push(block_id_b64);

        block_index += 1;
    }

    commit_blocks(client, url, &block_ids);
    
    println!("Upload complete!");
}

/// Uploads a single block as part of a chunked upload
fn upload_block(client: &Client, base_url: &str, data: &[u8], block_index: u32, total_chunks: usize) {
    let raw_id = format!("block-{:08}", block_index);
    let block_id_b64 = general_purpose::STANDARD.encode(raw_id.as_bytes());

    let put_block_url = format!(
        "{}&comp=block&blockid={}",
        base_url,
        urlencoding::encode(&block_id_b64)
    );

    let mut block_headers = HeaderMap::new();
    block_headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    block_headers.insert(
        HeaderName::from_static("x-ms-date"),
        HeaderValue::from_str(&httpdate::fmt_http_date(SystemTime::now()))
            .expect("Failed to apply timestamp."),
    );

    let resp = client
        .put(put_block_url)
        .headers(block_headers)
        .body(data.to_vec())
        .send()
        .expect("Put Block request failed.");

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp
            .text()
            .unwrap_or_else(|_| "Unable to read body".to_string());
        panic!(
            "Put Block failed (block {}): {} {}",
            block_index, status, body
        );
    }

    println!("Uploaded chunk {}/{}", block_index + 1, total_chunks);
}

/// Commits all uploaded blocks to finalize the blob
fn commit_blocks(client: &Client, base_url: &str, block_ids: &[String]) {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="utf-8"?><BlockList>"#);
    for id in block_ids {
        xml.push_str(&format!("<Latest>{}</Latest>", id));
    }
    xml.push_str("</BlockList>");

    let commit_url = format!("{}&comp=blocklist", base_url);

    let mut commit_headers = HeaderMap::new();
    commit_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
    commit_headers.insert(
        HeaderName::from_static("x-ms-date"),
        HeaderValue::from_str(&httpdate::fmt_http_date(SystemTime::now()))
            .expect("Failed to apply timestamp."),
    );

    let resp = client
        .put(commit_url)
        .headers(commit_headers)
        .body(xml)
        .send()
        .expect("Put Block List request failed.");

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp
            .text()
            .unwrap_or_else(|_| "Unable to read body".to_string());
        panic!("Put Block List failed: {} {}", status, body);
    }
}
