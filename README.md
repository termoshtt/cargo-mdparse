cargo-mdparse
==============

Split Rust code from Markdown for testing.

Usage
------

```
cargo-mdparse 

USAGE:
    cargo mdparse [FLAGS] <markdown_path>

FLAGS:
    -a, --anonymous    Parse anonymous code block
    -h, --help         Prints help information
    -V, --version      Prints version information

ARGS:
    <markdown_path>    Path to Markdown file
```


Named Rust code block will be split out into `exmaples/` directory.

```rust:test.rs
fn main() {
  println!("Split to examples/test.rs");
}
```

Anonymous block will be split out as `examples/mdparse1.rs` if `--anonymous` or `-a` flag is set.

```rust
fn main () {
  println!("1 + 1 = {}", 1 + 1);
}
```
