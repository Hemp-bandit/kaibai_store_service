use rbatis::{executor::Executor, py_sql};

use super::IdRes;

#[py_sql(
    "
SELECT id FROM store WHERE name=#{name}
"
)]
pub async fn get_store_by_name(rb: &dyn Executor, name: &str) -> Option<IdRes> {}
