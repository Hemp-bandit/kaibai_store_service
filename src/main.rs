use actix_cors::Cors;
use actix_web::middleware::{from_fn, Compress, Logger};
use actix_web::{http, App, HttpServer};
use env::dotenv;
use middleware::jwt_mw;
use once_cell::sync::OnceCell;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use rs_service_util::redis::RedisTool;
use rs_service_util::{self, jwt::JWT};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

mod entity;
mod middleware;
mod store;
mod util;

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
    static ref REDIS_KEY:String = "store_service".to_string();
    static ref RB:RBatis=RBatis::new();
    static ref REDIS: OnceCell<RedisTool> = OnceCell::new();
}

#[actix_web::main]
async fn main() {
    dotenv().expect("Failed to load .env file");
    env_logger::init();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let actor = RedisTool::new(redis_url).await;

    init_db().await;
    REDIS.set(actor.clone()).expect("msg");

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
            .wrap(Logger::new("%a %{Referer}i"))
            .wrap(from_fn(jwt_mw))
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
    let url = format!("{}:{}", host, 3000);
    log::info!("server is on, addr http://127.0.0.1:3000\n doc:  http://127.0.0.1:3000/doc");
    url
}

async fn init_db() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    log::info!("database_url {database_url}");
    if let Err(e) = RB.link(MysqlDriver {}, &database_url).await {
        panic!("db err: {}", e.to_string());
    }
}
