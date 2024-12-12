use rs_service_util::transaction;

use crate::{
    dao::store_dao::get_store_by_name, entity::store_entity::StoreEntity,
    util::store_err::StoreError, RB,
};

use super::CreateStoreData;

pub async fn create_store(data: CreateStoreData) -> Result<(), StoreError> {
    let ex = RB.acquire().await.expect("msg");
    // 检测是否存在
    let db_res = get_store_by_name(&ex, &data.name)
        .await
        .map_err(|_err| StoreError::RB)?;
    drop(ex);
    if db_res.is_some() {
        return Err(StoreError::StoreExists);
    }

    let tx = transaction!().await;
    let new_store = StoreEntity::from_create(data);

    StoreEntity::insert(&tx, &new_store).await.expect("msg");

    Ok(())
}
