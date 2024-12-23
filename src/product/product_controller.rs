use actix_web::{post, web, Responder};
use rs_service_util::response::ResponseBody;

use crate::{
    entity::product_entity::{CreateProductReqData, UpdateProductReqData},
    util::store_err::StoreError,
};

use super::{product_service as service, PageQueryProductData};

#[utoipa::path(
  tag = "product",
  description  ="创建商品",
  responses( (status = 200) )
)]
#[post("/create_product")]
pub async fn create_product(
    req_data: web::Json<CreateProductReqData>,
) -> Result<impl Responder, StoreError> {
    let data: CreateProductReqData = req_data.into_inner();
    service::create_product(data).await?;
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "product",
 description  ="",
responses( (status = 200) )
)]
#[post("/update_product/{id}")]
pub async fn update_product(
    data: web::Json<UpdateProductReqData>,
    id: web::Path<i32>,
) -> Result<impl Responder, StoreError> {
    service::update_product(id.into_inner(), data.into_inner()).await?;
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "product",
 description  ="获取产品列表",
responses( (status = 200) )
)]
#[post("/get_product_list")]
pub async fn get_product_list(
    data: web::Json<PageQueryProductData>,
) -> Result<impl Responder, StoreError> {
    let data = service::get_product_list(data.into_inner()).await?;
    Ok(ResponseBody::default(Some(data)))
}
