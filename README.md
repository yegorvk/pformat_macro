# `format_args!` without restrictions

The Rust standard library's [`format_args!`] macro combines a format string and its arguments into an [`Arguments`] structure without allocations. However, the result must be used immediately to satisfy the borrow checker.

[`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
[`Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html

This crate provides the `pformat_args!` macro, which returns an opaque `impl Display` instance holding the format string along with the arguments, such that it can be stored for later use.

```toml
[dependencies]
pformat_macro = { git = "https://github.com/yegorvk/pformat_macro" }
```

## Usage

The usage is similar to [`format_args!`], but as of the current version, all the {} placeholders must be empty.

```rust
use pformat_macro::pformat_args;

fn main() {
    let result_str = pformat_args!("1 + 1 = {}", 3);
    println!("{}", result_str); // prints "1 + 1 = 3"
}
```

<sub>Licensed under <a href="LICENSE.md">The MIT License</a></sub>
