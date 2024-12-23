use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{entity::store_entity::StoreEntity, util::structs::UserEntity};

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
pub struct PageQueryStoreData {
    pub name: Option<String>,
    pub create_by: Option<i32>, // 创建人
    pub page_no: i64,
    pub take: i64,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreListItem {
    pub id: i32,
    pub create_time: String,
    pub update_time: String,
    pub create_by: UserEntity, // 创建人
    pub name: String,
    pub picture: String,
    pub description: String,
    pub address: String,
    pub shell: String, // 销售方向
}

impl StoreListItem {
    pub fn from(data: StoreEntity, user: UserEntity) -> Self {
        Self {
            create_by: user,
            id: data.id.unwrap(),
            name: data.name,
            picture: data.picture,
            description: data.description,
            address: data.address,
            shell: data.shell,
            create_time: data.create_time,
            update_time: data.update_time,
        }
    }
}
