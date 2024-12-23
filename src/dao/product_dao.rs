use rbatis::{executor::Executor, py_sql};

use crate::entity::product_entity::ProductEntity;

#[py_sql(
    "
`SELECT id, create_time,update_time, name,price,picture,count,ext,status  FROM product WHERE status = 1 AND id = #{id}`
"
)]
pub async fn select_by_id(rb: &dyn Executor, id: i32) -> Option<ProductEntity> {}

// #[py_sql(
//     "
// `SELECT count(1) FROM store  WHERE status = 1`
//  if name != null:
//    ` AND name = #{name}`
//  if create_by != null:
//    ` AND create_by = #{create_by}`
// "
// )]
// pub async fn updated_product_by_id(rb: &dyn Executor, id: i32, data: UpdateProductReqData) -> u64 {}
