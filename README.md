# macros-rs

macros-rs is a simple, lightweight, and useful library for miscellaneous macros.

## Getting Started

To get started check out the docs on [docs.rs](https://docs.rs/macros-rs)!

```rust
// main.rs
use macros_rs::{exp::ternary, fmt::fmtstr};

fn main() {
  let value = true;
  let hello = "hello";

  println!("{:?} world", ternary!(value, fmtstr!("{hello}"), ""));
}
```

```bash
$ cargo run
Compiling project (/crates)
Finished build [unoptimized + debuginfo] target(s)

"hello" world
```
