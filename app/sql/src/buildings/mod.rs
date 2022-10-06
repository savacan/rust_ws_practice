use anyhow::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Executor, FromRow, MySql};

// ユーザーが入力する情報
#[derive(Debug, Default)]
pub struct BuildingInput {
    pub lessor_id: i64,
    pub name: String,
    pub prefecture: String,
    pub ward: String,
    pub city: String,
    pub block: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct Building {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub lessor_id: i64,
    pub name: String,
    pub prefecture: String,
    pub ward: String,
    pub city: String,
    pub block: Option<String>,
}

// buildings レコードに対応する構造体
// query_as! マクロでカラムの型チェックをするのに使う
#[derive(FromRow)]
struct RawBuilding {
    id: i64,
    lessor_id: i64,
    name: String,
    prefecture: String,
    ward: String,
    city: String,
    block: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl RawBuilding {
    async fn find_by_id<'a, E>(pool: E, id: i64) -> Result<Option<RawBuilding>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let raw_building = sqlx::query_as!(
            RawBuilding,
            r#"
            SELECT
                *
            FROM buildings WHERE id = ?"#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(raw_building)
    }

    async fn find_by_lessor_id<'a, E>(pool: E, lessor_id: i64) -> Result<Vec<RawBuilding>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let raw_buildings = sqlx::query_as!(
            RawBuilding,
            r#"
            SELECT
                *
            FROM buildings WHERE lessor_id = ?"#,
            lessor_id
        )
        .fetch_all(pool)
        .await?;
        Ok(raw_buildings)
    }
    async fn find_by_ids<'a, E>(pool: E, ids: &[i64]) -> Result<Vec<RawBuilding>>
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
        let query = format!("SELECT * FROM buildings WHERE id IN ({})", ids);
        let raw_buildings: Vec<RawBuilding> = sqlx::query_as(&query).fetch_all(pool).await?;
        Ok(raw_buildings)
    }
}

impl TryFrom<RawBuilding> for Building {
    type Error = anyhow::Error;
    fn try_from(raw_building: RawBuilding) -> Result<Self> {
        let RawBuilding {
            id,
            lessor_id,
            name,
            prefecture,
            ward,
            city,
            block,
            created_at,
            updated_at,
        } = raw_building;

        Ok(Building {
            id,
            created_at,
            updated_at,
            lessor_id,
            name,
            prefecture,
            ward,
            city,
            block,
        })
    }
}

impl Building {
    // pub async fn insert<'a, E>(conn: E, building: &BuildingInput) -> Result<i64>
    // where
    //     E: Executor<'a, Database = MySql>,
    // {
    //     let BuildingInput {
    //         lessor_id,
    //         name,
    //         prefecture,
    //         ward,
    //         city,
    //         block,
    //     } = building;
    //     let building_id = sqlx::query!(
    //         r#"
    //         INSERT INTO buildings (
    //             lessor_id,
    //             name,
    //             prefecture,
    //             ward,
    //             city,
    //             block,
    //         ) VALUES (?,?,?,?,?,?)"#,
    //         lessor_id,
    //         name,
    //         prefecture,
    //         ward,
    //         city,
    //         block,
    //     )
    //     .execute(conn)
    //     .await?
    //     .last_insert_id()
    //     .try_into()?;
    //     Ok(building_id)
    // }

    pub async fn find_by_id<'a, E>(pool: E, id: i64) -> Result<Option<Building>>
    where
        E: Executor<'a, Database = MySql>,
    {
        match RawBuilding::find_by_id(pool, id).await? {
            Some(raw) => Ok(Some(Building::try_from(raw)?)),
            None => Ok(None),
        }
    }
    pub async fn find_by_lessor_id<'a, E>(pool: E, lessor_id: i64) -> Result<Vec<Building>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let buildings = RawBuilding::find_by_lessor_id(pool, lessor_id)
            .await?
            .into_iter()
            .map(Building::try_from)
            .collect::<Result<_>>()?;
        Ok(buildings)
    }
    pub async fn find_by_ids<'a, E>(pool: E, ids: &[i64]) -> Result<Vec<Building>>
    where
        E: Executor<'a, Database = MySql>,
    {
        let buildings = RawBuilding::find_by_ids(pool, ids)
            .await?
            .into_iter()
            .map(Building::try_from)
            .collect::<Result<_>>()?;
        Ok(buildings)
    }
}
