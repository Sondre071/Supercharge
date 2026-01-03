use crate::blobstorage;
use crate::terminal;

use blobstorage::types::LocalFile;
use terminal::COLORS;

use base64::{Engine as _, engine::general_purpose};
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::fs;
use std::io::{self, Read, Write};
use std::time::SystemTime;

pub fn put_chunked_blob(url: &str, file: &LocalFile, file_size: usize) {
    let block_size: usize = 4 * 1024 * 1024;

    let total_chunks = (file_size + block_size - 1) / block_size;

    let client = reqwest::blocking::Client::new();

    let f = fs::File::open(file.path.clone()).expect("Failed to open file.");
    let mut reader = io::BufReader::new(f);

    let mut buffer = vec![0u8; block_size];
    let mut block_ids: Vec<String> = Vec::new();
    let mut block_index: u32 = 0;

    loop {
        let n = reader.read(&mut buffer).expect("Failed to read file.");
        if n == 0 {
            break;
        }

        print!(
            "\r{}Uploading {}{}{} ({} kb){}. Chunk {}/{}",
            COLORS.Yellow,
            COLORS.White,
            file.name,
            COLORS.Gray,
            file.content_length / 1024,
            COLORS.Yellow,
            block_index + 1,
            total_chunks,
        );

        io::stdout().flush().unwrap();

        upload_block(&client, url, &buffer[..n], block_index);

        let raw_id = format!("block-{:08}", block_index);
        let block_id_b64 = general_purpose::STANDARD.encode(raw_id.as_bytes());
        block_ids.push(block_id_b64);

        block_index += 1;
    }

    commit_blocks(&client, url, &block_ids);

    println!();
}

fn upload_block(client: &Client, base_url: &str, data: &[u8], block_index: u32) {
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
        .expect("Failed to upload chunk.");

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp
            .text()
            .unwrap_or_else(|_| "Unable to read body".to_string());
        panic!(
            "Failed to upload chunk ({}): {} {}",
            block_index, status, body
        );
    }
}

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
        .expect("Failed to commit blob chunks..");

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp
            .text()
            .unwrap_or_else(|_| "Unable to read body".to_string());
        panic!("Failed to commit blob chunks: {} {}", status, body);
    }
}
