use nimb::{Project, ProjectType, Loader};
use async_trait::async_trait;
use serde::Deserialize;

pub struct ModrinthWrapper;

#[derive(Debug, Deserialize)]
pub struct ModrinthSearchResponse {
    pub hits: Vec<Project>
}

#[async_trait]
pub trait ApiWrapper {
    async fn search_projects(query: String, version: String, project_type: ProjectType, loader: Option<Loader>) -> Vec<Project>;
}

#[async_trait]
impl ApiWrapper for ModrinthWrapper {
    async fn search_projects(query: String, version: String, project_type: ProjectType, loader: Option<Loader>) -> Vec<Project> {
        let search_url = format!(
            "https://api.modrinth.com/v2/search?query={}&facets=[{}[\"versions:{}\"],[\"project_type:{}\"]]",
            query,
            if project_type == ProjectType::Mod {
                format!("[\"categories:{}\"],", loader.unwrap().to_string())
            } else { String::new() },
            version,
            project_type
        );

        let raw_body = reqwest::get(search_url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let body: ModrinthSearchResponse = serde_json::from_str(&raw_body[..]).unwrap();

        body.hits
    }
}
