use actix_web::{get, post, put, web, Responder};
use rs_service_util::{response::ResponseBody, transaction};

use crate::{
    dao::store_dao::get_store_by_name, store::CreateStoreData, util::store_err::StoreError, RB,
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
    let ex = RB.acquire().await.expect("msg");
    // 检测是否存在
    let db_res = get_store_by_name(&ex, &req_data.name)
        .await
        .map_err(|_err| StoreError::RB)?;
    drop(ex);
    if db_res.is_some() {
        return Err(StoreError::StoreExists);
    }

    let tx = transaction!().await;
    // let new_store = 


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
