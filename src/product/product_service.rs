use rbatis::Page;
use rs_service_util::transaction;

use crate::{
    dao::product_dao::{list_count, select_by_id, select_product_list},
    entity::product_entity::{
        CreateProductReqData, ProductEntity, ProductItem, UpdateProductReqData,
    },
    util::store_err::StoreError,
    RB,
};

use super::PageQueryProductData;

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

pub async fn get_product_list(mut data: PageQueryProductData) -> Result<Page<ProductItem>, StoreError> {
    let ex = RB.acquire().await.expect("msg");

    data.page.page_no = (data.page.page_no - 1) * data.page.take;

    let list = select_product_list(&ex, &data.data).await.map_err(|er| {
        log::error!("select_product_list fail:{:?}", er);
        StoreError::SelectProductListFail
    })?;

    let total = list_count(&ex, &data.data).await.map_err(|er| {
        log::error!("select_product_list fail:{:?}", er);
        StoreError::CountFail
    })?;

    let res = Page::new(data.page.page_no as u64, data.page.take as u64, total, list);

    Ok(res)
}
