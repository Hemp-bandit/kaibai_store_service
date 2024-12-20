use actix_cors::Cors;
use actix_web::middleware::{from_fn, Compress, Logger};
use actix_web::{http, App, HttpServer};
use env::dotenv;
use once_cell::sync::OnceCell;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use rs_service_util::redis::RedisTool;
use rs_service_util::{self, jwt::JWT};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

mod dao;
mod entity;
mod util;
mod store;
mod product;
mod order;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "store", description = "store 接口"),
    ),
    modifiers(&JWT),
    security(
        ("JWT" = ["edit:items", "read:items"])
    )
)]
struct ApiDoc;

lazy_static::lazy_static! {
    static ref RB:RBatis=RBatis::new();
    static ref REDIS: OnceCell<RedisTool> = OnceCell::new();
}

#[actix_web::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    env_logger::init();
    init_db().await;
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let _ = REDIS.set(RedisTool::new(redis_url).await);

    let _ = HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .service(utoipa_actix_web::scope("/api/store").configure(store::configure()))
            .openapi_service(|api| Scalar::with_url("/doc", api))
            .into_app()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTION"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ]),
            )
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("t %P %s %{service_call}i"))
            // .wrap(from_fn(|req, next| {
            //     let rds = crate::REDIS.get().expect("msg");
            //     let conn = rds.conn.clone();
            //     rs_service_util::middleware::jwt_mw(req, next, conn)
            // }))
    })
    .keep_alive(None)
    .shutdown_timeout(5)
    .bind(gen_server_url())
    .expect("服务启动失败")
    .run()
    .await;
}
fn gen_server_url() -> String {
    let host = "0.0.0.0";
    let url = format!("{}:{}", host, 3001);
    log::info!("server is on, addr http://127.0.0.1:3001\n doc:  http://127.0.0.1:3001/doc");
    url
}

async fn init_db() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("database_url {database_url}");
    if let Err(e) = RB.link(MysqlDriver {}, &database_url).await {
        panic!("db err: {}", e.to_string());
    }
}
