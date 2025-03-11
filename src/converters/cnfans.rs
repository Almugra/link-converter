use crate::error::Result;

use super::{destination, LinkConverter};
use async_trait::async_trait;
use url::Url;

#[derive(Debug)]
pub struct CnFans;

impl CnFans {
    pub fn new() -> Self {
        CnFans
    }
}

#[async_trait]
impl LinkConverter for CnFans {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("cnfans.com") && url.path().starts_with("/product")
    }

    async fn convert(&self, url: Url) -> Result<String> {
        // Extract query parameters
        let id = url
            .query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.to_string());

        let platform = url
            .query_pairs()
            .find(|(key, _)| key == "platform")
            .map(|(_, value)| value.to_string());

        // Convert based on platform and id
        match (platform.as_deref(), id) {
            (Some("TAOBAO"), Some(id)) => Ok(destination::taobao(&id)),
            (Some("WEIDIAN"), Some(id)) => Ok(destination::weidian(&id)),
            (Some("ALI_1688"), Some(id)) => Ok(destination::ali_1688(&id)),
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
        let url = Url::parse("https://cnfans.com/product?id=758911450758&platform=TAOBAO")?;
        let converter = CnFans::new();

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
                "https://cnfans.com/product?id=758911450758&platform=TAOBAO",
                "https://item.taobao.com/item.htm?id=758911450758",
            ),
            (
                "https://cnfans.com/product?id=7322752149&platform=WEIDIAN",
                "https://weidian.com/item.html?itemID=7322752149",
            ),
            (
                "https://cnfans.com/product?id=681296637536&platform=ALI_1688",
                "https://detail.1688.com/offer/681296637536.html",
            ),
        ];

        for (input, expected) in test_cases {
            let url = Url::parse(input)?;
            let converter = CnFans::new();

            let actual = converter.convert(url).await?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}

// endregion: --- Tests
