use crate::domains::user::{CreateUser, User};
use crate::handlers::user::UserError;
use sqlx::PgExecutor;

impl User {
    pub async fn create(db: impl PgExecutor<'_>, new_user: CreateUser) -> Result<User, UserError> {
        sqlx::query_as!(
            User,
            "
                insert into users (name, email)
                values ($1, $2)
                returning id, name, email
            ",
            new_user.name,  // $1
            new_user.email  // $2
        )
        .fetch_one(db)
        .await
        .map_err(|_| UserError::Unknown)
    }

    pub async fn get(db: impl PgExecutor<'_>, user_id: i32) -> Result<User, UserError> {
        sqlx::query_as!(
            User,
            "
                select id, name, email
                from users
                where id = $1
            ",
            user_id // $1
        )
        .fetch_one(db)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => UserError::NotFound(user_id),
            _ => UserError::Unknown,
        })
    }

    pub async fn get_all(
        db: impl PgExecutor<'_>,
        per_page: i32,
        last_id: Option<i32>,
        order: Option<String>,
    ) -> Result<Vec<User>, UserError> {
        let order = order.unwrap_or_else(|| "ASC".to_string());
        let order_filter = format!("ORDER BY id {}", order);

        let query = format!(
            "
                SELECT id, name, email
                FROM users
                WHERE is_deleted = false
                and ($1::int is null or id > $1)
                {order_filter}
                LIMIT $2
            "
        );

        sqlx::query_as(&query)
            .bind(last_id) // $1
            .bind(per_page) // $2
            .fetch_all(db)
            .await
            .map_err(|_| UserError::Unknown)
    }

    pub async fn delete(db: impl PgExecutor<'_>, user_id: i32) -> Result<User, UserError> {
        sqlx::query_as!(
            User,
            "
            update users
            set is_deleted = true
            where id = $1
            returning id, name, email
        ",
            user_id // $1
        )
        .fetch_one(db)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => UserError::NotFound(user_id),
            _ => UserError::Unknown,
        })
    }
}
