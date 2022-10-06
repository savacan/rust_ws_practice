use anyhow::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Executor, MySql};

#[derive(Clone, Serialize)]
pub struct TestTable {
    pub id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(PartialEq, Debug, Default)]
pub struct TestTableInput {
    pub name: String,
}

impl TestTable {
    pub async fn insert<'a, E>(conn: E, input: &TestTableInput) -> Result<i64>
    where
        E: Executor<'a, Database = MySql>,
    {
        let id = sqlx::query!(
            r#"
            INSERT INTO test_table (
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

    pub async fn find_by_id<'a, E>(conn: E, id: i64) -> Result<Option<TestTable>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let test = sqlx::query_as!(TestTable, r#"SELECT * FROM test_table WHERE id = ?"#, id)
            .fetch_optional(conn)
            .await?;
        Ok(test)
    }

    pub async fn find_all<'a, E>(conn: E) -> Result<Vec<TestTable>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let all = sqlx::query_as!(TestTable, r#"SELECT * FROM test_table"#)
            .fetch_all(conn)
            .await?;
        Ok(all)
    }
}
