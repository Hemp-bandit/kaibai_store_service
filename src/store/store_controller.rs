use actix_web::{post, Responder};
use rs_service_util::response::{BizError, ResponseBody};

#[utoipa::path(
  tag = "store",
  description  ="创建商铺",
  responses( (status = 200) )
)]
#[post("/create_store")]
pub async fn create_store() -> Result<impl Responder, BizError> {
    Ok(ResponseBody::success("ok"))
}
