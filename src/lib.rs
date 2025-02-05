// region:    --- Modules

mod converters;
mod error;
mod validator;

use converters::{LinkConverter, you_shop_10::YouShop10};
use error::Result;
use url::Url;

// endregion: --- Modules

pub fn convert_to_raw(url: &Url) -> Result<String> {
    if YouShop10::can_convert(url) {
        return YouShop10::convert(url);
    }

    todo!()
}

pub fn convert_link(link: &str) -> Result<String> {
    Url::parse(link)?;

    todo!()
}
