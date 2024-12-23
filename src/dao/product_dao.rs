use rbatis::{executor::Executor, py_sql};

use crate::entity::product_entity::{PqProductItem, ProductEntity, ProductItem};

#[py_sql(
    "
`SELECT id, create_time,update_time, name,  price ,picture,count,ext,status  FROM product WHERE status = 1 AND id = #{id}`
"
)]
pub async fn select_by_id(rb: &dyn Executor, id: i32) -> Option<ProductEntity> {}

#[py_sql(
    "
`SELECT id,create_time,update_time,name,status, price ,picture,count,ext FROM product  `
 if data.name != null:
   ` WHERE name = #{data.name}`
"
)]
pub async fn select_product_list(rb: &dyn Executor, data:&PqProductItem) -> Vec<ProductItem> {}

#[py_sql(
    "
`SELECT count(1) FROM product`
 if data.name != null:
   ` WHERE name = #{data.name}`
"
)]
pub async fn list_count(rb: &dyn Executor, data:& PqProductItem) -> u64 {}
