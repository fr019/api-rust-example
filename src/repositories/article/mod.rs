use sqlx::PgExecutor;

use crate::domains::article::{Article, CreateArticle};

impl Article {
    pub async fn create(db: impl PgExecutor<'_>, new_article: CreateArticle) -> Article {
        sqlx::query_as!(
            Article,
            r#"
                insert into articles (title, short_description, content)
                    values ($1, $2, $3)
                returning id, title, short_description, content;
            "#,
            new_article.title,             // $1
            new_article.short_description, // $2
            new_article.content            // $3
        )
        .fetch_one(db)
        .await
        .expect("Failed to create article")
    }

    pub async fn get_all(db: impl PgExecutor<'_>) -> Vec<Article> {
        sqlx::query_as!(
            Article,
            r#"
                select
                    id,
                    title,
                    content,
                    short_description
                from articles;
            "#
        )
        .fetch_all(db)
        .await
        .expect("Failed to fetch articles")
    }

    pub async fn delete(db: impl PgExecutor<'_>, article_id: i32) -> Article {
        sqlx::query_as!(
            Article,
            r#"
                update articles
                    set is_deleted = true
                where id = $1
                returning id, title, content, short_description;
            "#,
            article_id // $1
        )
        .fetch_one(db)
        .await
        .expect("Failed to delete article")
    }
}
