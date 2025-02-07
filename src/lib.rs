// region:    --- Modules

mod converters;
mod error;

use converters::LinkConverter;
use once_cell::sync::Lazy;
use url::Url;

// -- Flatten
pub use error::{Error, Result};

// endregion: --- Modules

static CONVERTERS: Lazy<Vec<Box<dyn LinkConverter>>> = Lazy::new(|| {
    vec![
        Box::new(converters::you_shop_10::YouShop10),
        Box::new(converters::mobile_taobao::Mtbcn),
    ]
});

pub fn convert_to_raw(url: Url) -> Result<String> {
    for converter in &*CONVERTERS {
        if converter.can_convert(&url) {
            return converter.convert(url);
        }
    }

    return Err(Error::NonConvertableUrl { given_url: url });
}
