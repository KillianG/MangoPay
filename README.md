# WORK IN PROGRESS
MangoPay library for rust

### How to use (by examples)

```rust
let mango: Mangopay = Mangopay::init(
            env!("MANGO_CLIENT_ID").parse().unwrap(),
            env!("MANGO_API_KEY").parse().unwrap(),
        );
```