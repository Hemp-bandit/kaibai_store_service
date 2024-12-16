use crate::{
    dao::store_dao::{get_store_by_name, select_by_id, select_store_list},
    entity::store_entity::StoreEntity,
    util::store_err::StoreError,
    RB,
};
use rbatis::Page;
use rs_service_util::{time::get_current_time_fmt, transaction};

use super::{CreateStoreData, PageQueryStoreData, UpdateStoreData};

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
    tx.commit().await.expect("msg");

    Ok(())
}

pub async fn get_store_list(data: PageQueryStoreData) -> Result<Page<StoreEntity>, StoreError> {
    let ex = RB.acquire().await.expect("msg");
    let records: Vec<StoreEntity> = select_store_list(
        &ex,
        data.name,
        data.create_by,
        (data.offset - 1) * data.take,
        data.take,
    )
    .await
    .expect("msg");

    let res = Page {
        records,
        total: 0,
        page_no: data.offset as u64,
        page_size: data.take as u64,
        do_count: true,
    };

    Ok(res)
}

pub async fn update_store(data: UpdateStoreData) -> Result<(), StoreError> {
    let ex = RB.acquire().await.expect("msg");
    // 检测是否存在
    let db_res = select_by_id(&ex, data.id)
        .await
        .map_err(|_| StoreError::StoreNotExists)?;

    log::debug!("db_res {db_res:?}");
    match db_res {
        None => {
            return Err(StoreError::StoreNotExists);
        }
        Some(mut store) => {
            store.address = data.address.unwrap_or(store.address);
            store.name = data.name.unwrap_or(store.name);
            store.picture = data.picture.unwrap_or(store.picture);
            store.description = data.description.unwrap_or(store.description);
            store.shell = data.shell.unwrap_or(store.shell);
            store.update_time = get_current_time_fmt();
            StoreEntity::update_by_column(&ex, &store, "id")
                .await
                .map_err(|_| StoreError::StoreUpdateFail)?;
        }
    }

    Ok(())
}
