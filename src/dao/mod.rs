use serde::{Deserialize, Serialize};

pub mod store_dao;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdRes {
    id: Option<i32>,
}
