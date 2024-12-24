use super::IdRes;
use crate::entity::{store_entity::StoreEntity, store_product_entity::StoreProductEntity};
use rbatis::{executor::Executor, py_sql};

#[py_sql(
    "
SELECT id FROM store WHERE name=#{name}
"
)]
pub async fn get_store_by_name(rb: &dyn Executor, name: &str) -> Option<IdRes> {}

#[py_sql(
    "
`SELECT id,create_time,update_time,status,create_by,name,picture,description,address,shell FROM store WHERE status = 1`
 if name != null:
   ` AND name = #{name}`
 if create_by != null:
   ` AND create_by = #{create_by}`
` limit #{offset}, #{take}`
"
)]
pub async fn select_store_list(
    rb: &dyn Executor,
    name: Option<String>,
    create_by: Option<i32>,
    offset: u64,
    take: u64,
) -> Vec<StoreEntity> {
}

#[py_sql(
    "
`SELECT count(1) FROM store  WHERE status = 1`
 if name != null:
   ` AND name = #{name}`
 if create_by != null:
   ` AND create_by = #{create_by}`
"
)]
pub async fn list_count(rb: &dyn Executor, name: Option<String>, create_by: Option<i32>) -> u64 {}

#[py_sql(
    "
`SELECT id,create_time,update_time,status,create_by,name,picture,description,address,shell FROM store WHERE status = 1 AND id = #{id}`
"
)]
pub async fn select_by_id(rb: &dyn Executor, id: i32) -> Option<StoreEntity> {}

#[py_sql(
    "
`SELECT id FROM store_product WHERE store_id=#{data.store_id} AND product_id = #{data.product_id}`
"
)]
pub async fn has_bind_product(rb: &dyn Executor, data: &StoreProductEntity) -> Option<i32> {}


#[py_sql(
  "
`DELETE FROM store_product WHERE store_id=#{data.store_id} AND product_id = #{data.product_id}`
"
)]
pub async fn delete_bind_product(rb: &dyn Executor, data: &StoreProductEntity) -> Option<i32> {}
