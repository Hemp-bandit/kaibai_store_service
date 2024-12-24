use actix_web::{delete, post, put, web, Responder};
use rs_service_util::response::ResponseBody;

use crate::{
    entity::{store_entity::CreateStoreData, store_product_entity::StoreProductEntity},
    store::store_service as service,
    util::store_err::StoreError,
};

use super::{PageQueryStoreBindData, PageQueryStoreData, UpdateStoreData};

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
    service::create_store(data).await?;
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "store",
 description  ="获取商铺列表",
responses( (status = 200) )
)]
#[post("/get_store_list")]
pub async fn get_store_list(
    data: web::Json<PageQueryStoreData>,
) -> Result<impl Responder, StoreError> {
    let res = service::get_store_list(data.into_inner()).await?;

    Ok(ResponseBody::default(Some(res)))
}

#[utoipa::path(
tag = "store",
description  ="更新店铺",
responses( (status = 200) )
)]
#[put("/update_store")]
pub async fn update_store(data: web::Json<UpdateStoreData>) -> Result<impl Responder, StoreError> {
    service::update_store(data.into_inner()).await?;
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "store",
 description  ="商店绑定商品",
responses( (status = 200) )
)]
#[post("/bind_product")]
pub async fn bind_product(
    data: web::Json<StoreProductEntity>,
) -> Result<impl Responder, StoreError> {
    service::bind_product(data.into_inner()).await?;
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "store",
 description  ="解除商品绑定",
responses( (status = 200) )
)]
#[delete("/unbind_product")]
pub async fn unbind_product(
    data: web::Json<StoreProductEntity>,
) -> Result<impl Responder, StoreError> {
    service::unbind_product(data.into_inner()).await?;
    Ok(ResponseBody::success("ok"))
}

#[utoipa::path(
tag = "store",
 description  ="获取商铺中的商品",
responses( (status = 200) )
)]
#[post("/get_bind_products")]
pub async fn get_bind_products(
    data: web::Json<PageQueryStoreBindData>,
) -> Result<impl Responder, StoreError> {
    let list = service::get_bind_products(data.into_inner()).await?;
    Ok(ResponseBody::default(Some(list)))
}
