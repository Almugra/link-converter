use crate::error::Result;

use super::LinkConverter;
use async_trait::async_trait;
use lazy_regex::regex_captures;
use url::Url;

#[derive(Debug)]
pub struct CSSBuy;

impl CSSBuy {
    pub fn new() -> Self {
        CSSBuy
    }
}

#[async_trait]
impl LinkConverter for CSSBuy {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("www.cssbuy.com") && url.path().starts_with("/item-")
    }

    async fn convert(&self, url: Url) -> Result<String> {
        let path = url.path();

        // Pattern 1: item-{id}.html -> item.taobao.com/item.htm?id={id}
        if let Some((_, id)) = regex_captures!(r"^/item-(\d+)\.html$", path) {
            return Ok(format!("https://item.taobao.com/item.htm?id={}", id));
        }

        // Pattern 2: item-micro-{id}.html -> weidian.com/item.html?itemID={id}
        if let Some((_, id)) = regex_captures!(r"^/item-micro-(\d+)\.html$", path) {
            return Ok(format!("https://weidian.com/item.html?itemID={}", id));
        }

        // Pattern 3: item-1688-{id}.html -> detail.1688.com/offer/{id}.html
        if let Some((_, id)) = regex_captures!(r"^/item-1688-(\d+)\.html$", path) {
            return Ok(format!("https://detail.1688.com/offer/{}.html", id));
        }

        Err(crate::Error::NonConvertableUrl { given_url: url })
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
        let url = Url::parse("https://www.cssbuy.com/item-758911450758.html")?;
        let converter = CSSBuy::new();

        // -- Exec
        let actual_value = converter.can_convert(&url);

        // -- Check
        assert!(actual_value);

        Ok(())
    }

    #[tokio::test]
    async fn test_url_conversion_taobao() -> Result<()> {
        // -- Patterns
        let test_cases = [
            (
                "https://www.cssbuy.com/item-758911450758.html",
                "https://item.taobao.com/item.htm?id=758911450758",
            ),
            (
                "https://www.cssbuy.com/item-micro-7322752149.html",
                "https://weidian.com/item.html?itemID=7322752149",
            ),
            (
                "https://www.cssbuy.com/item-1688-681296637536.html",
                "https://detail.1688.com/offer/681296637536.html",
            ),
        ];

        for (input, expected) in test_cases {
            let url = Url::parse(input)?;
            let converter = CSSBuy::new();

            let actual = converter.convert(url).await?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}

// endregion: --- Tests
