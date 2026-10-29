// VIPER-PATCH(ascii-domain): Viper's patch tests, one group per ledger id in
// PATCHES.md; WPT's own cases for each are in urltestdata.json and
// setters_tests.json, run by wpt.rs.

use url::{Host, Url};

// VIPER-PATCH(ascii-domain)
#[test]
fn ascii_domain_is_lowercased_whatever_to_ascii_says() {
    let url = Url::parse("https://XN--/p").unwrap();
    assert_eq!(url.host_str(), Some("xn--"));
    let url = Url::parse("http://a.B.c.xn--pokxncvks/").unwrap();
    assert_eq!(url.host_str(), Some("a.b.c.xn--pokxncvks"));
    assert_eq!(
        Host::parse("Example.COM"),
        Ok(Host::Domain("example.com".into()))
    );
    for forbidden in ["a%25b", "a b", "a|b", "a^b", "a\u{7f}b", "a\u{1}b"] {
        assert!(Host::parse(forbidden).is_err(), "{:?}", forbidden);
    }
    assert!(Url::parse("http://%41.example/").is_ok());
    // Non-ASCII domains still go through UTS 46.
    let url = Url::parse("http://☕.example/").unwrap();
    assert_eq!(url.host_str(), Some("xn--53h.example"));
}
