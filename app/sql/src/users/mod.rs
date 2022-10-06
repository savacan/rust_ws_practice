use anyhow::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Executor, FromRow, MySql};

#[derive(Clone, Serialize, Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub lessor_id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(PartialEq, Debug, Default)]
pub struct UserInput {
    pub name: String,
}

impl User {
    pub async fn insert<'a, E>(conn: E, input: &UserInput) -> Result<i64>
    where
        E: Executor<'a, Database = MySql>,
    {
        let id = sqlx::query!(
            r#"
            INSERT INTO users (
                name
            ) VALUES (?);"#,
            input.name
        )
        .execute(conn)
        .await?
        .last_insert_id()
        .try_into()?;

        Ok(id)
    }

    pub async fn find_by_id<'a, E>(conn: E, id: i64) -> Result<Option<User>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let test = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = ?"#, id)
            .fetch_optional(conn)
            .await?;
        Ok(test)
    }

    pub async fn find_by_ids<'a, E>(conn: E, ids: &[i64]) -> Result<Vec<User>>
    where
        E: Executor<'a, Database = MySql>,
    {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let ids = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let query = format!(
            r#"
        SELECT * FROM users WHERE id IN ({ids})
        "#
        );
        let users = sqlx::query_as(&query).fetch_all(conn).await?;
        Ok(users)
    }

    pub async fn find_by_lessor_id<'a, E>(conn: E, lessor_id: i64) -> Result<Vec<User>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let lessor_users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE lessor_id = ?
            "#,
            lessor_id
        )
        .fetch_all(conn)
        .await?;
        Ok(lessor_users)
    }

    pub async fn find_all<'a, E>(conn: E) -> Result<Vec<User>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let all = sqlx::query_as!(User, r#"SELECT * FROM users"#)
            .fetch_all(conn)
            .await?;
        Ok(all)
    }
}
