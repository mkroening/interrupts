# interrupts

[![Crates.io](https://img.shields.io/crates/v/interrupts)](https://crates.io/crates/interrupts)
[![docs.rs](https://img.shields.io/docsrs/interrupts)](https://docs.rs/interrupts)
[![CI](https://github.com/mkroening/interrupts/actions/workflows/ci.yml/badge.svg)](https://github.com/mkroening/interrupts/actions/workflows/ci.yml)

Cross-architecture utilities for temporarily disabling interrupts.

Use [`disable`] to disable interrupts with a guard:

```rust
// interrupts may or may not be enabled
let guard = interrupts::disable();
// interrupts are disabled
drop(guard);
// interrupts are restored to the previous state
```

Use [`without`] to run a closure with disabled interrupts:

```rust
// interrupts may or may not be enabled
interrupts::without(|| {
    // interrupts are disabled
});
// interrupts are restored to the previous state
```

For API documentation, see the [docs].

[`disable`]: https://docs.rs/interrupts/latest/interrupts/fn.disable.html
[`without`]: https://docs.rs/interrupts/latest/interrupts/fn.without.html
[docs]: https://docs.rs/interrupts

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
