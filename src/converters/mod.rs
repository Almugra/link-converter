use std::fmt::Debug;

use crate::Result;
use async_trait::async_trait;
use url::Url;

pub mod acbuy;
pub mod cnfans;
pub mod cssbuy;
pub mod joyabuy;
pub mod lovegobuy;
pub mod mobile_intl_taobao;
pub mod mobile_taobao;
pub mod mulebuy;
pub mod oopbuy;
pub mod ootdbuy;
pub mod orientdig;
pub mod you_shop_10;

#[async_trait]
/// A trait that defines how to convert a link into its raw form
pub trait LinkConverter: Send + Sync + Debug {
    /// Checks if this converter can handle the given URL.
    fn can_convert(&self, url: &Url) -> bool;

    /// Converts the URL into its raw form;
    async fn convert(&self, url: Url) -> Result<String>;
}

pub mod destination {
    pub fn taobao(id: &str) -> String {
        format!("https://item.taobao.com/item.htm?id={}", id)
    }

    pub fn weidian(id: &str) -> String {
        format!("https://weidian.com/item.html?itemID={}", id)
    }

    pub fn ali_1688(id: &str) -> String {
        format!("https://detail.1688.com/offer/{}.html", id)
    }
}
