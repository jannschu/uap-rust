# ua-parser for rust

User agent parser library for Rust based on the
[ua-parser](https://github.com/ua-parser) project.

Add to your `Cargo.toml`:

```
[dependencies]
uap-rust = "0.0.*"
```

## Usage example

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

## Documentation

Documentation is available [here](https://mrbechcrates.github.io/uap-rust-doc/uap_rust/index.html)

## Building from source

```
cargo build
cargo test
```
