extern crate smtpapi;
extern crate rustc_serialize;
extern crate time;

use smtpapi::{Header, Filter};
use std::collections::HashMap;

#[test]
fn version() {
    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(smtpapi::VERSION, version);
}

#[test]
fn header_as_empty_json_string() {
    let header = Header::new();
    assert_eq!("{}", header.to_json_string());
}

#[test]
fn header_with_tos() {
    let mut header = Header::new();
    let emails = vec!["bar1@domain.com", "bar2@domain.com"];

    header.add_to("foo1@domain.com");
    assert_eq!("{\"to\":[\"foo1@domain.com\"]}", header.to_json_string());

    header.add_to("foo2@domain.com")
          .add_tos(emails);
    assert_eq!("{\"to\":[\"foo1@domain.com\",\"foo2@domain.com\",\"bar1@domain.com\",\"bar2@domain.com\"]}", header.to_json_string());
}

#[test]
fn header_with_substitution_tags() {
    let mut header = Header::new();
    let tags = vec!["rust", "power"];

    let mut all_subs : HashMap<String, Vec<String>> = HashMap::new();
    all_subs.insert("-item1-".to_string(), vec!["rust".to_string(), "power".to_string()]);
    all_subs.insert("-item2-".to_string(), vec!["rust".to_string(), "power".to_string()]);

    header.add_substitution("-top-", "foobar1");
    assert_eq!("{\"sub\":{\"-top-\":[\"foobar1\"]}}", header.to_json_string());

    header.add_substitution("-top-", "foobar2");
    assert_eq!("{\"sub\":{\"-top-\":[\"foobar1\",\"foobar2\"]}}", header.to_json_string());

    header.add_substitutions("-ztags-", tags);
    assert_eq!("{\"sub\":{\"-top-\":[\"foobar1\",\"foobar2\"],\"-ztags-\":[\"rust\",\"power\"]}}", header.to_json_string());

    header.set_substitutions(all_subs);
    assert_eq!("{\"sub\":{\"-item1-\":[\"rust\",\"power\"],\"-item2-\":[\"rust\",\"power\"]}}", header.to_json_string());
}

#[test]
fn header_with_sections_to_json_string() {
    let mut header = Header::new();
    let mut sections : HashMap<String, String> = HashMap::new();
    sections.insert("-item1-".to_string(), "value1".to_string());
    sections.insert("-item2-".to_string(), "value2".to_string());

    header.add_section("-top1-", "foobar1");

    assert_eq!("{\"section\":{\"-top1-\":\"foobar1\"}}", header.to_json_string());

    header.add_section("-top1-", "foobar2");
    assert_eq!("{\"section\":{\"-top1-\":\"foobar2\"}}", header.to_json_string());

    header.set_sections(sections);
    assert_eq!("{\"section\":{\"-item1-\":\"value1\",\"-item2-\":\"value2\"}}", header.to_json_string());
}

#[test]
fn header_with_unique_args_to_json_string() {
    let mut header = Header::new();
    let mut unique_args : HashMap<String, String> = HashMap::new();
    unique_args.insert("-arg1-".to_string(), "value1".to_string());
    unique_args.insert("-arg2-".to_string(), "value2".to_string());

    header.add_unique_arg("-arg1-", "foobar1");
    assert_eq!("{\"unique_args\":{\"-arg1-\":\"foobar1\"}}", header.to_json_string());

    header.add_unique_arg("-arg1-", "foobar2");
    assert_eq!("{\"unique_args\":{\"-arg1-\":\"foobar2\"}}", header.to_json_string());

    header.set_unique_args(unique_args);
    assert_eq!("{\"unique_args\":{\"-arg1-\":\"value1\",\"-arg2-\":\"value2\"}}", header.to_json_string());
}

#[test]
fn header_with_one_filter_to_json_string() {
    let mut header = Header::new();
    header.add_filter("clicktrack", "enabled", "1");

    assert_eq!("{\"filters\":{\"clicktrack\":{\"settings\":{\"enabled\":\"1\"}}}}", header.to_json_string());
}

#[test]
fn header_with_more_filters_to_json_string() {
    let mut header = Header::new();

    header.add_filter("clicktrack", "enabled", "1")
          .add_filter("footer", "enabled", "1")
          .add_filter("footer", "text/html", "<strong>boo</strong>");

    assert_eq!("{\"filters\":{\"clicktrack\":{\"settings\":{\"enabled\":\"1\"}},\"footer\":{\"settings\":{\"enabled\":\"1\",\"text/html\":\"<strong>boo</strong>\"}}}}", header.to_json_string());
}

#[test]
fn header_with_set_filter_to_json_string() {
    let mut header = Header::new();
    let mut filter = Filter::new();
    filter.add_setting("enabled", "1");
    header.set_filter("clicktrack", filter);

    assert_eq!("{\"filters\":{\"clicktrack\":{\"settings\":{\"enabled\":\"1\"}}}}", header.to_json_string());
}

#[test]
fn header_with_unicode_categories_to_json_string() {
    let mut header = Header::new();

    header.add_category("天破活殺");
    header.add_category("天破活殺");
    header.add_category("á");

    assert_eq!("{\"category\":[\"\u{5929}\u{7834}\u{6d3b}\u{6bba}\",\"\u{5929}\u{7834}\u{6d3b}\u{6bba}\",\"\u{e1}\"]}", header.to_json_string());
}

#[test]
fn header_with_categories_to_json_string() {
    let mut header = Header::new();

    header.add_category("category_1");
    assert_eq!("{\"category\":[\"category_1\"]}", header.to_json_string());

    header.add_category("category_2")
          .add_category("category_3");
    assert_eq!("{\"category\":[\"category_1\",\"category_2\",\"category_3\"]}", header.to_json_string());

    header.add_categories(vec!["abc", "def"]);
    assert_eq!("{\"category\":[\"category_1\",\"category_2\",\"category_3\",\"abc\",\"def\"]}", header.to_json_string());

    header.set_categories(vec!["abc", "def"]);
    assert_eq!("{\"category\":[\"abc\",\"def\"]}", header.to_json_string());

    header.set_categories(vec!["abc".to_string(), "def".to_string()]);
    assert_eq!("{\"category\":[\"abc\",\"def\"]}", header.to_json_string());
}

#[test]
fn header_with_ip_pool_to_json_string() {
    let mut header = Header::new();
    header.set_ip_pool("pool_1");

    assert_eq!("{\"ip_pool\":\"pool_1\"}", header.to_json_string());
}

#[test]
fn header_with_asm_group_id_to_json_string() {
    let mut header = Header::new();

    header.set_asm_group_id(12);
    assert_eq!("{\"asm_group_id\":12}", header.to_json_string());

    header.set_asm_group_id(123);
    assert_eq!("{\"asm_group_id\":123}", header.to_json_string());
}

#[test]
fn header_with_send_at_to_json_string() {
    let x = time::now_utc().to_timespec().sec as i64;
    let mut timestamps = Vec::new();
    timestamps.push(x);
    timestamps.push(x);

    let mut header = Header::new();
    header.set_send_each_at(timestamps);

    header.set_send_at(x);

    let mut s = "{\"send_at\":".to_string();
    s.push_str(&format!("{:?}", x));
    s.push_str("}");

    assert_eq!(s, header.to_json_string());
}

#[test]
fn header_with_send_each_at_to_json_string() {
    let x = time::now_utc().to_timespec().sec as i64;
    let y = x + 50;
    let z = x + 40;
    let timestamps = vec![x, y];

    let mut header = Header::new();

    header.set_send_at(x);

    header.set_send_each_at(timestamps)
          .add_send_each_at(z);

    let mut s = "{\"send_each_at\":[".to_string();
    s.push_str(&format!("{:?}", x));
    s.push_str(",");
    s.push_str(&format!("{:?}", y));
    s.push_str(",");
    s.push_str(&format!("{:?}", z));
    s.push_str("]}");

    assert_eq!(s, header.to_json_string());
}

#[test]
fn header_with_add_send_each_at_to_json_string() {
    let x = time::now_utc().to_timespec().sec as i64;
    let y = x + 50;
    let z = x + 40;

    let mut header = Header::new();

    header.set_send_at(x);

    header.add_send_each_at(x)
          .add_send_each_at(y)
          .add_send_each_at(z);

    let mut s = "{\"send_each_at\":[".to_string();
    s.push_str(&format!("{:?}", x));
    s.push_str(",");
    s.push_str(&format!("{:?}", y));
    s.push_str(",");
    s.push_str(&format!("{:?}", z));
    s.push_str("]}");

    assert_eq!(s, header.to_json_string());
}
