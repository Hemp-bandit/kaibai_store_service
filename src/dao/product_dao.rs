use rbatis::{executor::Executor, py_sql};

use crate::entity::product_entity::ProductItem;

#[py_sql(
    "
`SELECT id, create_time,update_time, name,price,picture,count,ext  FROM product WHERE status = 1 AND id = #{id}`
"
)]
pub async fn select_by_id(rb: &dyn Executor, id: i32) -> Option<ProductItem> {}
