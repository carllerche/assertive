# Assertive

A collection of assertions.

## Usage

To use `assertive`, first add this to your `Cargo.toml`:

```toml
[dependencies]
assertive = "0.1.0"
```

Next, add this to your crate:

```rust
use assertive::*;

fn main() {
    let value = assert_ok!(Ok::<_, ()>("hello"));
    assert_eq!(value, "hello");
}
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `assertive` by you, shall be licensed as MIT, without
any additional terms or conditions.

