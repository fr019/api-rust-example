use serde::Deserialize;

pub mod article;
pub mod user;

#[derive(Deserialize)]
pub struct CollectionParams {
    #[serde(default = "default_per_page")]
    pub per_page: usize,
    #[serde(default)]
    pub order: Option<String>,
    pub last_id: Option<usize>,
}

fn default_per_page() -> usize {
    10
}
