use super::{PageQueryStoreData, StoreListItem, UpdateStoreData};
use crate::{
    dao::store_dao::{get_store_by_name, select_by_id, select_store_list},
    entity::store_entity::{CreateStoreData, StoreEntity},
    http_client,
    util::{store_err::StoreError, structs::UserEntity},
    RB,
};
use rbatis::Page;
use rs_service_util::{response::ResponseBody, time::get_current_time_fmt, transaction};

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

pub async fn get_store_list(data: PageQueryStoreData) -> Result<Page<StoreListItem>, StoreError> {
    let ex = RB.acquire().await.expect("msg");
    let mut offset = data.page_no - 1;
    if offset < 0 {
        offset = 0;
    }
    let records: Vec<StoreEntity> = select_store_list(
        &ex,
        data.name,
        data.create_by,
        (offset * data.take) as u64,
        data.take as u64,
    )
    .await
    .expect("msg");

    let client = http_client!();
    let mut item_list: Vec<StoreListItem> = vec![];
    for ele in records.into_iter() {
        let url = format!(
            "{}/api/user/{}",
            std::env::var("USER_SERVICE").expect("USER_SERVICE must be set"),
            ele.create_by
        );
        let response = client.get(url).send().await.expect("msg");

        let user_info: ResponseBody<UserEntity> = response.json().await.expect("msg");
        let item: StoreListItem = StoreListItem::from(ele, user_info.data);
        item_list.push(item);
    }

    let res = Page {
        records: item_list,
        total: 0,
        page_no: data.page_no as u64,
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
