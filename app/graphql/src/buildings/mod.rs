use dataloader::{cached::Loader, BatchFn};
use juniper::{graphql_object, FieldResult};
use std::collections::HashMap;
use ws_sql::{Building, MySqlPool};

use crate::{
    context::GraphQLContext,
    impl_i64_convert,
    lessors::{GQLLessor, LessorId},
    users::GQLUser,
};

#[derive(juniper::GraphQLScalarValue, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[graphql(transparent)]
pub(crate) struct BuildingId(i32);
impl_i64_convert!(BuildingId);

#[derive(Clone)]
pub(crate) struct GQLBuilding(Building);

impl From<Building> for GQLBuilding {
    fn from(building: Building) -> Self {
        GQLBuilding(building)
    }
}
impl GQLBuilding {
    pub(crate) fn lessor_id(&self) -> FieldResult<LessorId> {
        Ok(self.0.lessor_id.try_into()?)
    }
}

#[graphql_object(context=GraphQLContext)]
impl GQLBuilding {
    fn id(&self) -> FieldResult<BuildingId> {
        Ok(BuildingId(self.0.id.try_into()?))
    }
    fn name(&self) -> &str {
        self.0.name.as_str()
    }
    fn prefecture(&self) -> &str {
        self.0.prefecture.as_str()
    }
    fn ward(&self) -> &str {
        self.0.ward.as_str()
    }
    fn city(&self) -> &str {
        self.0.city.as_str()
    }
    fn block(&self) -> Option<&str> {
        self.0.block.as_deref()
    }
    pub(crate) async fn lessor(&self, ctx: &GraphQLContext) -> FieldResult<GQLLessor> {
        let lessor_id = self.0.lessor_id.try_into()?;
        let lessor = ctx.lessor_loader.try_load(lessor_id).await?;
        Ok(lessor)
    }
}

pub(crate) struct Batcher(MySqlPool);
pub(crate) type BuildingLoader = Loader<BuildingId, GQLBuilding, Batcher>;
pub(crate) fn building_loader(pool: &MySqlPool) -> BuildingLoader {
    Loader::new(Batcher(pool.clone()))
}

#[juniper::async_trait]
impl BatchFn<BuildingId, GQLBuilding> for Batcher {
    async fn load(&mut self, keys: &[BuildingId]) -> HashMap<BuildingId, GQLBuilding> {
        let building_ids = keys.iter().map(|&id| id.0.into()).collect::<Vec<_>>();
        let buildings = match Building::find_by_ids(&self.0, &building_ids).await {
            Ok(buildings) => buildings,
            Err(e) => {
                log::error!("{:?}", e);
                return HashMap::new();
            }
        };

        buildings
            .into_iter()
            .filter_map(|building| {
                let building_id = BuildingId(building.id.try_into().ok()?);
                let building = GQLBuilding::from(building);
                Some((building_id, building))
            })
            .collect()
    }
}
