// region:    --- Modules

mod converters;
mod error;

use converters::LinkConverter;
use lazy_regex::regex_captures_iter;
use reqwest::Client;
use url::Url;

// -- Flatten

pub use error::{Error, Result};
pub use headless_chrome::{Browser, LaunchOptions};

// endregion: --- Modules

#[derive(Debug)]
pub struct Converter {
    converters: Vec<Box<dyn LinkConverter>>,
}

impl Converter {
    pub fn builder() -> ConverterBuilder {
        ConverterBuilder::default()
    }

    pub async fn convert_one(&self, url: Url) -> Result<String> {
        for converter in &self.converters {
            if converter.can_convert(&url) {
                return converter.convert(url).await;
            }
        }

        Err(Error::NonConvertableUrl { given_url: url })
    }

    pub async fn convert_bulk(&self, text: &str) -> Result<ConversionResult> {
        let mut succeses = Vec::new();
        let mut errors = Vec::new();

        let iter = regex_captures_iter!(r"(https?://[^\s]+)", text);

        for (_, [url]) in iter.map(|c| c.extract()) {
            match Url::parse(url) {
                Ok(parsed_url) => match self.convert_one(parsed_url).await {
                    Ok(converted) => succeses.push(converted),
                    Err(e) => errors.push((url.to_string(), e)),
                },
                Err(e) => errors.push((url.to_string(), e.into())),
            }
        }

        Ok(ConversionResult {
            successes: succeses,
            errors,
        })
    }
}

#[derive(Default)]
pub struct ConverterBuilder {
    client: Option<Client>,
    browser: Option<Browser>,
}

impl ConverterBuilder {
    pub fn set_client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn set_browser(mut self, browser: Browser) -> Self {
        self.browser = Some(browser);
        self
    }

    pub fn build(self) -> Result<Converter> {
        let client = self.client.unwrap_or_else(|| Client::new());
        let browser = self.browser.unwrap_or_else(
            || Browser::new(LaunchOptions::default()).expect("Default browser should be installed"),
        );

        Ok(Converter {
            converters: vec![
                Box::new(converters::you_shop_10::YouShop10::new(browser)),
                Box::new(converters::mobile_taobao::MobileTaobao::new(client)),
            ],
        })
    }
}

#[derive(Debug)]
pub struct ConversionResult {
    pub successes: Vec<String>,
    pub errors: Vec<(String, Error)>,
}
