use actix_cors::Cors;
use actix_service::{Service, Transform};
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::{self, header},
    web, App, Error, HttpMessage, HttpServer,
};
use anyhow::Result;
use futures_util::{
    future::{self, LocalBoxFuture},
    FutureExt,
};
use std::{env, rc::Rc};
use ws_graph::GraphQLAppExt;
use ws_sql::{MySqlPool, User};

async fn setup_mysql() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to MySQL")
}

#[actix_web::main]
async fn main() {
    env_logger::init();
    let db_pool = setup_mysql().await;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|_origin, _req_head| true)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT]),
            )
            .wrap(EasyAuthentication::new(db_pool.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .configure_graphql_api()
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to 0.0.0.0:8080")
    .run()
    .await
    .expect("Failed to run server");
}

// test middleware

pub struct EasyAuthentication {
    pool: MySqlPool,
}
impl EasyAuthentication {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl<S, B> Transform<S, ServiceRequest> for EasyAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    B::Error: Into<Error>,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = TestAuthMiddleware<S>;
    type InitError = ();
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(TestAuthMiddleware {
            service: Rc::new(service),
            pool: self.pool.clone(),
        })
    }
}

pub struct TestAuthMiddleware<S> {
    service: Rc<S>,
    pool: MySqlPool,
}

impl<S, B> Service<ServiceRequest> for TestAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    S::Error: 'static,
    B: MessageBody + 'static,
    B::Error: Into<Error>,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, std::result::Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let pool = self.pool.clone();
        async move {
            let token = parse_token(&req);
            if let Ok(t) = token {
                if let Ok(Some(user)) = User::find_by_id(&pool, t).await {
                    req.extensions_mut().insert(user);
                }
            }
            service.call(req).await.map(|res| res.map_into_left_body())
        }
        .boxed_local()
    }
}

fn parse_token(request: &ServiceRequest) -> Result<i64> {
    let header = request
        .headers()
        .get(&actix_web::http::header::AUTHORIZATION)
        .ok_or_else(|| anyhow::anyhow!("Request doesn't have the authorization header."))?;

    let token = header.to_str()?.parse::<i64>()?;
    Ok(token)
}
