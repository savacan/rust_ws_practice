mod buildings;
mod context;
mod lessors;
mod macros;
mod schema;
mod users;

use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    route, web, App, Error, HttpResponse,
};
use juniper_actix::graphql_handler;
use ws_sql::{MySqlPool, User};

use crate::{context::GraphQLContext, schema::Schema};

pub trait GraphQLAppExt {
    fn configure_graphql_api(self) -> Self;
}

impl<T, B> GraphQLAppExt for App<T>
where
    B: MessageBody,
    T: ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
    >,
{
    fn configure_graphql_api(self) -> Self {
        self.app_data(web::Data::new(schema::schema()))
            .service(graphql_route)
    }
}

#[route("/api/graphql", method = "GET", method = "POST")]
async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
    user: Option<web::ReqData<User>>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, Error> {
    let context = GraphQLContext::new(pool.as_ref(), user.map(|req| req.into_inner()));
    graphql_handler(&schema, &context, req, payload).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
