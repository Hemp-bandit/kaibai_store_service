use super::{PageQueryStoreBindData, PageQueryStoreData, StoreListItem, UpdateStoreData};
use crate::{
    dao::{
        product_dao::get_bind_product,
        store_dao::{
            delete_bind_product, get_store_by_name, has_bind_product, list_count, select_by_id,
            select_store_list,
        },
    },
    entity::{
        product_entity::ProductItem,
        store_entity::{CreateStoreData, StoreEntity},
        store_product_entity::StoreProductEntity,
    },
    http_client,
    util::{store_err::StoreError, structs::UserEntity},
    RB,
};
use rbatis::Page;
use rs_service_util::{response::ResponseBody, time::get_current_time_fmt, transaction};

/// 创建商店
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

/// 商店列表
pub async fn get_store_list(data: PageQueryStoreData) -> Result<Page<StoreListItem>, StoreError> {
    let ex = RB.acquire().await.expect("msg");
    let mut offset = data.page_no - 1;
    if offset < 0 {
        offset = 0;
    }
    let records: Vec<StoreEntity> = select_store_list(
        &ex,
        data.name.clone(),
        data.create_by.clone(),
        (offset * data.take) as u64,
        data.take as u64,
    )
    .await
    .expect("msg");

    let total = list_count(&ex, data.name, data.create_by)
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

    let res = Page::new(data.page_no as u64, data.take as u64, total, item_list);

    Ok(res)
}

/// 更新商店信息
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

/// 绑定商品
pub async fn bind_product(data: StoreProductEntity) -> Result<(), StoreError> {
    let ex = RB.acquire().await.expect("msg");
    let res = has_bind_product(&ex, &data).await.map_err(|err| {
        log::error!("has_bind_product err: {:?}", err);
        StoreError::GetBindProductFail
    })?;

    if res.is_some() {
        return Ok(());
    }

    let tx = transaction!().await;
    StoreProductEntity::insert(&tx, &data)
        .await
        .map_err(|err| {
            log::error!("bind_product err: {:?}", err);
            StoreError::BindProductFail
        })?;

    tx.commit().await.expect("msg");
    Ok(())
}

pub async fn unbind_product(data: StoreProductEntity) -> Result<(), StoreError> {
    let tx = transaction!().await;
    delete_bind_product(&tx, &data).await.map_err(|err| {
        log::error!("unbind_product err: {:?}", err);
        StoreError::UnbindProductFail
    })?;

    tx.commit().await.expect("msg");
    Ok(())
}

pub async fn get_bind_products(
    mut data: PageQueryStoreBindData,
) -> Result<Page<ProductItem>, StoreError> {
    let ex = RB.acquire().await.expect("msg");
    data.page.page_no = (data.page.page_no - 1) * data.page.take;
    
    let list = get_bind_product(&ex, &data).await.map_err(|err| {
        log::error!("get_bind_products err: {:?}", err);
        StoreError::GetBindProductFail
    })?;


    let p = Page::new(data.page.page_no as u64, data.page.take as u64, 0, list);
    Ok(p)
}
