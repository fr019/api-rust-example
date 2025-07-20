use sqlx::{PgExecutor, Postgres, QueryBuilder};
use crate::domains::categories::Category;
use crate::handlers::users::UserError;
use crate::validators::{SortBy, SortOrder};

#[derive(Debug, Default)]
pub struct CategoriesRepo;

#[derive(Debug, Default)]
pub struct Params {
    pub ids: Option<Vec<i32>>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub limit: Option<i32>,
    pub last_id: Option<i32>,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
}

impl CategoriesRepo {
    pub async fn get_all(
        &self,
        executor: impl PgExecutor<'_>,
        params: Params,
    ) -> Result<Vec<Category>, UserError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            "SELECT id, name, email, password, created_at, updated_at, is_deleted FROM categories WHERE NOT is_deleted",
        );

        if let Some(ids) = params.ids {
            query_builder.push(" AND id = ANY(");
            query_builder.push_bind(ids);
            query_builder.push(")");
        }

        if let Some(ref name) = params.name {
            query_builder.push(" AND name ILIKE ");
            query_builder.push_bind(format!("%{name}%"));
        }

        if let Some(ref email) = params.email {
            query_builder.push(" AND email = ");
            query_builder.push_bind(email);
        }

        if let Some(last_id) = params.last_id {
            query_builder.push(" AND id > ");
            query_builder.push_bind(last_id);
        }

        if let Some(limit) = params.limit {
            query_builder.push(" LIMIT ");
            query_builder.push_bind(limit);
        }

        query_builder.push(format!(
            " ORDER BY {} {}",
            params.sort_by, params.sort_order
        ));

        query_builder
            .build_query_as()
            .fetch_all(executor)
            .await
            .map_err(|err| {
                tracing::error!("Failed to fetch users in UsersRepo::get_all: {:?}", err);
                UserError::Unknown
            })
    }
}
