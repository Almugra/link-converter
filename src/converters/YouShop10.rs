use std::time::Duration;

use super::LinkConverter;
use crate::Result;
use headless_chrome::Browser;
use url::Url;

pub struct YouShop10;

impl LinkConverter for YouShop10 {
    fn can_convert(url: &Url) -> bool {
        url.host_str() == Some("k.youshop10.com")
    }

    fn convert(url: &Url) -> Result<String> {
        let browser = Browser::default()?;

        let tab = browser.new_tab()?;

        tab.navigate_to(url.as_str())?;

        tab.wait_for_element_with_custom_timeout(".into-cart", Duration::from_secs(5))?;

        Ok(tab.get_url())
    }
}
