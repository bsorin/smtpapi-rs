extern crate rustc_serialize;

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;
use rustc_serialize::json::{ToJson, Json};
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub static VERSION: &'static str = "0.1.0";

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Filter {
    settings: HashMap<String, String>
}

impl fmt::Display for Filter {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        self.settings.clear();
    }
}

impl Filter {
    /// Constructs a new `Filter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Filter};
    ///
    /// let filter = Filter::new();
    /// println!("{}", filter.to_string());
    /// ```
    pub fn new() -> Filter {
        Filter { settings: HashMap::new() }
    }

    /// Add settings for a `Filter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Filter};
    ///
    /// let mut filter = Filter::new();
    /// filter.add_setting("enabled", "1");
    /// filter.add_setting("text", "some text");
    ///
    /// println!("{}", filter.to_string());
    /// ```
    pub fn add_setting<S>(&mut self, setting: S, value: S) -> &mut Filter where S: Into<String> {
        self.settings.insert(setting.into(), value.into());
        self
    }

    /// Returns the JSON String reprezentation of `Filter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Filter};
    ///
    /// let filter = Filter::new();
    /// println!("{}", filter.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        return self.to_json().to_string();
    }
}

impl ToJson for Filter {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        if !self.settings.is_empty() {
            d.insert("settings".to_string(), self.settings.to_json());
        }

        Json::Object(d)
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Header {
    to: Vec<String>,
    sub: HashMap<String, Vec<String>>,
    section: HashMap<String, String>,
    categories: Vec<String>,
    unique_args: HashMap<String, String>,
    filters: HashMap<String, Filter>,
    asm_group_id: Option<i32>,
    send_at: Option<i64>,
    send_each_at: Option<Vec<i64>>,
    ip_pool: Option<String>
}

impl Drop for Header {
    fn drop(&mut self) {
        self.to.clear();
        self.sub.clear();
        self.section.clear();
        self.categories.clear();
        self.unique_args.clear();
        self.filters.clear();
    }
}

impl ToJson for Header {
    /// Constructs the JSON reprezentation of `Header`.
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        if !self.to.is_empty() {
            d.insert("to".to_string(), self.to.to_json());
        }

        if !self.sub.is_empty() {
            d.insert("sub".to_string(), self.sub.to_json());
        }

        if !self.section.is_empty() {
            d.insert("section".to_string(), self.section.to_json());
        }

        if !self.categories.is_empty() {
            d.insert("category".to_string(), self.categories.to_json());
        }

        if !self.unique_args.is_empty() {
            d.insert("unique_args".to_string(), self.unique_args.to_json());
        }

        if !self.filters.is_empty() {
            d.insert("filters".to_string(), self.filters.to_json());
        }

        if let Some(ref x) = self.asm_group_id {
            d.insert("asm_group_id".to_string(), x.to_json());
        }



        if let Some(ref x) = self.send_at {
            d.insert("send_at".to_string(), x.to_json());
        }

        if let Some(ref x) = self.send_each_at {
            d.insert("send_each_at".to_string(), x.to_json());
        }

        if let Some(ref x) = self.ip_pool {
            d.insert("ip_pool".to_string(), x.to_json());
        }

        Json::Object(d)
    }
}

/// Implement Display for Header
impl fmt::Display for Header {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "b\"{}\"", self.to_string())
        // let x = self.to_string();
        // let y = self.escape_bytestring(x.as_bytes().as_ref())
        // write!(f, "b\"{}\"", y);
    }
}

impl Header {
    /// Constructs a new `Header`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let header = Header::new();
    /// println!("{}", header.to_string());
    /// ```
    pub fn new() -> Header {
        Header { to: Vec::new(),
                 sub: HashMap::new(),
                 section: HashMap::new(),
                 categories: Vec::new(),
                 unique_args: HashMap::new(),
                 filters: HashMap::new(),
                 asm_group_id: None,
                 send_at: None,
                 send_each_at: None,
                 ip_pool: None
               }
    }

    /// Returns the JSON String reprezentation of `Header`.
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let header = Header::new();
    /// println!("{}", header.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        return self.to_json().to_string();
    }

    /// It appends a single email to the To header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_to("email@domain.com");
    /// ```
    pub fn add_to<S>(&mut self, email: S) -> &mut Header where S: Into<String> {
        self.to.push(email.into());
        self
    }

    /// It appends multiple emails to the To header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_tos(vec!["email1@domain.com", "email2@domain.com"]);
    /// ```
    pub fn add_tos<S>(&mut self, emails: Vec<S>) -> &mut Header where S: Into<String> {
        for email in emails {
            self.to.push(email.into())
        }

        self
    }

    /// It sets the value of the To header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.set_tos(vec!["email1@domain.com", "email2@domain.com"]);
    /// ```
    pub fn set_tos<S>(&mut self, emails: Vec<S>) -> &mut Header where S: Into<String> {
        self.to = Vec::new();
        for email in emails {
            self.to.push(email.into())
        }

        self
    }

    /// It adds a new substitution to a specific key
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_substitution("[name]", "my_name");
    /// ```
    pub fn add_substitution<S>(&mut self, key: S, sub: S) -> &mut Header where S: Into<String> {
        match self.sub.entry(key.into()) {
            Vacant(entry) => {
                let mut subs = Vec::new();
                subs.push(sub.into());
                entry.insert( subs );
            },
            Occupied(entry) => entry.into_mut().push(sub.into()),
        };

        self
    }

    /// It adds a multiple substitutions to a specific key
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_substitutions("[name]", vec!["my_name_1", "my_name_2"]);
    /// ```
    pub fn add_substitutions<S>(&mut self, key: S, subs: Vec<&str>) -> &mut Header where S: Into<String> {
        match self.sub.entry(key.into()) {
            Vacant(entry) => { entry.insert(subs.iter().map(|&x| x.to_string()).collect::<Vec<String>>()); },
            Occupied(entry) => { entry.into_mut().extend(subs.iter().map(|&x| x.to_string()).collect::<Vec<String>>().iter().cloned()); },
        };

        self
    }

    /// It sets the value of the substitutions on the Sub header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    /// use std::collections::HashMap;
    ///
    /// let mut header = Header::new();
    /// let mut all_subs : HashMap<String, Vec<String>> = HashMap::new();
    ///
    /// all_subs.insert("-item1-".to_string(), vec!["rust".to_string(), "power".to_string()]);
    /// all_subs.insert("-item2-".to_string(), vec!["rust".to_string(), "power".to_string()]);
    ///
    /// header.set_substitutions(all_subs);
    /// ```
    pub fn set_substitutions(&mut self, subs: HashMap<String, Vec<String>>) -> &mut Header {
        self.sub = subs;

        self
    }

    /// It sets the value for a specific section
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_section("-top-", "sample");
    /// ```
    pub fn add_section<S>(&mut self, section: S, value: S) -> &mut Header where S: Into<String> {
        self.section.insert(section.into(), value.into());

        self
    }

    /// It sets the value for the Section header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    /// use std::collections::HashMap;
    ///
    /// let mut header = Header::new();
    /// let mut sections : HashMap<String, String> = HashMap::new();
    /// sections.insert("-item1-".to_string(), "value1".to_string());
    /// sections.insert("-item2-".to_string(), "value2".to_string());
    ///
    /// header.set_sections(sections);
    /// ```
    pub fn set_sections(&mut self, sections: HashMap<String, String>) -> &mut Header {
        self.section = sections;

        self
    }

    /// It adds a new category to the Category header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_category("welcome");
    /// ```
    pub fn add_category<S>(&mut self, category: S) -> &mut Header where S: Into<String> {
        // let x = ASCII.encode(&category.into(), EncoderTrap::Ignore).unwrap();
        // println!("{:?}", String::from_utf8(x).unwrap());
        // self.categories.push(std::str::from_utf8(&x).unwrap().to_string());
        self.categories.push(category.into());
        self
    }

    /// It adds multiple categories to the Category header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_categories(vec!["welcome", "new_accounts"]);
    /// ```
    pub fn add_categories<S>(&mut self, categories: Vec<S>) -> &mut Header where S: Into<String> {
        for category in categories {
            self.categories.push(category.into());
        }

        self
    }

    /// It sets the value of the Categories field
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.set_categories(vec!["welcome", "new_accounts"]);
    /// ```
    pub fn set_categories<S>(&mut self, categories: Vec<S>) -> &mut Header where S: Into<String> {
        self.categories = Vec::new();
        for category in categories {
            self.categories.push(category.into());
        }

        self
    }

    /// It sets the value of a specific unique argument
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_unique_arg("account_id", "123412-121-1212");
    /// ```
    pub fn add_unique_arg<S>(&mut self, unique_arg: S, value: S) -> &mut Header where S: Into<String> {
        self.unique_args.insert(unique_arg.into(), value.into());

        self
    }

    /// It will set the value of the Unique_args header
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    /// use std::collections::HashMap;
    ///
    /// let mut header = Header::new();
    /// let mut unique_args : HashMap<String, String> = HashMap::new();
    /// unique_args.insert("-arg1-".to_string(), "value1".to_string());
    /// unique_args.insert("-arg2-".to_string(), "value2".to_string());
    ///
    /// header.set_unique_args(unique_args);
    /// ```
    pub fn set_unique_args(&mut self, unique_args: HashMap<String, String>) -> &mut Header {
        self.unique_args = unique_args;

        self
    }

    /// It will set the specific setting for a filter
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    /// header.add_filter("clicktrack", "enabled", "1")
    ///       .add_filter("opentrack", "enabled", "1");
    /// ```
    pub fn add_filter<S>(&mut self, filter_name: S, setting: S, value: S) -> &mut Header where S: Into<String> {
        match self.filters.entry(filter_name.into()) {
            Vacant(entry) => {
                let mut filter = Filter::new();
                filter.add_setting(setting, value);
                entry.insert(filter);
            },
            Occupied(entry) => { entry.into_mut().settings.insert(setting.into(), value.into()); },
        };

        self
    }

    /// It takes in a Filter struct with predetermined settings and sets it for such Filter key
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header, Filter};
    ///
    /// let mut header = Header::new();
    /// let mut filter = Filter::new();
    /// filter.add_setting("enabled", "1");
    ///
    /// header.set_filter("clicktrack", filter);
    /// ```
    pub fn set_filter<S>(&mut self, filter: S, setting: Filter) -> &mut Header where S: Into<String> {
        self.filters.insert(filter.into(), setting);
        self
    }

    /// It sets the value of the IpPool field
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    ///
    /// header.set_ip_pool("newsletter_pool");
    /// ```
    pub fn set_ip_pool<S>(&mut self, name: S) -> &mut Header where S: Into<String> {
        self.ip_pool = Some(name.into());
        self
    }

    /// It will set the value of the ASMGroupID field
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    ///
    /// header.set_asm_group_id(1221);
    /// ```
    pub fn set_asm_group_id(&mut self, asm_group_id: i32) -> &mut Header {
        self.asm_group_id = Some(asm_group_id);
        self
    }

    /// It takes in a timestamp which determines when the email will be sent
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    ///
    /// header.set_send_at(1453213937);
    /// ```
    pub fn set_send_at(&mut self, send_at: i64) -> &mut Header {
        self.send_at = Some(send_at);
        self.send_each_at = None;
        self
    }

    /// It takes in a timestamp and pushes it into a list. Must match length of To emails
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    ///
    /// header.add_send_each_at(1453213937)
    ///       .add_send_each_at(1453213939);
    /// ```
    pub fn add_send_each_at(&mut self, send_at: i64) -> &mut Header {
        if let Some(ref mut x) = self.send_each_at {
            x.push(send_at);
        }
        self.send_at = None;
        self
    }

    /// It takes an array of timestamps. Must match length of To emails
    ///
    /// # Examples
    ///
    /// ```
    /// use smtpapi::{Header};
    ///
    /// let mut header = Header::new();
    ///
    /// header.set_send_each_at(vec![1453213939, 1453213932, 1453213933]);
    /// ```
    pub fn set_send_each_at(&mut self, send_each_at: Vec<i64>) -> &mut Header {
        self.send_each_at = Some(send_each_at);
        self.send_at = None;
        self
    }
}

#[cfg(test)]
mod tests {
    extern crate time;

    use super::*;
    use std::collections::HashMap;
    use super::rustc_serialize::json::ToJson;

    #[test]
    fn to_string() {
        let header = Header::new();

        assert_eq!("{}", header.to_string());
    }

    #[test]
    fn header_to_empty_json_string() {
        let header = Header::new();

        assert_eq!("{}", header.to_string());
    }

    #[test]
    fn header_with_tos_to_json_string() {
        let mut header = Header::new();
        let emails = vec!["bar1@domain.com", "bar2@domain.com"];
        let emails2 = vec!["foo1@domain.com", "foo2@domain.com"];

        header.add_to("foo1@domain.com");

        assert_eq!(header.to.len(), 1);
        assert_eq!("{\"to\":[\"foo1@domain.com\"]}", header.to_string());

        header.add_to("foo2@domain.com")
              .add_tos(emails);

        assert_eq!(header.to.len(), 4);
        assert_eq!("{\"to\":[\"foo1@domain.com\",\"foo2@domain.com\",\"bar1@domain.com\",\"bar2@domain.com\"]}", header.to_string());

        header.set_tos(emails2);
        assert_eq!(header.to.len(), 2);
        assert_eq!("{\"to\":[\"foo1@domain.com\",\"foo2@domain.com\"]}", header.to_string());
    }

    #[test]
    fn header_with_subs_to_json_string() {
        let mut header = Header::new();
        let tags = vec!["rust", "power"];

        let mut all_subs : HashMap<String, Vec<String>> = HashMap::new();
        all_subs.insert("-item1-".to_string(), vec!["rust".to_string(), "power".to_string()]);
        all_subs.insert("-item2-".to_string(), vec!["rust".to_string(), "power".to_string()]);

        header.add_substitution("-top-", "foobar1");

        assert_eq!(true, header.sub.contains_key("-top-"));
        assert_eq!("{\"sub\":{\"-top-\":[\"foobar1\"]}}", header.to_string());

        header.add_substitution("-top-", "foobar2");
        assert_eq!("{\"sub\":{\"-top-\":[\"foobar1\",\"foobar2\"]}}", header.to_string());

        header.add_substitutions("-ztags-", tags);
        assert_eq!(true, header.sub.contains_key("-ztags-"));
        assert_eq!("{\"sub\":{\"-top-\":[\"foobar1\",\"foobar2\"],\"-ztags-\":[\"rust\",\"power\"]}}", header.to_string());

        header.set_substitutions(all_subs);
        assert_eq!(true, header.sub.contains_key("-item1-"));
        assert_eq!(true, header.sub.contains_key("-item2-"));
        assert_eq!("{\"sub\":{\"-item1-\":[\"rust\",\"power\"],\"-item2-\":[\"rust\",\"power\"]}}", header.to_string());

    }

    #[test]
    fn header_with_sections_to_json_string() {
        let mut header = Header::new();
        let mut sections : HashMap<String, String> = HashMap::new();
        sections.insert("-item1-".to_string(), "value1".to_string());
        sections.insert("-item2-".to_string(), "value2".to_string());

        header.add_section("-top1-", "foobar1");

        assert_eq!(true, header.section.contains_key("-top1-"));
        assert_eq!("{\"section\":{\"-top1-\":\"foobar1\"}}", header.to_string());

        header.add_section("-top1-", "foobar2");
        assert_eq!("{\"section\":{\"-top1-\":\"foobar2\"}}", header.to_string());

        header.set_sections(sections);
        assert_eq!(true, header.section.contains_key("-item1-"));
        assert_eq!(true, header.section.contains_key("-item2-"));
        assert_eq!("{\"section\":{\"-item1-\":\"value1\",\"-item2-\":\"value2\"}}", header.to_string());
    }

    #[test]
    fn header_with_unique_args_to_json_string() {
        let mut header = Header::new();
        let mut unique_args : HashMap<String, String> = HashMap::new();
        unique_args.insert("-arg1-".to_string(), "value1".to_string());
        unique_args.insert("-arg2-".to_string(), "value2".to_string());

        header.add_unique_arg("-arg1-", "foobar1");

        assert_eq!(true, header.unique_args.contains_key("-arg1-"));
        assert_eq!("{\"unique_args\":{\"-arg1-\":\"foobar1\"}}", header.to_string());

        header.add_unique_arg("-arg1-", "foobar2");
        assert_eq!("{\"unique_args\":{\"-arg1-\":\"foobar2\"}}", header.to_string());

        header.set_unique_args(unique_args);
        assert_eq!(true, header.unique_args.contains_key("-arg1-"));
        assert_eq!(true, header.unique_args.contains_key("-arg2-"));
        assert_eq!("{\"unique_args\":{\"-arg1-\":\"value1\",\"-arg2-\":\"value2\"}}", header.to_string());
    }

    #[test]
    fn header_with_one_filter_to_json_string() {
        let mut header = Header::new();
        header.add_filter("clicktrack", "enabled", "1");

        assert_eq!(true, header.filters.contains_key("clicktrack"));
        assert_eq!("{\"filters\":{\"clicktrack\":{\"settings\":{\"enabled\":\"1\"}}}}", header.to_json().to_string());
    }

    #[test]
    fn header_with_more_filters_to_json_string() {
        let mut header = Header::new();

        header.add_filter("clicktrack", "enabled", "1")
              .add_filter("footer", "enabled", "1")
              .add_filter("footer", "text/html", "<strong>boo</strong>");

        assert_eq!(true, header.filters.contains_key("clicktrack"));
        assert_eq!(true, header.filters.contains_key("footer"));
        assert_eq!("{\"filters\":{\"clicktrack\":{\"settings\":{\"enabled\":\"1\"}},\"footer\":{\"settings\":{\"enabled\":\"1\",\"text/html\":\"<strong>boo</strong>\"}}}}", header.to_json().to_string());
    }

    #[test]
    fn header_with_set_filter_to_json_string() {
        let mut header = Header::new();
        let mut filter = Filter::new();
        filter.add_setting("enabled", "1");
        header.set_filter("clicktrack", filter);

        assert_eq!(true, header.filters.contains_key("clicktrack"));
        assert_eq!("{\"filters\":{\"clicktrack\":{\"settings\":{\"enabled\":\"1\"}}}}", header.to_string());
    }

    #[test]
    fn header_with_unicode_categories_to_json_string() {
        let mut header = Header::new();

        header.add_category("天破活殺");
        header.add_category("天破活殺");

        assert_eq!("{\"category\":[\"\u{5929}\u{7834}\u{6d3b}\u{6bba}\",\"\u{5929}\u{7834}\u{6d3b}\u{6bba}\"]}", header.to_string());
    }

    #[test]
    fn header_with_categories_to_json_string() {
        let mut header = Header::new();

        header.add_category("category_1");
        assert_eq!("{\"category\":[\"category_1\"]}", header.to_string());

        header.add_category("category_2")
              .add_category("category_3");
        assert_eq!("{\"category\":[\"category_1\",\"category_2\",\"category_3\"]}", header.to_string());

        header.add_categories(vec!["abc", "def"]);
        assert_eq!("{\"category\":[\"category_1\",\"category_2\",\"category_3\",\"abc\",\"def\"]}", header.to_string());

        header.set_categories(vec!["abc", "def"]);
        assert_eq!("{\"category\":[\"abc\",\"def\"]}", header.to_string());
    }

    #[test]
    fn header_with_ip_pool_to_json_string() {
        let mut header = Header::new();
        header.set_ip_pool("pool_1");

        assert_eq!("{\"ip_pool\":\"pool_1\"}", header.to_string());
    }

    #[test]
    fn header_with_asm_group_id_to_json_string() {
        let mut header = Header::new();

        header.set_asm_group_id(12);
        assert_eq!("{\"asm_group_id\":12}", header.to_string());

        header.set_asm_group_id(123);
        assert_eq!("{\"asm_group_id\":123}", header.to_string());
    }

    #[test]
    fn header_with_send_at_to_json_string() {
        let x = time::now_utc().to_timespec().sec as i64;
        let mut timestamps = Vec::new();
        timestamps.push(x);
        timestamps.push(x);

        let mut header = Header::new();
        header.set_send_each_at(timestamps);
        assert_eq!(header.send_each_at.is_some(), true);

        header.set_send_at(x);

        let mut s = "{\"send_at\":".to_string();
        s.push_str(&format!("{:?}", x));
        s.push_str("}");

        assert_eq!(s, header.to_string());
        assert_eq!(header.send_each_at.is_none(), true);
    }

    #[test]
    fn header_with_send_each_at_to_json_string() {
        let x = time::now_utc().to_timespec().sec as i64;
        let y = x + 50;
        let z = x + 40;
        let timestamps = vec![x, y];

        let mut header = Header::new();

        header.set_send_at(x);
        assert_eq!(header.send_at.is_some(), true);

        header.set_send_each_at(timestamps)
              .add_send_each_at(z);

        let mut s = "{\"send_each_at\":[".to_string();
        s.push_str(&format!("{:?}", x));
        s.push_str(",");
        s.push_str(&format!("{:?}", y));
        s.push_str(",");
        s.push_str(&format!("{:?}", z));
        s.push_str("]}");

        assert_eq!(s, header.to_string());
        assert_eq!(header.send_at.is_none(), true);
    }
}
