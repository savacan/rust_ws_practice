use actix_web::web::ReqData;
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use ws_sql::{Building, Lessor};

use crate::{buildings::GQLBuilding, context::GraphQLContext, lessors::GQLLessor};

pub(super) type Schema =
    RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

pub(super) fn schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}

pub(super) struct QueryRoot;

#[graphql_object(context = GraphQLContext)]
impl QueryRoot {
    async fn buildings(ctx: &GraphQLContext) -> FieldResult<Vec<GQLBuilding>> {
        // 単純なビルの一覧はlessorしか取れない
        match &ctx.user {
            Some(user) => {
                let buildings = Building::find_by_lessor_id(&ctx.pool, user.lessor_id).await?;
                Ok(buildings.into_iter().map(From::from).collect())
            }
            // 認証errorは一旦考えない
            None => Ok(vec![]),
        }
    }
    async fn lessors(ctx: &GraphQLContext) -> FieldResult<Vec<GQLLessor>> {
        let lessors = Lessor::find_all(&ctx.pool).await?;
        Ok(lessors.into_iter().map(From::from).collect())
    }
    async fn foo() -> FieldResult<String> {
        Ok("foo".to_string())
    }
    async fn hoge(ctx: &GraphQLContext) -> FieldResult<String> {
        let user = &ctx.user;
        if let Some(u) = user {
            println!("{:?}", u);
        }
        Ok("hoge".to_string())
    }
}

pub(super) struct MutationRoot;

#[graphql_object(context = GraphQLContext)]
impl MutationRoot {
    async fn bar(ctx: &GraphQLContext) -> FieldResult<String> {
        let user = &ctx.user;
        if let Some(u) = user {
            println!("{:?}", u);
        }
        Ok("bar".to_string())
    }
}
