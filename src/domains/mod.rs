use serde::Deserialize;

pub mod article;
pub mod user;

#[derive(Deserialize)]
pub struct CollectionParams {
    #[serde(default = "default_per_page")]
    pub per_page: i32,
    #[serde(default)]
    pub order: Option<String>,
    pub last_id: Option<i32>,
}

fn default_per_page() -> i32 {
    10
}
