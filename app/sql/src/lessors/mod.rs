use anyhow::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Executor, FromRow, MySql};

#[derive(PartialEq, Debug, Default)]
pub struct LessorInput {
    pub name: String,
}

#[derive(Clone, Serialize)]
pub struct Lessor {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Lessor {
    pub async fn insert<'a, E>(conn: E, input: &LessorInput) -> Result<i64>
    where
        E: Executor<'a, Database = MySql>,
    {
        let id = sqlx::query!(
            r#"
            INSERT INTO lessors (
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

    pub async fn find_by_id<'a, E>(conn: E, id: i64) -> Result<Option<Lessor>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let lessor = sqlx::query_as!(RawLessor, r#"SELECT * FROM lessors WHERE id = ?"#, id)
            .map(Lessor::from)
            .fetch_optional(conn)
            .await?;
        Ok(lessor)
    }
    pub async fn find_by_ids<'a, E>(conn: E, ids: &[i64]) -> Result<Vec<Lessor>>
    where
        E: Executor<'a, Database = MySql>,
    {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let ids = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let query = format!("SELECT * FROM lessors WHERE id IN ({})", ids);
        let raw_lessors: Vec<RawLessor> = sqlx::query_as(&query).fetch_all(conn).await?;
        let lessors = raw_lessors
            .into_iter()
            .map(Lessor::from)
            .collect::<Vec<_>>();

        Ok(lessors)
    }

    pub async fn find_all<'a, E>(conn: E) -> Result<Vec<Lessor>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let lessors = sqlx::query_as!(RawLessor, r#"SELECT * FROM lessors"#)
            .map(Lessor::from)
            .fetch_all(conn)
            .await?;

        Ok(lessors)
    }
}

#[derive(FromRow)]
struct RawLessor {
    id: i64,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<RawLessor> for Lessor {
    fn from(raw: RawLessor) -> Self {
        let RawLessor {
            id,
            name,
            created_at,
            updated_at,
        } = raw;
        Self {
            id,
            name,
            created_at,
            updated_at,
        }
    }
}
