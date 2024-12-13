use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateStoreData {
    pub name: String,
    pub picture: String,
    pub description: String,
    pub address: String,
    pub shell: String,
    pub create_by: i32, // 创建人
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PageQueryStoreData {
    pub name: Option<String>,
    pub create_by: Option<i32>, // 创建人
    pub offset: u64,
    pub take: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateStoreData {
    pub id: i32,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub shell: Option<String>,
}
