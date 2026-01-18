# A commonmark viewer for [egui](https://github.com/emilk/egui)

[![Crate](https://img.shields.io/crates/v/gorbie-commonmark.svg)](https://crates.io/crates/gorbie-commonmark)
[![Documentation](https://docs.rs/gorbie-commonmark/badge.svg)](https://docs.rs/gorbie-commonmark)

<img src="https://raw.githubusercontent.com/triblespace/gorbie_commonmark/main/assets/example-v4.png" alt="showcase" width=280/>

This is a fork of [egui_commonmark](https://github.com/lampsitter/egui_commonmark) to customize the
appearance so it better matches the GORBIE visual style.

While this crate's main focus is commonmark, it also supports a subset of
Github's markdown syntax: tables, strikethrough, tasklists and footnotes.

## Usage

In Cargo.toml:

```toml
gorbie-commonmark = "0.22"
# Specify what image formats you want to use
image = { version = "0.25", default-features = false, features = ["png"] }
```

```rust
use gorbie_commonmark::*;
let markdown =
r"# Hello world

* A list
* [ ] Checkbox
";

let mut cache = CommonMarkCache::default();
CommonMarkViewer::new().show(ui, &mut cache, markdown);
```


## Compile time evaluation of markdown

If you want to embed markdown directly the binary then you can enable the `macros` feature.
This will do the parsing of the markdown at compile time and output egui widgets.

### Example

```rust
use gorbie_commonmark::{CommonMarkCache, commonmark};
let mut cache = CommonMarkCache::default();
let _response = commonmark!(ui, &mut cache, "# ATX Heading Level 1");
```

Alternatively you can embed a file

### Example

```rust
use gorbie_commonmark::{CommonMarkCache, commonmark_str};
let mut cache = CommonMarkCache::default();
commonmark_str!(ui, &mut cache, "content.md");
```


## Features

* `macros`: macros for compile time parsing of markdown
* `better_syntax_highlighting`: Syntax highlighting inside code blocks with
  [`syntect`](https://crates.io/crates/syntect)
* `svg`: Support for viewing svg images
* `fetch`: Images with urls will be downloaded and displayed
* `embedded_image`: Load base64 image data urls from within markdown files


## Examples

For an easy intro check out the `hello_world` example. To see all the different
features gorbie_commonmark has to offer check out the `book` example.

## FAQ

### URL is not displayed when hovering over a link

By default egui does not show urls when you hover hyperlinks. To enable it,
you can do the following before calling any ui related functions:

```rust
ui.style_mut().url_in_tooltip = true;
```

## MSRV Policy

This crate uses the same MSRV as the latest released egui version.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
