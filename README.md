# htmlproc

[![crates.io](https://img.shields.io/crates/v/htmlproc?label=latest)](https://crates.io/crates/htmlproc)
[![Documentation](https://docs.rs/htmlproc/badge.svg?version=latest)](https://docs.rs/htmlproc/latest)
[![Dependency Status](https://deps.rs/crate/htmlproc/latest/status.svg)](https://deps.rs/crate/htmlproc/latest)
[![License](https://img.shields.io/github/license/nabbisen/htmlproc-rs)](https://github.com/nabbisen/htmlproc-rs/blob/main/LICENSE)

HTML processors as utils written in Rust.
Each function is offered as a single `feature`, so the dependencies are kept small.

## Install in Rust project

```sh
# intall crate
cargo add htmlproc

# intall crate with specific features
cargo add htmlproc --features path_to_url

# uninstall
# cargo remove htmlproc
```

## Functions (Features)

### omit_enclosure

Remove specific tag enclosure(s) from HTML text.

#### Usage

```rust
use htmlproc::omit_enclosure::manipulate;

let result: String = manipulate("<div>...<span>---</span>...</div>", &["span"]);
```

### path_to_url

Convert paths to URLs.

#### Usage

```rust
use htmlproc::path_to_url::{convert, ConvertOptions};

let result: String = convert("<a href=\"/some/path\">link</a>", ConvertOptions::new("target.domain"));
```
