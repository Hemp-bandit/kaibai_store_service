use rbatis::crud;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StoreProductEntity {
    pub store_id: i32,
    pub product_id: i32, //产品id
}

crud!(StoreProductEntity {}, "store_product");
