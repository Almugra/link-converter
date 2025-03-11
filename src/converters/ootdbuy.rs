use crate::error::Result;

use super::{destination, LinkConverter};
use async_trait::async_trait;
use url::Url;

#[derive(Debug)]
pub struct OotdBuy;

impl OotdBuy {
    pub fn new() -> Self {
        OotdBuy
    }
}

#[async_trait]
impl LinkConverter for OotdBuy {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("www.ootdbuy.com") && url.path().starts_with("/goods/details")
    }

    async fn convert(&self, url: Url) -> Result<String> {
        // Extract query parameters
        let id = url
            .query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.to_string());

        let channel = url
            .query_pairs()
            .find(|(key, _)| key == "channel")
            .map(|(_, value)| value.to_string());

        // Convert based on channel and id
        match (channel.as_deref(), id) {
            (Some("TAOBAO"), Some(id)) => Ok(destination::taobao(&id)),
            (Some("weidian"), Some(id)) => Ok(destination::weidian(&id)),
            (Some("1688"), Some(id)) => Ok(destination::ali_1688(&id)),
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
        let url =
            Url::parse("https://www.ootdbuy.com/goods/details?id=758911450758&channel=TAOBAO")?;
        let converter = OotdBuy::new();

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
                "https://www.ootdbuy.com/goods/details?id=758911450758&channel=TAOBAO",
                "https://item.taobao.com/item.htm?id=758911450758",
            ),
            (
                "https://www.ootdbuy.com/goods/details?id=7322752149&channel=weidian",
                "https://weidian.com/item.html?itemID=7322752149",
            ),
            (
                "https://www.ootdbuy.com/goods/details?id=681296637536&channel=1688",
                "https://detail.1688.com/offer/681296637536.html",
            ),
        ];

        for (input, expected) in test_cases {
            let url = Url::parse(input)?;
            let converter = OotdBuy::new();

            let actual = converter.convert(url).await?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}

// endregion: --- Tests
