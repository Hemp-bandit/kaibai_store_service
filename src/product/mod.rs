use utoipa_actix_web::service_config::ServiceConfig;

mod product_controller;
mod product_service;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        // config.service(product_controllder);
    }
}
