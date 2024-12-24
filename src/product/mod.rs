use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::util::structs::PageData;

mod product_controller;
mod product_service;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(product_controller::create_product);
        config.service(product_controller::update_product);
        config.service(product_controller::get_product_list);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PageQueryProductData {
    pub name: Option<String>,
    pub page: PageData,
}
