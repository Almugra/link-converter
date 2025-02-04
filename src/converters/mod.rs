use crate::Result;
use url::Url;

pub mod YouShop10;

/// A trait that defines how to convert a link into its raw form
pub trait LinkConverter {
    /// Checks if this converter can handle the given URL.
    fn can_convert(url: &Url) -> bool;

    /// Converts the URL into its raw form;
    fn convert(url: &Url) -> Result<String>;
}
