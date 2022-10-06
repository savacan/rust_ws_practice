use ws_sql::{MySqlPool, User};

use crate::{
    buildings::{building_loader, BuildingLoader},
    lessors::{lessor_loader, LessorLoader},
    users::{user_loader, UserLoader},
};

#[allow(dead_code)]
pub(crate) struct GraphQLContext {
    pub(crate) pool: MySqlPool,
    pub(crate) user: Option<User>,
    // ここにloader置く予定
    pub(crate) user_loader: UserLoader,
    pub(crate) lessor_loader: LessorLoader,
    pub(crate) building_loader: BuildingLoader,
}

impl GraphQLContext {
    pub(crate) fn new(pool: &MySqlPool, user: Option<User>) -> Self {
        Self {
            pool: pool.clone(),
            user,
            user_loader: user_loader(pool),
            lessor_loader: lessor_loader(pool),
            building_loader: building_loader(pool),
        }
    }
}

impl juniper::Context for GraphQLContext {}
