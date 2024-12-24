use rbatis::{executor::Executor, py_sql};

use crate::{
    entity::product_entity::{ProductEntity, ProductItem},
    product::PageQueryProductData,
    store::PageQueryStoreBindData,
};

#[py_sql(
    "
`SELECT id, create_time,update_time, name,  price ,picture,count,ext,status  FROM product WHERE id = #{id}`
"
)]
pub async fn select_by_id(rb: &dyn Executor, id: i32) -> Option<ProductEntity> {}

#[py_sql(
    "
`SELECT id,create_time,update_time,name,status, price ,picture,count,ext FROM product  `
 if data.name != null:
   ` WHERE name = #{data.name} `
` limit #{data.page.page_no}, #{data.page.take}`
"
)]
pub async fn select_product_list(
    rb: &dyn Executor,
    data: &PageQueryProductData,
) -> Vec<ProductItem> {
}

#[py_sql(
    "
`SELECT count(1) FROM product`
 if data.name != null:
   ` WHERE name = #{data.name} `
"
)]
pub async fn list_count(rb: &dyn Executor, data: &PageQueryProductData) -> u64 {}

#[py_sql(
    "
  `SELECT id,create_time,update_time,name,status, price ,picture,count,ext FROM product WHERE id in (SELECT product_id FROM store_product WHERE store_id = #{data.id})  limit #{data.page.page_no}, #{data.page.take}`
  "
  )]
pub async fn get_bind_product(
    rb: &dyn Executor,
    data: &PageQueryStoreBindData,
) -> Vec<ProductItem> {
}
