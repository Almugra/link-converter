// region:    --- Modules

mod converters;
mod error;

use converters::LinkConverter;
use lazy_regex::regex_captures_iter;
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

#[derive(Debug)]
pub struct ConversionResult {
    pub succeses: Vec<String>,
    pub errors: Vec<(String, Error)>,
}

pub fn convert_bulk(text: &str) -> Result<ConversionResult> {
    let mut succeses = Vec::new();
    let mut errors = Vec::new();

    let iter = regex_captures_iter!(r"(https?://[^\s]+)", text);

    for (_, [url]) in iter.map(|c| c.extract()) {
        match Url::parse(url) {
            Ok(parsed_url) => match convert_to_raw(parsed_url) {
                Ok(converted) => succeses.push(converted),
                Err(e) => errors.push((url.to_string(), e)),
            },
            Err(e) => errors.push((url.to_string(), e.into())),
        }
    }

    Ok(ConversionResult { succeses, errors })
}
