use crate::error::Error;

use super::{destination, LinkConverter};
use async_trait::async_trait;
use lazy_regex::regex_captures;
use url::Url;

#[derive(Debug)]
pub struct MobileIntlTaobao;

#[async_trait]
impl LinkConverter for MobileIntlTaobao {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("m.intl.taobao.com")
    }

    async fn convert(&self, url: Url) -> crate::error::Result<String> {
        let Some((_, item_id)) = regex_captures!(r"(?:id=(\d+))", &url.as_str()) else {
            return Err(Error::FailedToRedirectUrl { url });
        };

        if !item_id.is_empty() {
            Ok(destination::taobao(item_id))
        } else {
            Err(Error::FailedToRedirectUrl { url })
        }
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use url::Url;

    use super::*;

    #[test]
    fn test_detects_convertable_url() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://m.intl.taobao.com/detail/detail.html?id=635308355125")?;

        // -- Exec
        let actual_value = MobileIntlTaobao.can_convert(&url);

        // -- Check
        assert!(actual_value);

        Ok(())
    }

    #[tokio::test]
    async fn test_url_conversion() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://m.intl.taobao.com/detail/detail.html?id=635308355125")?;

        // -- Exec
        let actual_converted_url = MobileIntlTaobao.convert(url).await?;

        // -- Check
        let expected_converted_url = "https://item.taobao.com/item.htm?id=635308355125";

        assert_eq!(actual_converted_url, expected_converted_url);

        Ok(())
    }
}

// endregion: --- Tests
