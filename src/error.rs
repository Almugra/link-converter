use derive_more::{Display, From};
use url::Url;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    NonConvertableUrl {
        given_url: Url,
    },
    FailedToRedirectUrl {
        url: Url,
    },

    // -- Externals
    #[from]
    Reqwest(reqwest::Error),
    #[from]
    HeadlessChrome(anyhow::Error),
}

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
