use rs_service_util::transaction;

use crate::{
    dao::product_dao::select_by_id,
    entity::product_entity::{CreateProductReqData, ProductEntity, UpdateProductReqData},
    util::store_err::StoreError,
};

pub async fn create_product(data: CreateProductReqData) -> Result<(), StoreError> {
    let entity = ProductEntity::from(data);
    let tx = transaction!().await;
    ProductEntity::insert(&tx, &entity).await.map_err(|er| {
        log::error!("create_product fail:{:?}", er);
        StoreError::CreateProductFail
    })?;
    tx.commit().await.expect("msg");
    Ok(())
}

pub async fn update_product(id: i32, data: UpdateProductReqData) -> Result<(), StoreError> {
    let tx = transaction!().await;

    let entity = select_by_id(&tx, id).await.map_err(|er| {
        log::error!("select_by_id fail:{:?}", er);
        StoreError::SelectProductFail
    })?;

    let mut entity = match entity {
        None => {
            return Err(StoreError::SelectProductFail);
        }
        Some(data) => data,
    };

    entity.name = data.name.unwrap_or(entity.name);
    entity.picture = data.picture.unwrap_or(entity.picture);
    entity.price = data.price.unwrap_or(entity.price);
    entity.count = data.count.unwrap_or(entity.count);
    entity.ext = data.ext.unwrap_or(entity.ext);
    entity.update_time = rs_service_util::time::get_current_time_fmt();

    ProductEntity::update_by_column(&tx, &entity, "id")
        .await
        .map_err(|er| {
            log::error!("update_by_id fail:{:?}", er);
            StoreError::UpdateProductFail
        })?;

    Ok(())
}


