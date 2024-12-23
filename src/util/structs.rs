use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub picture: Option<String>,
    pub introduce: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PageData {
    pub page_no: i64,
    pub take: i64,
}
