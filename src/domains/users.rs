use crate::validators::{SortBy, SortOrder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub is_deleted: bool,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    #[validate(length(min = 2, message = "Can not be empty"))]
    pub name: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct SearchQuery {
    #[serde(default)]
    pub sort_by: SortBy,
    #[serde(default)]
    pub sort_order: SortOrder,
    pub limit: Option<i32>,
    pub last_id: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    pub password: String,
}
