use crate::error::Result;

use super::{destination, LinkConverter};
use async_trait::async_trait;
use url::Url;

#[derive(Debug)]
pub struct JoyaBuy;

impl JoyaBuy {
    pub fn new() -> Self {
        JoyaBuy
    }
}

#[async_trait]
impl LinkConverter for JoyaBuy {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("joyabuy.com") && url.path().starts_with("/product/")
    }

    async fn convert(&self, url: Url) -> Result<String> {
        // Extract query parameters
        let shop_type = url
            .query_pairs()
            .find(|(key, _)| key == "shop_type")
            .map(|(_, value)| value.to_string());

        let id = url
            .query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.to_string());

        // Convert based on shop_type and id
        match (shop_type.as_deref(), id) {
            (Some("taobao"), Some(id)) => Ok(destination::taobao(&id)),
            (Some("weidian"), Some(id)) => Ok(destination::weidian(&id)),
            (Some("ali_1688"), Some(id)) => Ok(destination::ali_1688(&id)),
            _ => Err(crate::Error::NonConvertableUrl { given_url: url }),
        }
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>;

    use url::Url;

    use super::*;

    #[test]
    fn test_detects_convertable_url() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://joyabuy.com/product/?shop_type=taobao&id=758911450758")?;
        let converter = JoyaBuy::new();

        // -- Exec
        let actual_value = converter.can_convert(&url);

        // -- Check
        assert!(actual_value);

        Ok(())
    }

    #[tokio::test]
    async fn test_url_conversion() -> Result<()> {
        // -- Patterns
        let test_cases = [
            (
                "https://joyabuy.com/product/?shop_type=taobao&id=758911450758",
                "https://item.taobao.com/item.htm?id=758911450758",
            ),
            (
                "https://joyabuy.com/product/?shop_type=weidian&id=7322752149",
                "https://weidian.com/item.html?itemID=7322752149",
            ),
            (
                "https://joyabuy.com/product/?shop_type=ali_1688&id=681296637536",
                "https://detail.1688.com/offer/681296637536.html",
            ),
        ];

        for (input, expected) in test_cases {
            let url = Url::parse(input)?;
            let converter = JoyaBuy::new();

            let actual = converter.convert(url).await?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}

// endregion: --- Tests
