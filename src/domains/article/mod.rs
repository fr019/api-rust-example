use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub short_description: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    pub title: String,
    pub short_description: String,
    pub content: String,
}
