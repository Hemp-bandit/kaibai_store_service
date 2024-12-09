use utoipa_actix_web::service_config::ServiceConfig;

mod store_controller;
mod store_service;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(store_controller::create_store);
        config.service(store_controller::update_store);
        config.service(store_controller::get_store_list);

      

    }
}
