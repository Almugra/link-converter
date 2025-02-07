use crate::error::Error;

use super::LinkConverter;
use lazy_regex::regex_captures;
use url::Url;

pub struct Mtbcn;

impl LinkConverter for Mtbcn {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("m.tb.cn")
    }

    fn convert(&self, url: Url) -> crate::error::Result<String> {
        let resp = reqwest::blocking::get(url.to_owned())?.text()?;

        let Some((_, item_id, shop_id)) = regex_captures!(r"(?:itemId=(\d+))|(?:shop(\d+))", &resp)
        else {
            return Err(Error::FailedToRedirectUrl { url });
        };

        if !item_id.is_empty() {
            Ok(format!("https://www.goofish.com/item?id={}", item_id))
        } else if !shop_id.is_empty() {
            Ok(format!("https://shop{}.world.taobao.com/", shop_id))
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
        let url = Url::parse("https://m.tb.cn/h.TjKAehX?tk=Jrdnecne92w")?;

        // -- Exec
        let actual_value = Mtbcn.can_convert(&url);

        // -- Check
        assert!(actual_value);

        Ok(())
    }

    #[test]
    fn test_url_conversion_goofish() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://m.tb.cn/h.TjKAehX?tk=Jrdnecne92w")?;

        // -- Exec
        let actual_converted_url = Mtbcn.convert(url)?;

        // -- Check
        let expected_converted_url = "https://www.goofish.com/item?id=713649093700";

        assert_eq!(actual_converted_url, expected_converted_url);

        Ok(())
    }

    #[test]
    fn test_url_conversion_taobao() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://m.tb.cn/h.TTHL3ZZKsh88JtB")?;

        // -- Exec
        let actual_converted_url = Mtbcn.convert(url)?;

        // -- Check
        let expected_converted_url = "https://shop247709762.world.taobao.com/";

        assert_eq!(actual_converted_url, expected_converted_url);

        Ok(())
    }
}

// endregion: --- Tests
