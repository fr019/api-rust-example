use crate::application::errors::ApiError;
use axum::extract::{FromRequest, Query, Request};
use axum::Json;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;
use validator::Validate;

pub struct ValidJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;

        value.validate().map_err(ApiError::ValidationError)?;

        Ok(ValidJson(value))
    }
}
pub struct ValidQuery<T>(pub T);

impl<T, S> FromRequest<S> for ValidQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state).await?;

        value.validate().map_err(ApiError::ValidationError)?;

        Ok(ValidQuery(value))
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    #[default]
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortBy::Id => write!(f, "id"),
            SortBy::Name => write!(f, "name"),
            SortBy::CreatedAt => write!(f, "created_at"),
            SortBy::UpdatedAt => write!(f, "updated_at"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "ASC"),
            SortOrder::Desc => write!(f, "DESC"),
        }
    }
}
