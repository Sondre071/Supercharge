use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename = "EnumerationResults")]
pub struct ContainerEnumerationResults {
    #[serde(rename = "@ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "MaxResults", default)]
    pub max_results: i32,
    #[serde(rename = "Containers", default)]
    pub containers: ContainersList,
    #[serde(rename = "NextMarker", default)]
    pub next_marker: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct ContainersList {
    #[serde(rename = "Container", default)]
    pub container: Vec<Container>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Container {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: ContainerProps,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ContainerProps {
    #[serde(rename = "Last-Modified")]
    pub last_modified: String,
    #[serde(rename = "Etag")]
    pub etag: String,
    #[serde(rename = "PublicAccess", default)]
    pub public_access: String,
}
