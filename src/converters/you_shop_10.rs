use core::fmt;
use std::time::Duration;

use super::LinkConverter;
use crate::{error::Error, Result};
use async_trait::async_trait;
use headless_chrome::Browser;
use lazy_regex::regex_captures;
use url::Url;

pub struct YouShop10(Browser);

impl YouShop10 {
    pub fn new(browser: Browser) -> Self {
        YouShop10(browser)
    }
}

#[async_trait]
impl LinkConverter for YouShop10 {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("k.youshop10.com")
    }

    async fn convert(&self, url: Url) -> Result<String> {
        let tab = self.0.new_tab()?;

        tab.navigate_to(url.as_str())?;

        tab.wait_for_element_with_custom_timeout(".into-cart", Duration::from_secs(20))?;

        match regex_captures!(r"itemID=(\d+)", &tab.get_url()) {
            Some((_, item_id)) if !item_id.is_empty() => {
                Ok(format!("https://weidian.com/item.html?itemID={}", item_id))
            }
            _ => Err(Error::FailedToRedirectUrl { url }),
        }
    }
}

// region:    --- Impl-Boilerplate

impl fmt::Debug for YouShop10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("YouShop10").finish()
    }
}

// endregion: --- Impl-Boilerplate

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use super::*;

    #[test]
    fn test_detects_convertable_url() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://k.youshop10.com/-s=uo-wD?a=b&p=iphone&wfr=BuyercopyURL&share_relation=e0fd773efc74bec4_1651287329_1")?;
        let converter = YouShop10::new(Browser::default()?);

        // -- Exec
        let actual_value = converter.can_convert(&url);

        // -- Check
        assert!(actual_value);

        Ok(())
    }

    #[tokio::test]
    async fn test_url_conversion() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://k.youshop10.com/-s=uo-wD?a=b&p=iphone&wfr=BuyercopyURL&share_relation=e0fd773efc74bec4_1651287329_1")?;
        let converter = YouShop10::new(Browser::default()?);

        // -- Exec
        let actual_converted_url = converter.convert(url).await?;

        // -- Check
        let expected_converted_url = "https://weidian.com/item.html?itemID=7301608442";
        assert_eq!(actual_converted_url, expected_converted_url);

        Ok(())
    }
}

// endregion: --- Tests
