# snowflake-rust

Kubernetes "twitter" snowflakes.this is not release version,please do not use in production.

By default the original Twitter snowflake format defines:
- 35 bits are used to store a custom epoch with 10 millisecond precision
- 16 bits are used to store low 16 bit from ip address
- 12 bits are used to store a sequence number

## Usage
Add this to your `Cargo.toml`:

```toml

[dependencies]
snowflake-rust = "0.5.6"
```
and this to your crate root:

```rust

use snowflake_rust;

```

## Example

```rust

use snowflake_rust::Snowflake;

fn main() {
    let mut s = Snowflake::kubernetes();
    let id = s.generate().unwrap();
    println!("{:?}", id)
}

```

```rust
// singleton example
use snowflake_rust::Snowflake;

fn main() {
    let id = id().unwrap();
    println!("{:?}", id)
}

pub fn id() -> Option<i64> {
    let instance = get_instance();
    let mut sf = instance.lock().unwrap();
    sf.generate()
}

fn get_instance() -> Arc<Mutex<Snowflake>> {

    static mut SINGLETON: Option<Arc<Mutex<Snowflake>>> = None;

    unsafe {
        SINGLETON.get_or_insert_with( || {
            Arc::new(Mutex::new(Snowflake::kubernetes()))
        }).clone()
    }
}

```
