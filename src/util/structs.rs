use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub picture: Option<String>,
    pub introduce: Option<String>,
}
