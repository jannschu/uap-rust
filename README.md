# uap-rust

## ua-parser for rust

This is a web browser user agent parser for Rust based on
[ua-parser](https://github.com/ua-parser).

### Usage example

```rust
use uap_rust::Client;
let agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 5_1_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9B206 Safari/7534.48.3";
let client = Client::new(agent);

let browser = client.browser();
let os = client.os();
let device = client.device();

println!("{:?}", browser);
println!("{:?}", os);
println!("{:?}", device);
```

License: MIT
