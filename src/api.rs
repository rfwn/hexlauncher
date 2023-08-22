use nimb::{Asset, AssetType};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    hits: Vec<Asset>,
    offset: i32,
    limit: u32,
    total_hits: u32
}

pub async fn find(asset_type: AssetType, version: String, name: String) -> Vec<Asset> {
    let project_type = match asset_type {
        AssetType::Shader => "shader",
        AssetType::ResourcePack => "resourcepack",
        _ => "mod"
    };

    let query = format!(
        "https://api.modrinth.com/v2/search?query={}&facets=[{}[\"versions:{}\"],[\"project_type:{}\"]]",
        name,
        if project_type == "mod" {
            format!(
                "[\"categories:{}\"],",
                match asset_type {
                    AssetType::QuiltMod => "quilt",
                    AssetType::FabricMod => "fabric",
                    AssetType::ForgeMod => "forge",
                    AssetType::DataPack => "datapack",
                    AssetType::Plugin => "plugin",
                    _ => ""
                }
            )
        } else { "".to_string() },
        version,
        project_type
    );

    let raw_body = reqwest::get(query).await.unwrap().text().await.unwrap();

    let body: SearchResponse = serde_json::from_str(&raw_body[..]).unwrap();

    body.hits
}
