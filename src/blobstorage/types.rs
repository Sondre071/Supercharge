use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename = "EnumerationResults")]
pub struct BlobEnumerationResults {
    #[serde(rename = "@ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "@ContainerName")]
    pub container_name: String,
    #[serde(rename = "Prefix", default)]
    pub prefix: String,
    #[serde(rename = "Marker", default)]
    pub marker: String,
    #[serde(rename = "MaxResults", default)]
    pub max_results: i32,
    #[serde(rename = "Blobs", default)]
    pub blobs: BlobsList,
    #[serde(rename = "NextMarker", default)]
    pub next_marker: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct BlobsList {
    #[serde(rename = "Blob", default)]
    pub blob: Vec<Blob>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Blob {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Snapshot", default)]
    pub snapshot: String,
    #[serde(rename = "VersionId", default)]
    pub version_id: String,
    #[serde(rename = "Properties")]
    pub properties: BlobProps,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BlobProps {
    #[serde(rename = "Content-Length")]
    pub content_length: i64,
    #[serde(rename = "Content-MD5", default)]
    pub content_md5: String,
    #[serde(rename = "Last-Modified")]
    pub last_modified: String,
    #[serde(rename = "BlobType")]
    pub blob_type: String,
    #[serde(rename = "Creation-Time", default)]
    pub creation_time: String,
}
