use crate::error::{Error, Result};

use super::LinkConverter;
use async_trait::async_trait;
use url::Url;

#[derive(Debug)]
pub struct LoveGoBuy;

impl LoveGoBuy {
    pub fn new() -> Self {
        LoveGoBuy
    }
}

#[async_trait]
impl LinkConverter for LoveGoBuy {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("m.lovegobuy.com") && url.path() == "/product"
    }

    async fn convert(&self, url: Url) -> Result<String> {
        // Extract the shop_type and id query parameters
        let shop_type = url
            .query_pairs()
            .find(|(key, _)| key == "shop_type")
            .map(|(_, value)| value.to_string());

        let id = url
            .query_pairs()
            .find(|(key, _)| key == "id")
            .map(|(_, value)| value.to_string());

        // Ensure both shop_type and id are present
        let (shop_type, id) = match (shop_type, id) {
            (Some(shop_type), Some(id)) => (shop_type, id),
            _ => return Err(Error::NonConvertableUrl { given_url: url }),
        };

        // Convert based on shop_type
        match shop_type.as_str() {
            "taobao" => Ok(format!("https://item.taobao.com/item.htm?id={}", id)),
            "weidian" => Ok(format!("https://weidian.com/item.html?itemID={}", id)),
            "1688" => Ok(format!("https://detail.1688.com/offer/{}.html", id)),
            _ => Err(Error::NonConvertableUrl { given_url: url }),
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
        let url = Url::parse("https://m.lovegobuy.com/product?shop_type=taobao&id=758911450758")?;
        let converter = LoveGoBuy::new();

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
                "https://m.lovegobuy.com/product?shop_type=taobao&id=758911450758",
                "https://item.taobao.com/item.htm?id=758911450758",
            ),
            (
                "https://m.lovegobuy.com/product?shop_type=weidian&id=7322752149",
                "https://weidian.com/item.html?itemID=7322752149",
            ),
            (
                "https://m.lovegobuy.com/product?shop_type=1688&id=681296637536",
                "https://detail.1688.com/offer/681296637536.html",
            ),
        ];

        for (input, expected) in test_cases {
            let url = Url::parse(input)?;
            let converter = LoveGoBuy::new();

            let actual = converter.convert(url).await?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}

// endregion: --- Tests
