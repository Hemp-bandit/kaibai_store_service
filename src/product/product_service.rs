use rs_service_util::transaction;

use crate::{
    entity::product_entity::{CreateProductReqData, ProductEntity},
    util::store_err::StoreError,
};

pub async fn create_product(data: CreateProductReqData) -> Result<(), StoreError> {
    let entity = ProductEntity::from(data);
    let tx = transaction!().await;
    ProductEntity::insert(&tx, &entity).await.map_err(|er| {
        log::error!("create_product fail:{:?}", er);
        StoreError::createProductFail
    })?;
    Ok(())
}
