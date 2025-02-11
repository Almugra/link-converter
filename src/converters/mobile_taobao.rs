use crate::error::Error;

use super::LinkConverter;
use lazy_regex::regex_captures;
use reqwest::blocking::Client;
use url::Url;

pub struct Mtbcn(Client);

impl Mtbcn {
    pub fn new(client: Client) -> Self {
        Mtbcn(client)
    }
}

impl LinkConverter for Mtbcn {
    fn can_convert(&self, url: &Url) -> bool {
        url.host_str() == Some("m.tb.cn")
    }

    fn convert(&self, url: Url) -> crate::error::Result<String> {
        let resp = self.0.get(url.to_owned()).send()?.text()?;

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
        let converter = Mtbcn::new(Client::new());

        // -- Exec
        let actual_value = converter.can_convert(&url);

        // -- Check
        assert!(actual_value);

        Ok(())
    }

    #[test]
    fn test_url_conversion_taobao() -> Result<()> {
        // -- Setup & Fixtures
        let url = Url::parse("https://m.tb.cn/h.TTHL3ZZKsh88JtB")?;
        let converter = Mtbcn::new(Client::new());

        // -- Exec
        let actual_converted_url = converter.convert(url)?;

        // -- Check
        let expected_converted_url = "https://shop247709762.world.taobao.com/";

        assert_eq!(actual_converted_url, expected_converted_url);

        Ok(())
    }
}

// endregion: --- Tests
