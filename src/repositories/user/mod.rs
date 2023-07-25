use crate::domains::user::{CreateUser, User};
use sqlx::PgExecutor;

impl User {
    pub async fn create(db: impl PgExecutor<'_>, new_user: CreateUser) -> User {
        let sql = format!(
            "
                insert into users (name, email)
                    values ('{name}', '{email}')
                returning id, name, email;
            ",
            name = new_user.name.as_str(),
            email = new_user.email.as_str()
        );

        sqlx::query_as(&sql)
            .fetch_one(db)
            .await
            .expect("Failed to create user")
    }

    pub async fn get_all(
        db: impl PgExecutor<'_>,
        per_page: usize,
        last_id: Option<usize>,
        order: Option<String>,
    ) -> Vec<User> {
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
            .expect("Failed to fetch users")
    }

    pub async fn delete(db: impl PgExecutor<'_>, user_id: i32) -> User {
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
            .expect("Failed to delete user")
    }
}
