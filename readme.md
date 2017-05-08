# pangu

[![Build Status][build-badge]][build-status]
[![Crates Version][crates-badge]][crates-url]
[![Rust Docs][docs-badge]][docs-url]

Paranoid text spacing for good readability, to automatically insert whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).

- [pangu.clj](https://github.com/coldnew/pangu.clj) (Clojure)
- [pangu.ex](https://github.com/cataska/pangu.ex) (Elixir)
- [pangu.go](https://github.com/vinta/pangu) (Go)
- [pangu.java](https://github.com/vinta/pangu.java) (Java)
- [pangu.js](https://github.com/vinta/pangu.js) (JavaScript, both Node and Browser)
- [pangu.objective-c](https://github.com/Cee/pangu.objective-c) (Objective-C)
- [pangu.php](https://github.com/Kunr/pangu.php) (PHP)
- [pangu.py](https://github.com/vinta/pangu.py) (Python)
- [pangu.rb](https://github.com/dlackty/pangu.rb) (Ruby)
- [pangu.swift](https://github.com/X140Yu/pangu.Swift) (Swift)

## usage

add this to `Cargo.toml`:

```toml
[dependencies]
pangu = "0.0.1"
```

example:

```rust
extern crate pangu;

fn main() {
  assert_eq!(
    pangu::spacing("新八的構造成分有95%是眼鏡、3%是水、2%是垃圾"),
    "新八的構造成分有 95% 是眼鏡、3% 是水、2% 是垃圾"
  );
}
```

## license

MIT

[build-badge]: https://img.shields.io/travis/airt/pangu.rs.svg
[build-status]: https://travis-ci.org/airt/pangu.rs
[crates-badge]: https://img.shields.io/crates/v/pangu.svg
[crates-url]: https://crates.io/crates/pangu
[docs-badge]: https://docs.rs/pangu/badge.svg
[docs-url]: https://docs.rs/pangu
