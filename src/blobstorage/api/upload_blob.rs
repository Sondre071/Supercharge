use crate::blobstorage;

use blobstorage::types::LocalFile;
use blobstorage::utils::types::StorageAccount;

use std::io::Read;
use std::time::SystemTime;

use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};

use std::fs;
use std::io;

pub fn upload_blob(account: &StorageAccount, container_name: &str, file: &LocalFile) {
    let url = format!(
        "{}{}/{}?{}",
        account.blob_endpoint, container_name, file.name, account.shared_access_signature
    );

    let client = reqwest::blocking::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    let x_ms_date = httpdate::fmt_http_date(SystemTime::now());

    headers.insert(
        HeaderName::from_static("x-ms-date"),
        HeaderValue::from_str(&x_ms_date).expect("Failed to apply timestamp."),
    );

    let threshold: usize = 20 * 1024 * 1024;

    if file.content_length < threshold {
        let file_content = fs::read(&file.path).expect("Failed to parse file content.");

        headers.insert(
            HeaderName::from_static("x-ms-blob-type"),
            HeaderValue::from_static("blockblob"),
        );

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
    } else {
        let block_size: usize = 4 * 1024 * 1024;

        let f = fs::File::open(&file.path).expect("Failed to open file.");
        let mut reader = io::BufReader::new(f);

        let mut buffer = vec![0u8; block_size];
        let mut block_ids: Vec<String> = Vec::new();
        let mut block_index: u32 = 0;

        loop {
            let n = reader.read(&mut buffer).expect("Failed to read file.");
            if n == 0 {
                break;
            }

            let raw_id = format!("block-{:08}", block_index);
            let block_id_b64 = general_purpose::STANDARD.encode(raw_id.as_bytes());
            block_ids.push(block_id_b64.clone());

            let put_block_url = format!(
                "{}&comp=block&blockid={}",
                url,
                urlencoding::encode(&block_id_b64)
            );

            let mut block_headers = HeaderMap::new();
            block_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            );
            block_headers.insert(
                HeaderName::from_static("x-ms-date"),
                HeaderValue::from_str(&httpdate::fmt_http_date(SystemTime::now())).expect("Failed to apply timestamp."),
            );

            let resp = client
                .put(put_block_url)
                .headers(block_headers)
                .body(buffer[..n].to_vec())
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

            println!("Uploaded chunk {} of {}", block_index, block_ids.len());

            block_index += 1;
        }

        let mut xml = String::from(r#"<?xml version="1.0" encoding="utf-8"?><BlockList>"#);
        for id in &block_ids {
            xml.push_str(&format!("<Latest>{}</Latest>", id));
        }
        xml.push_str("</BlockList>");

        let commit_url = format!("{}&comp=blocklist", url);

        let mut commit_headers = HeaderMap::new();
        commit_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));
        commit_headers.insert(
            HeaderName::from_static("x-ms-date"),
            HeaderValue::from_str(&httpdate::fmt_http_date(SystemTime::now())).expect("Failed to apply timestamp."),
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
}
