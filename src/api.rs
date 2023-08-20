use crate::AssetType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    slug: String,
    title: String,
    description: String,
    versions: Vec<String>,
    dependencies: Option<Vec<String>>
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    hits: Vec<Project>,
    offset: i32,
    limit: u32,
    total_hits: u32
}

pub async fn find(asset_type: AssetType, version: String, name: String) -> Vec<Project> {
    let query = format!(
        "https://api.modrinth.com/v2/search?query={}?facets=[[versions:{}],[project_type:{}]]",
        name,
        version,
        asset_type.to_string()
    );

    let raw_body = reqwest::get(query).await.unwrap().text().await.unwrap();

    let body: SearchResponse = serde_json::from_str(&raw_body[..]).unwrap();

    body.hits
}
