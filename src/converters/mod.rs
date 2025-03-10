use std::fmt::Debug;

use crate::Result;
use async_trait::async_trait;
use url::Url;

pub mod cssbuy;
pub mod mobile_intl_taobao;
pub mod mobile_taobao;
pub mod you_shop_10;

#[async_trait]
/// A trait that defines how to convert a link into its raw form
pub trait LinkConverter: Send + Sync + Debug {
    /// Checks if this converter can handle the given URL.
    fn can_convert(&self, url: &Url) -> bool;

    /// Converts the URL into its raw form;
    async fn convert(&self, url: Url) -> Result<String>;
}
