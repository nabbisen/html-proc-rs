# htmlproc

[![crates.io](https://img.shields.io/crates/v/htmlproc?label=latest)](https://crates.io/crates/htmlproc)
[![Documentation](https://docs.rs/htmlproc/badge.svg?version=latest)](https://docs.rs/htmlproc/latest)
[![Dependency Status](https://deps.rs/crate/htmlproc/latest/status.svg)](https://deps.rs/crate/htmlproc/latest)
[![License](https://img.shields.io/github/license/nabbisen/htmlproc-rs)](https://github.com/nabbisen/htmlproc-rs/blob/main/LICENSE)

HTML processors as utils written in Rust.
Each function is offered as a single `feature`, so the dependencies are kept small. (`omit_enclosure` which is used as document outline formatter is exception.)

## Install in Rust project

```sh
# install crate
cargo add htmlproc

# install crate with specific features
cargo add htmlproc --features path_to_url

# uninstall
# cargo remove htmlproc
```

## Functions (Features)

### omit_attr

Remove specific tag attribute(s) from HTML text.

#### Usage

First, run `cargo add htmlproc --features omit_attr`. Then specify attrs to omit. Three formats are available:

- `attr`: remove all attrs from all tags.
- `*.attr`: same to the above.
- `tag.attr`: remove all attrs from specifig tag. ex) `span.style`

```rust
use htmlproc::omit_attr::manipulate;

let html = "<div id=\"preserved\"><span style=\"want: remove;\" class=\"also: wanted;\" z-index=\"1\">Content</span></div>";
let omit_attrs = &["style", "*.class", "span.z-index"];
let result: String = manipulate(html, omit_attrs);
```

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

In this case, `href` value "`/some/path`" is converted to "`https://target.domain/some/path`". Options such as http protocol, port number and current directory are available.
