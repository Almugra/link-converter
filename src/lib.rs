// region:    --- Modules

mod converters;
mod error;

use converters::LinkConverter;
use lazy_regex::regex_captures_iter;
use reqwest::{redirect::Policy, Client};
use url::Url;

// -- Flatten

pub use error::{Error, Result};

// endregion: --- Modules

/// Main Converter struct holding registered URL conversion strategies.
#[derive(Debug)]
pub struct Converter {
    converters: Vec<Box<dyn LinkConverter>>,
}

impl Converter {
    /// Creates a new Converter instance with default HTTP client settings.
    ///
    /// # Errors
    /// Returns `Error` if the underlying HTTP client fails to build.
    pub fn new() -> Result<Self> {
        let client = Client::builder().redirect(Policy::limited(10)).build()?;
        Ok(Self {
            converters: Self::init_converters(client),
        })
    }

    /// Creates a new Converter using a pre-configured HTTP client.
    ///
    /// Its suggested to set a redirect Policy.
    pub fn from_client(client: Client) -> Self {
        Self {
            converters: Self::init_converters(client),
        }
    }

    /// Initializes the converters
    fn init_converters(client: Client) -> Vec<Box<dyn LinkConverter>> {
        vec![
            Box::new(converters::you_shop_10::YouShop10::new(client.clone())),
            Box::new(converters::mobile_taobao::MobileTaobao::new(client.clone())),
            Box::new(converters::mobile_intl_taobao::MobileIntlTaobao),
            Box::new(converters::cssbuy::CSSBuy::new()),
            Box::new(converters::lovegobuy::LoveGoBuy::new()),
            Box::new(converters::mulebuy::MuleBuy::new()),
            Box::new(converters::ootdbuy::OotdBuy::new()),
            Box::new(converters::cnfans::CnFans::new()),
            Box::new(converters::orientdig::OrientDig::new()),
            Box::new(converters::oopbuy::OopBuy::new()),
            Box::new(converters::joyabuy::JoyaBuy::new()),
        ]
    }

    /// Converts a single URL using the first applicable converter.
    ///
    /// # Parameters
    /// - `url`: The URL to be converted
    ///
    /// # Errors
    /// Returns `Error::NonConvertableUrl` if no registered converter can handle the URL.
    pub async fn convert_one(&self, url: Url) -> Result<String> {
        for converter in &self.converters {
            if converter.can_convert(&url) {
                return converter.convert(url).await;
            }
        }

        Err(Error::NonConvertableUrl { given_url: url })
    }

    /// Processes text content to find and convert all HTTP/HTTPS URLs. Returns a [`ConversionResult`].
    ///
    /// # Note
    /// URL detection uses a simple regex pattern (`https?://[^\s]+`). Some valid URLs
    /// might not be detected if they contain spaces or unusual formatting.
    pub async fn convert_bulk(&self, text: &str) -> Result<ConversionResult> {
        let mut successes = Vec::new();
        let mut errors = Vec::new();

        let iter = regex_captures_iter!(r"(https?://[^\s]+)", text);

        for (_, [url]) in iter.map(|c| c.extract()) {
            match Url::parse(url) {
                Ok(parsed_url) => match self.convert_one(parsed_url).await {
                    Ok(converted) => successes.push(converted),
                    Err(e) => errors.push((url.to_string(), e)),
                },
                Err(e) => errors.push((url.to_string(), e.into())),
            }
        }

        Ok(ConversionResult { successes, errors })
    }
}

/// Result container for bulk conversion operations.
#[derive(Debug)]
pub struct ConversionResult {
    /// Successfully converted URLs in the order they were found
    pub successes: Vec<String>,
    /// Conversion failures with original URL and error details
    pub errors: Vec<(String, Error)>,
}
