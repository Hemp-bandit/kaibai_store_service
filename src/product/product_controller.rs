use actix_web::{post, web, Responder};
use rs_service_util::response::ResponseBody;

use crate::{entity::product_entity::CreateProductReqData, util::store_err::StoreError};

use super::product_service as service;

#[utoipa::path(
  tag = "store",
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
