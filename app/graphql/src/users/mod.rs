use std::collections::HashMap;

use dataloader::{cached::Loader, BatchFn};
use juniper::{graphql_object, FieldResult};
use ws_sql::{MySqlPool, User};

use crate::{context::GraphQLContext, impl_i64_convert, lessors::GQLLessor};

#[derive(juniper::GraphQLScalarValue, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[graphql(transparent)]
pub(crate) struct UserId(i32);
impl_i64_convert!(UserId);

#[derive(Clone)]
pub(crate) struct GQLUser(User);

impl From<User> for GQLUser {
    fn from(user: User) -> Self {
        GQLUser(user)
    }
}

#[graphql_object(context=GraphQLContext)]
impl GQLUser {
    fn id(&self) -> FieldResult<UserId> {
        Ok(UserId(self.0.id.try_into()?))
    }
    fn name(&self) -> &str {
        self.0.name.as_str()
    }
    async fn lessor(&self, ctx: &GraphQLContext) -> FieldResult<GQLLessor> {
        let lessor_id = self.0.lessor_id.try_into()?;
        let lessor = ctx.lessor_loader.try_load(lessor_id).await?;
        Ok(lessor)
    }
}

pub(crate) struct Batcher(MySqlPool);
pub(crate) type UserLoader = Loader<UserId, GQLUser, Batcher>;
pub(crate) fn user_loader(pool: &MySqlPool) -> UserLoader {
    Loader::new(Batcher(pool.clone()))
}

#[juniper::async_trait]
impl BatchFn<UserId, GQLUser> for Batcher {
    async fn load(&mut self, keys: &[UserId]) -> HashMap<UserId, GQLUser> {
        let user_ids = keys.iter().map(|&id| id.0.into()).collect::<Vec<_>>();
        let users = match User::find_by_ids(&self.0, &user_ids).await {
            Ok(users) => users,
            Err(e) => {
                log::error!("{:?}", e);
                return HashMap::new();
            }
        };

        users
            .into_iter()
            .filter_map(|user| {
                let user_id = UserId(user.id.try_into().ok()?);
                let user = GQLUser::from(user);
                Some((user_id, user))
            })
            .collect()
    }
}
