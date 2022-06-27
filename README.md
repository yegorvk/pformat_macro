
Alternative to the builtin [`format_args!`] macro
=================================================

Rust standard library has [`format_args!`] macro that lets
you combine format string and its arguments into the [`Arguments`] structure
without allocations in cost of directly referencing parts of format string,
so the result must be used immediately (to satisfy borrow checker).

[`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
[`Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html

This crates offers `pformat_args!` macro that returns `impl Display` instance
that can be used just like any normal structure holding provided format arguments.

```toml
pformat_args = { git = "https://github.com/egor-vaskon/pformat_macro" }
```

# How to use

The usage is almost the same as  [`format_args!`] except that all `{}`
placeholders must be empty.

```rust
use pformat_macro::pformat_args;

fn main() {
    let result_str = pformat_args!("1 + 1 = {}", 3);
    println!("{}", result_str) //prints 1 + 1 = 3
}
```

<sub>
Licensed under <a href="LICENSE.md">The MIT License</a>
</sub>