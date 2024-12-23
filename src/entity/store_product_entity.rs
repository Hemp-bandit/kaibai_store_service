use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreProductEntity {
    pub create_time: String,
    pub update_time: String,
    pub store_id: i32,
    pub product_id: i32, //产品id
}

crud!(StoreProductEntity {}, "store_product");
