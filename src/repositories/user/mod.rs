use crate::domains::user::{CreateUser, User};
use crate::handlers::user::UserError;
use sqlx::PgExecutor;

impl User {
    pub async fn create(executor: impl PgExecutor<'_>, new_user: CreateUser) -> Result<User, UserError> {
        let sql = format!(
            "
                insert into users (name, email)
                    values ('{name}', '{email}')
                returning id, name, email;
            ",
            name = new_user.name,
            email = new_user.email
        );

        sqlx::query_as(&sql)
            .fetch_one(executor)
            .await
            .map_err(|err| match err {
                _ => UserError::Unknown,
            })
    }

    pub async fn get(executor: impl PgExecutor<'_>, user_id: i32) -> Result<User, UserError> {
        let sql = format!(
            "
                select id
                    ,name
                    ,email
                    ,is_deleted 
                from users
                where id = {user_id};
            "
        );

        sqlx::query_as(&sql)
            .fetch_one(executor)
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
        let order_filter = format!("order by id {}", order.unwrap_or_default());
        let last_id_filter = match last_id {
            Some(id) => format!(" and id > {id}"),
            None => "".to_string(),
        };

        let sql = format!(
            "
                select
                    id
                    ,name
                    ,email
                    ,is_deleted
                from users
                where not is_deleted
                {last_id_filter}
                {order_filter}
                limit {per_page};
            "
        );

        sqlx::query_as(&sql)
            .fetch_all(db)
            .await
            .map_err(|err| match err {
                _ => UserError::Unknown,
            })
    }

    pub async fn delete(db: impl PgExecutor<'_>, user_id: i32) -> Result<User, UserError> {
        let sql = format!(
            "
                update users
                    set is_deleted = true
                where id = {user_id}
                returning id, name, email;
            "
        );

        sqlx::query_as(&sql)
            .fetch_one(db)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => UserError::NotFound(user_id),
                _ => UserError::Unknown,
            })
    }
}
