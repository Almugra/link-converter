use link_converter::ConverterBuilder;
use url::Url;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

#[test]
fn test_converting_correct_url() -> Result<()> {
    let url = Url::parse("https://k.youshop10.com/-s=uo-wD?a=b&p=iphone&wfr=BuyercopyURL&share_relation=e0fd773efc74bec4_1651287329_1")?;

    let converter = ConverterBuilder::default().build().unwrap();

    let converted_url = converter.convert_one(url)?;

    assert_eq!(
        converted_url, "https://weidian.com/item.html?itemID=7301608442",
        "url should convert correctly"
    );

    Ok(())
}

#[test]
fn test_converting_bulk_convert_url() -> Result<()> {
    let text = "Hello https://www.rust-lang.org/ friend https://k.youshop10.com/-s=uo-wD?a=b&p=iphone&wfr=BuyercopyURL&share_relation=e0fd773efc74bec4_1651287329_1   LOL THIS IS
		what the flip https://crates.io/ asdasd";

    let converter = ConverterBuilder::default().build().unwrap();
    let conversion_res = converter.convert_bulk(text)?;

    dbg!(&conversion_res);

    assert!(conversion_res.succeses.len() == 1, "Should convert 1 URL");
    assert!(
        conversion_res.errors.len() == 2,
        "Should fail to convert 2 URL's"
    );

    Ok(())
}

#[test]
fn test_fail_converting_wrong_url() -> Result<()> {
    let url = Url::parse("https://item.taobao.com/item.htm?id=586064449302")?;

    let converter = ConverterBuilder::default().build().unwrap();
    let converted_url = converter.convert_one(url.clone());

    assert!(converted_url.is_err(), "url conversion should fail");

    Ok(())
}
