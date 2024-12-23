use rbatis::crud;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoreStuffEntity {
    pub id: Option<i32>,
    pub create_time: String,
    pub update_time: String,
    pub store_id: i32,
    pub stuff_id: i32,
    pub closeness:i32 // 亲密度
}

crud!(StoreStuffEntity {}, "store_stuff");
