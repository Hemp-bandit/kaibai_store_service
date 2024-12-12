use actix_web::{get, post, put, web, Responder};
use rs_service_util::response::ResponseBody;

use crate::{
    store::{store_service, CreateStoreData},
    util::store_err::StoreError,
};

#[utoipa::path(
  tag = "store",
  description  ="创建商铺",
  responses( (status = 200) )
)]
#[post("/create_store")]
pub async fn create_store(
    req_data: web::Json<CreateStoreData>,
) -> Result<impl Responder, StoreError> {
    let data: CreateStoreData = req_data.into_inner();
    store_service::create_store(data).await?;

    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "store",
 description  ="获取商铺列表",
responses( (status = 200) )
)]
#[get("/get_store_list")]
pub async fn get_store_list() -> Result<impl Responder, StoreError> {
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "store",
 description  ="更新店铺",
responses( (status = 200) )
)]
#[put("/update_store")]
pub async fn update_store() -> Result<impl Responder, StoreError> {
    Ok(ResponseBody::success("ok"))
}
