use crate::error::Result;

use super::{destination, LinkConverter};
use async_trait::async_trait;
use url::Url;

#[derive(Debug)]
pub struct OopBuy;

impl OopBuy {
    pub fn new() -> Self {
        OopBuy
    }
}

#[async_trait]
impl LinkConverter for OopBuy {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("oopbuy.com") && url.path().starts_with("/product/")
    }

    async fn convert(&self, url: Url) -> Result<String> {
        // Extract path segments
        let path = url.path();

        // Expected format: /product/{shop_type}/{id}
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if segments.len() >= 3 && segments[0] == "product" {
            let shop_type = segments[1];
            let id = segments[2];

            match shop_type {
                "1" => {
                    // Taobao
                    Ok(destination::taobao(id))
                }
                "weidian" => {
                    // Weidian
                    Ok(destination::weidian(id))
                }
                "0" => {
                    // 1688
                    Ok(destination::ali_1688(id))
                }
                _ => Err(crate::Error::NonConvertableUrl { given_url: url }),
            }
        } else {
            Err(crate::Error::NonConvertableUrl { given_url: url })
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
        let url = Url::parse("https://oopbuy.com/product/1/758911450758")?;
        let converter = OopBuy::new();

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
                "https://oopbuy.com/product/1/758911450758",
                "https://item.taobao.com/item.htm?id=758911450758",
            ),
            (
                "https://oopbuy.com/product/weidian/7322752149",
                "https://weidian.com/item.html?itemID=7322752149",
            ),
            (
                "https://oopbuy.com/product/0/681296637536",
                "https://detail.1688.com/offer/681296637536.html",
            ),
        ];

        for (input, expected) in test_cases {
            let url = Url::parse(input)?;
            let converter = OopBuy::new();

            let actual = converter.convert(url).await?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}

// endregion: --- Tests
