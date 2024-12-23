use rbatis::crud;
use rs_service_util::{time::get_current_time_fmt, Status};
use serde::{Deserialize, Serialize};
use utility_types::Pick;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, Pick)]
#[pick(
    arg(ident = CreateStoreData, fields(name, picture,description,shell,create_by), derive(Clone, Debug, Serialize, Deserialize, ToSchema)), forward_attrs()
)]
pub struct StoreEntity {
    pub id: Option<i32>,
    pub create_time: String,
    pub update_time: String,
    pub status: i8,
    pub create_by: i32, // 创建人
    pub name: String,
    pub picture: String,
    pub description: String,
    pub address: String,
    pub shell: String, // 销售方向
}

impl StoreEntity {
    pub fn from_create(data: CreateStoreData) -> Self {
        Self {
            id: None,
            create_time: get_current_time_fmt(),
            update_time: get_current_time_fmt(),
            status: Status::ACTIVE as i8,
            name: data.name,
            create_by: data.create_by,
            picture: data.picture,
            description: data.description,
            address: "".to_string(),
            shell: data.shell,
        }
    }
}

crud!(StoreEntity {}, "store");
