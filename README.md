# uap-rust

[![Build Status](https://travis-ci.org/jannschu/uap-rust.svg?branch=master)](https://travis-ci.org/jannschu/uap-rust)

This is a web browser user agent parser for Rust based on
[ua-parser](https://github.com/ua-parser).

This fork ended up being a complete rewrite. All uap-core tests are passing.

The crate offers parsers with optional thread
safety. The regular expressions for detecting the browser, device or os
are only run when requested. Also, parsers can be chosen to own or
borrow the user agent string. We try to avoid string allocation as much
as possible.

## Usage example

```rust
use uap_rust::unsync::BorrowingParser as Parser;
let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
let parser = Parser::new(agent);

let browser = parser.browser();
assert_eq!(browser.family, "Mobile Safari");
let browser_version = browser.version().unwrap();
assert_eq!(browser_version.major, 5);
assert_eq!(browser_version.minor, 1);

let os = parser.os();
assert_eq!(os.family, "iOS");
let os_version = os.version().unwrap();
assert_eq!(os_version.major, 5);
assert_eq!(os_version.minor, 1);

let device = parser.device();
assert_eq!(device.family, "iPhone");
assert_eq!(device.brand.as_ref().unwrap(), "Apple");
```

To use a `Arc<str>` as a user agent do

```rust
# use std::sync::Arc;
use uap_rust::sync::OwningParser as Parser;
let agent: Arc<str> = Arc::from("Mozilla/5.0 ...");
let parser = Parser::new(agent.clone());
```

In the example above `agent` can also be a `String`. To use `Rc`,
additionally replace `unsync` by `sync`.

The `OwningParser` variant is a convenience wrapper around
`BorrowingParser` to allow storing the user agent along the parser, which
is not trivial, since rust does not understand self-referential structs.