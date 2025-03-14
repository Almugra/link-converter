use link_converter::Converter;
use url::Url;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

#[tokio::test]
async fn test_converting_correct_url() -> Result<()> {
    let url = Url::parse("https://k.youshop10.com/-s=uo-wD?a=b&p=iphone&wfr=BuyercopyURL&share_relation=e0fd773efc74bec4_1651287329_1")?;

    let converter = Converter::new()?;

    let converted_url = converter.convert_one(url).await?;

    assert_eq!(
        converted_url, "https://weidian.com/item.html?itemID=7301608442",
        "url should convert correctly"
    );

    Ok(())
}

#[tokio::test]
async fn test_converting_bulk_convert_url() -> Result<()> {
    let text = "Hello https://www.rust-lang.org/ friend https://k.youshop10.com/-s=uo-wD?a=b&p=iphone&wfr=BuyercopyURL&share_relation=e0fd773efc74bec4_1651287329_1   LOL THIS IS
		what the flip https://crates.io/ asdasd";

    let converter = Converter::new()?;
    let conversion_res = converter.convert_bulk(text).await?;

    assert!(conversion_res.successes.len() == 1, "Should convert 1 URL");
    assert!(
        conversion_res.errors.len() == 2,
        "Should fail to convert 2 URL's"
    );

    Ok(())
}

#[tokio::test]
async fn test_fail_converting_wrong_url() -> Result<()> {
    let url = Url::parse("https://item.taobao.com/item.htm?id=586064449302")?;

    let converter = Converter::new()?;
    let converted_url = converter.convert_one(url.clone()).await;

    assert!(converted_url.is_err(), "url conversion should fail");

    Ok(())
}
