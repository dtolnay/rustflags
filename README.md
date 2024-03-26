Parser for CARGO_ENCODED_RUSTFLAGS
==================================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/rustflags-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/rustflags)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rustflags.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rustflags)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rustflags-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/rustflags)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/rustflags/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/rustflags/actions?query=branch%3Amaster)

`CARGO_ENCODED_RUSTFLAGS` is one of the environment variables [provided by Cargo
to build scripts][reference]. It synthesizes several sources of flags affecting
Cargo's rustc invocations that build scripts might care about:

- Flags passed via the RUSTFLAGS environment variable,
- Cargo config entries under `target.<triple>.rustflags` and
  `target.<cfg>.rustflags` and `build.rustflags`, including from the
  project-specific Cargo config file and the Cargo config in the user's
  CARGO_HOME.

If a build script needs to make some rustc invocations, or needs to characterize
aspects of the upcoming rustc invocation, it likely needs these flags.

[reference]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

```toml
[build-dependencies]
rustflags = "0.1"
```

<br>

## Example

This build script uses the `cmake` crate to compile some C code, and must
configure it with a particular C preprocessor #define if the Rust build is being
performed with sanitizers.

```rust
// build.rs

use rustflags::Flag;
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_source_dir = manifest_dir.join("lib");
    assert!(lib_source_dir.join("CMakeLists.txt").exists());

    let mut builder = cmake::Config::new(lib_source_dir);

    // Look for -Zsanitizer=address
    for flag in rustflags::from_env() {
        if matches!(flag, Flag::Z(z) if z == "sanitizer=address") {
            builder.define("ENABLE_SANITIZERS", "ON");
            builder.define("SANITIZERS", "address");
            break;
        }
    }

    builder.build();
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
