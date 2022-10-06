use std::collections::HashMap;

use dataloader::{cached::Loader, BatchFn};
use juniper::{graphql_object, FieldResult};
use ws_sql::{Building, Lessor, MySqlPool, User};

use crate::{buildings::GQLBuilding, context::GraphQLContext, impl_i64_convert, users::GQLUser};

#[derive(juniper::GraphQLScalarValue, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[graphql(transparent)]
pub(crate) struct LessorId(i32);

impl_i64_convert!(LessorId);

#[derive(Clone)]
pub(crate) struct GQLLessor(Lessor);

impl From<Lessor> for GQLLessor {
    fn from(lessor: Lessor) -> Self {
        GQLLessor(lessor)
    }
}
#[graphql_object(context=GraphQLContext)]
impl GQLLessor {
    fn id(&self) -> FieldResult<LessorId> {
        Ok(LessorId(self.0.id.try_into()?))
    }
    fn name(&self) -> &str {
        self.0.name.as_str()
    }

    async fn users(&self, ctx: &GraphQLContext) -> FieldResult<Vec<GQLUser>> {
        let lessor_users = User::find_by_lessor_id(&ctx.pool, self.0.id).await?;
        Ok(lessor_users.into_iter().map(From::from).collect())
    }

    async fn buildings(&self, ctx: &GraphQLContext) -> FieldResult<Vec<GQLBuilding>> {
        let buildings = Building::find_by_lessor_id(&ctx.pool, self.0.id).await?;
        Ok(buildings.into_iter().map(From::from).collect())
    }
}

pub(crate) struct Batcher(MySqlPool);
pub(crate) type LessorLoader = Loader<LessorId, GQLLessor, Batcher>;
pub(crate) fn lessor_loader(pool: &MySqlPool) -> LessorLoader {
    Loader::new(Batcher(pool.clone()))
}

#[juniper::async_trait]
impl BatchFn<LessorId, GQLLessor> for Batcher {
    async fn load(&mut self, keys: &[LessorId]) -> HashMap<LessorId, GQLLessor> {
        let lessor_ids = keys.iter().map(|&id| id.0.into()).collect::<Vec<_>>();
        let lessors = match Lessor::find_by_ids(&self.0, &lessor_ids).await {
            Ok(lessors) => lessors,
            Err(e) => {
                log::error!("{:?}", e);
                return HashMap::new();
            }
        };

        lessors
            .into_iter()
            .filter_map(|lessor| {
                let lessor_id = LessorId(lessor.id.try_into().ok()?);
                let lessor = GQLLessor::from(lessor);
                Some((lessor_id, lessor))
            })
            .collect()
    }
}
