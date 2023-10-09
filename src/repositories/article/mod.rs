use sqlx::PgExecutor;

use crate::domains::article::{Article, CreateArticle};

impl Article {
    pub async fn create(executor: impl PgExecutor<'_>, new_article: CreateArticle) -> Article {
        let sql = format!(
            r#"
                insert into articles (title, short_description, content)
                    values ({title}, {short_description}, {content})
                returning id, title, short_description, content;
            "#,
            title = new_article.title,
            short_description = new_article.short_description,
            content = new_article.content
        );

        sqlx::query_as(&sql)
            .fetch_one(executor)
            .await
            .expect("Failed to create article")
    }

    pub async fn get_all(executor: impl PgExecutor<'_>) -> Vec<Article> {
        let sql = r#"
                select 
                    id
                    ,title
                    ,content
                    ,short_description
                    ,is_deleted
                from articles;
            "#;

        sqlx::query_as(sql)
            .fetch_all(executor)
            .await
            .expect("Failed to fetch articles")
    }

    pub async fn delete(executor: impl PgExecutor<'_>, article_id: i32) -> Article {
        let sql = format!(
            r#"
                update articles
                    set is_deleted = true
                where id = {article_id}
                returning id, title, content, short_description;
            "#
        );

        sqlx::query_as(&sql)
            .fetch_one(executor)
            .await
            .expect("Failed to delete article")
    }
}
