# SMTP-API

This crate allows you to quickly and more easily generate [SendGrid](https://sendgrid.com) [X-SMTPAPI](https://sendgrid.com/docs/API_Reference/SMTP_API/index.html) headers.

[![BuildStatus](https://travis-ci.org/bsorin/smtpapi-rs.svg?branch=master)](https://travis-ci.org/bsorin/smtpapi-rs)

## Examples

### New Header

```rust
use smtpapi::{Header};

let mut header = Header::new();
```

### Recipients

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.add_to("email@domain.com");
// or
header.add_tos(vec!["email1@domain.com", "email2@domain.com"]);
// or
header.set_tos(vec!["email1@domain.com", "email2@domain.com"]);
```

### [Substitutions](https://sendgrid.com/docs/API_Reference/SMTP_API/substitution_tags.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();

header.add_substitution("[name]", "my_name");
// or
header.add_substitutions("[name]", vec!["my_name_1", "my_name_2"]);
// or
let mut all_subs : HashMap<String, Vec<String>> = HashMap::new();
all_subs.insert("-item1-".to_string(), vec!["rust".to_string(), "power".to_string()]);
all_subs.insert("-item2-".to_string(), vec!["rust".to_string(), "power".to_string()]);

header.set_substitutions(all_subs);
```

### [Section](https://sendgrid.com/docs/API_Reference/SMTP_API/section_tags.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.add_section("-top-", "sample");
// or
let mut sections : HashMap<String, String> = HashMap::new();
sections.insert("-item1-".to_string(), "value1".to_string());
sections.insert("-item2-".to_string(), "value2".to_string());

header.set_sections(sections);
```

### [Category](https://sendgrid.com/docs/Delivery_Metrics/categories.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.add_category("welcome");
// or
header.add_categories(vec!["welcome", "new_accounts"]);
// or
header.set_categories(vec!["welcome", "new_accounts"]);
```

### [Unique Arguments](https://sendgrid.com/docs/API_Reference/SMTP_API/unique_arguments.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.add_unique_arg("account_id", "123412-121-1212");
// or
let mut unique_args : HashMap<String, String> = HashMap::new();
unique_args.insert("-arg1-".to_string(), "value1".to_string());
unique_args.insert("-arg2-".to_string(), "value2".to_string());

header.set_unique_args(unique_args);
```

### [Filters](https://sendgrid.com/docs/API_Reference/SMTP_API/apps.html)

```rust
use smtpapi::{Header, Filter};

let mut header = Header::new();
header.add_filter("clicktrack", "enabled", "1")
      .add_filter("opentrack", "enabled", "1");
// or
let mut filter = Filter::new();
filter.add_setting("enable", "1")
      .add_setting("text/plain", "You can haz footers!");

header.set_filter("footer", filter);

```

### [Send At](https://sendgrid.com/docs/API_Reference/SMTP_API/scheduling_parameters.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();

header.set_send_at(1453213937);
// or
header.set_send_each_at(vec![1453213939, 1453213932, 1453213933]);
// or
header.add_send_each_at(1453213937)
      .add_send_each_at(1453213939);
```

### [ASM Group ID](https://sendgrid.com/docs/User_Guide/advanced_suppression_manager.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.set_asm_group_id(1221);
```

### [IP Pools](https://sendgrid.com/docs/API_Reference/Web_API_v3/IP_Management/ip_pools.html)

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.set_ip_pool("newsletter_pool");
```

### JSONString

```rust
use smtpapi::{Header};

let mut header = Header::new();
header.to_string() //returns a JSON string representation of the headers
```

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Added some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request

## Running Tests

````bash
cargo test
```

## MIT License
