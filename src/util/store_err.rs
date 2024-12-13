use derive_more::derive::{Display, Error};
use http_error_macro::ImplHttpError;

#[derive(Debug, Display, Error, ImplHttpError)]
pub enum StoreError {

    #[display("数据库错误")]
    RB,

    #[display("店铺不存在")]
    StoreNotExists,

    #[display("店铺已存在")]
    StoreExists,

    #[display("更新店铺失败")]
    StoreUpdateFail,
}
