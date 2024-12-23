use rbatis::crud;
use rs_service_util::{time::get_current_time_fmt, Status};
use serde::{Deserialize, Serialize};
use utility_types::Omit;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, Omit)]
#[omit(
    arg(ident = ProductItem, fields(status), derive(Clone, Debug, Serialize, Deserialize, ToSchema),forward_attrs(),),
    arg(ident = CreateProductReqData, fields(status, id, create_time, update_time), derive(Clone, Debug, Serialize, Deserialize, ToSchema),forward_attrs()),
)]
pub struct ProductEntity {
    pub id: Option<i32>,
    pub create_time: String,
    pub update_time: String,
    pub name: String,
    pub status: u8,      // 0:下架 1:上架
    pub price: f64,      // 价格
    pub picture: String, // 图片
    pub count: i32,      // 库存
    pub ext: String,     // 扩展字段
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductReqData {
    pub name: Option<String>,
    pub status: Option<u8>,
    pub price: Option<f64>,
    pub picture: Option<String>,
    pub count: Option<i32>,
    pub ext: Option<String>,
}

impl Default for ProductEntity {
    fn default() -> Self {
        Self {
            id: None,
            create_time: "".to_string(),
            update_time: "".to_string(),
            name: "".to_string(),
            status: 0,
            price: 0.0,
            picture: "".to_string(),
            count: 0,
            ext: "".to_string(),
        }
    }
}

impl From<CreateProductReqData> for ProductEntity {
    fn from(data: CreateProductReqData) -> Self {
        Self {
            id: None,
            create_time: get_current_time_fmt(),
            update_time: get_current_time_fmt(),
            name: data.name,
            status: Status::DEACTIVE as u8,
            price: data.price,
            picture: data.picture,
            count: data.count,
            ext: data.ext,
        }
    }
}
crud!(ProductEntity {}, "product");
