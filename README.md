# as-result

[![Crates.io](https://img.shields.io/crates/v/as-result)](https://crates.io/crates/as-result)

Rust crate which provides the `AsResult<T, E>` and `IntoResult<T, E>` traits.

## Supported Types

By default, the following types from the standard library have implementations of these traits:

- [std::process::ExitStatus] implemented for [std::io::Result]<()>
- [std::process::Output] implemented for [std::io::Result]<[std::process::Output]>

[std::io::Result]: https://doc.rust-lang.org/std/io/type.Result.html
[std::process::ExitStatus]: https://doc.rust-lang.org/std/process/struct.ExitStatus.html
[std::process::Output]: https://doc.rust-lang.org/std/process/struct.Output.html

## Example

Common when spawning commands is the desire to convert the exit status into a result:

```rust
use as_result::*;
use std::process::Command;

Command::new("/bin/echo")
    .arg("hello world")
    .status()
    .and_then(IntoResult::into_result)
    .unwrap();

Command::new("/bin/echo")
    .arg("hello world")
    .status()
    .unwrap()
    .into_result()
    .unwrap();

Command::new("/bin/echo")
    .arg("hello world")
    .status()
    .map_result()
    .unwrap()
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
