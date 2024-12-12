use derive_more::derive::{Display, Error};
use http_error_macro::ImplHttpError;

#[derive(Debug, Display, Error, ImplHttpError)]
pub enum StoreError {

    // #[display("asdfasd")]
    // BizError,
}
