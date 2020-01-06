# rustflake

Kubernetes "twitter" snowflakes.this is not release version,please do not use in production.

By default the original Twitter snowflake format defines:
- 35 bits are used to store a custom epoch with 10 millisecond precision
- 16 bits are used to store low 16 bit from ip address
- 12 bits are used to store a sequence number

## Usage
Add this to your `Cargo.toml`:

```toml

[dependencies]
snowflake-rust = "0.2.0"
```
and this to your crate root:

```rust

use snowflake;

```

## Example

```rust

use snowflake;

fn main() {
    let mut s = Snowflake::kubernetes();
    let id = s.generate().unwrap();
    println!("{}", id)
}

```
