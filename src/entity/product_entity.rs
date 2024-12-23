use rbatis::{crud, rbdc::Decimal};
use rs_service_util::{time::get_current_time_fmt, Status};
use serde::{Deserialize, Serialize};
use utility_types::{Omit, Pick};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, Omit, ToSchema)]
#[omit(
    arg(ident = CreateProductReqData, fields(status, id, create_time, update_time), derive(Clone, Debug, Serialize, Deserialize,ToSchema),forward_attrs()),
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProductItem {
    pub id: Option<i32>,
    pub create_time: String,
    pub update_time: String,
    pub name: String,
    pub picture: String,
    pub count: i32,
    pub ext: String,
    pub price: Decimal,
}
impl From<ProductEntity> for ProductItem {
    fn from(src: ProductEntity) -> Self {
        Self {
            id: src.id,
            create_time: src.create_time,
            update_time: src.update_time,
            name: src.name,
            picture: src.picture,
            count: src.count,
            ext: src.ext,
            price: Decimal::from_f64(src.price).unwrap(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Pick, ToSchema)]
#[pick(
    arg(ident = PqProductItem, fields(name), derive(Clone, Debug, Serialize, Deserialize,ToSchema),forward_attrs(),),
)]
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
