Small library utilizing `crates_io_api` to offer a straightforward interface for retrieving information about crates.

```rust
let mut krate = Krate::new("env_logger", Version::new(0,10,0));
let owners = krate.get_crate_owners().unwrap();
let metadata = krate.get_krate_meta().unwrap();
```