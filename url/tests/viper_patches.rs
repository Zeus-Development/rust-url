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

// VIPER-PATCH(opaque-path-space)
#[test]
fn opaque_path_space_before_query_or_fragment_is_percent_encoded() {
    let url = Url::parse("non-special:opaque  ?hi").unwrap();
    assert_eq!(url.as_str(), "non-special:opaque %20?hi");
    assert_eq!(url.path(), "opaque %20");
    let url = Url::parse("non-special:opaque\t\t  \r #hi").unwrap();
    assert_eq!(url.as_str(), "non-special:opaque  %20#hi");
    let url = Url::parse("non-special:a b").unwrap();
    assert_eq!(url.as_str(), "non-special:a b");
    let mut url = Url::parse("sc:space  ?query#fragment").unwrap();
    url::quirks::set_search(&mut url, "");
    assert_eq!(url.as_str(), "sc:space %20#fragment");
    url::quirks::set_hash(&mut url, "");
    assert_eq!(url.as_str(), "sc:space %20");
}

// VIPER-PATCH(blob-origin)
#[test]
fn blob_origin_is_its_path_origin_only_for_http_https_and_file() {
    let origin = |input: &str| Url::parse(input).unwrap().origin().ascii_serialization();
    assert_eq!(
        origin("blob:https://example.org:8443/x"),
        "https://example.org:8443"
    );
    assert_eq!(origin("blob:http://example.org/x"), "http://example.org");
    for opaque in [
        "blob:blob:https://example.org/",
        "blob:ftp://host/path",
        "blob:ws://example.org/",
        "blob:wss://example.org/",
        "blob:file:///x",
        "blob:not a url",
    ] {
        assert_eq!(origin(opaque), "null", "{}", opaque);
    }
}

fn href(input: &str, base: Option<&str>) -> String {
    let base = base.map(|base| Url::parse(base).unwrap());
    Url::options()
        .base_url(base.as_ref())
        .parse(input)
        .unwrap()
        .to_string()
}

// VIPER-PATCH(file-empty-segments)
#[test]
fn file_url_keeps_leading_empty_segments() {
    assert_eq!(href("file:////foo", None), "file:////foo");
    assert_eq!(href("file://localhost////foo", None), "file://////foo");
    assert_eq!(href("file://spider///", None), "file://spider///");
    assert_eq!(href("file:.//p", None), "file:////p");
    assert_eq!(href("file:///w|/m", None), "file:///w:/m");
    assert_eq!(href("file:\\\\\\\\?fox", None), "file:////?fox");
    assert_eq!(href("file:///a/b", None), "file:///a/b");
    assert_eq!(href("file://host/a", None), "file://host/a");
}

// VIPER-PATCH(file-base-host)
#[test]
fn file_url_keeps_base_host_before_a_drive_letter() {
    let base = Some("file://host/dir/file");
    assert_eq!(href("C|", base), "file://host/C:");
    assert_eq!(href("C|/", base), "file://host/C:/");
    assert_eq!(href("/c:/foo/bar", base), "file://host/c:/foo/bar");
    assert_eq!(href("file:C:/", Some("file://host/")), "file://host/C:/");
    assert_eq!(href("/", Some("file://h/C:/a/b")), "file://h/C:/");
    assert_eq!(href("C|", Some("file:///dir/file")), "file:///C:");
}

// VIPER-PATCH(relative-slash)
#[test]
fn relative_slashes_follow_whether_url_is_special() {
    let base = Some("http://example.org/");
    assert_eq!(href("///test", base), "http://test/");
    assert_eq!(href("///\\//\\//test", base), "http://test/");
    assert_eq!(href("//test", base), "http://test/");
    let base = Some("foo://foo/a");
    assert_eq!(href("\\a", base), "foo://foo/\\a");
    assert_eq!(href("\\/a", base), "foo://foo/\\/a");
    assert_eq!(href("//a", base), "foo://a");
    assert_eq!(href("/a", base), "foo://foo/a");
}

// VIPER-PATCH(file-only-drive-letter)
#[test]
fn drive_letter_is_kept_for_file_urls_only() {
    assert_eq!(href("..", Some("abc://x/y/z/C:/")), "abc://x/y/z/");
    assert_eq!(href("..", Some("file:///C:/")), "file:///C:/");
}

// VIPER-PATCH(port-setter-tabs)
#[test]
fn port_setter_with_only_tabs_and_newlines_keeps_the_port() {
    let mut url = Url::parse("https://domain.com:3000").unwrap();
    assert!(url::quirks::set_port(&mut url, "\n\n\t\t").is_err());
    assert_eq!(url.port(), Some(3000));
    url::quirks::set_port(&mut url, "").unwrap();
    assert_eq!(url.port(), None);
}

// VIPER-PATCH(path-dot-prefix)
#[test]
fn setters_keep_the_dot_prefix_rule() {
    let mut url = Url::parse("non-spec:/").unwrap();
    url::quirks::set_pathname(&mut url, "//p");
    assert_eq!(url.as_str(), "non-spec:/.//p");
    assert_eq!(url.path(), "//p");
    url::quirks::set_pathname(&mut url, "p");
    assert_eq!(url.as_str(), "non-spec:/p");
    let mut url = Url::parse("non-spec:/.//p?q#f").unwrap();
    url::quirks::set_hostname(&mut url, "h").unwrap();
    assert_eq!(url.as_str(), "non-spec://h//p?q#f");
    assert_eq!(url.query(), Some("q"));
    let mut url = Url::parse("non-spec:/.//p").unwrap();
    url::quirks::set_hostname(&mut url, "").unwrap();
    assert_eq!(url.as_str(), "non-spec:////p");
}

// VIPER-PATCH(path-setter-null-host)
#[test]
fn empty_pathname_appends_a_segment_only_for_a_null_host() {
    let mut url = Url::parse("foo:///some/path").unwrap();
    url::quirks::set_pathname(&mut url, "");
    assert_eq!(url.as_str(), "foo://");
    let mut url = Url::parse("foo:/some/path").unwrap();
    url::quirks::set_pathname(&mut url, "");
    assert_eq!(url.as_str(), "foo:/");
}
