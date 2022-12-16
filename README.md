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

## Examples

This build script wants to know whether it is okay to enable
`#![feature(proc_macro_span)]`. If the user is building with `-Zallow-features`
with a feature list that does not include `proc_macro_span`, we need to not
enable the feature or the build will fail.

```rust
// build.rs

use rustflags::Flag;

fn main() {
    if is_nightly() && feature_allowed("proc_macro_span") {
        println!("cargo:rustc-cfg=proc_macro_span");
    }
}

// Look for `-Z allow-features=feature1,feature2`
fn feature_allowed(feature: &str) -> bool {
    for flag in rustflags::from_env() {
        if let Flag::Z(option) = flag {
            if option.starts_with("allow-features=") {
                return option["allow-features=".len()..]
                    .split(',')
                    .any(|allowed| allowed == feature);
            }
        }
    }

    // No allow-features= flag, allowed by default.
    true
}
```

This build scripts wants to try compiling a source file to figure out whether an
unstable API is supported in the expected form by the current toolchain.

```rust
// build.rs

use rustflags::Flag;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};

// This code exercises the surface area that we expect of the unstable
// feature. If the current toolchain is able to compile it, we go ahead and
// enable the feature.
const PROBE: &str = r#"
    #![feature(backtrace)]
    #![allow(dead_code)]

    use std::backtrace::{Backtrace, BacktraceStatus};

    fn probe() {
        let backtrace = Backtrace::capture();
        match backtrace.status() {
            BacktraceStatus::Captured | BacktraceStatus::Disabled | _ => {}
        }
    }
"#;

fn main() {
    match compile_probe() {
        Some(status) if status.success() => println!("cargo:rustc-cfg=backtrace"),
        _ => {}
    }
}

fn compile_probe() -> Option<ExitStatus> {
    let rustc = env::var_os("RUSTC")?;
    let out_dir = env::var_os("OUT_DIR")?;
    let probefile = Path::new(&out_dir).join("probe.rs");
    fs::write(&probefile, PROBE).ok()?;

    // Make sure to pick up Cargo rustc configuration.
    let mut cmd = if let Some(wrapper) = env::var_os("CARGO_RUSTC_WRAPPER") {
        let mut cmd = Command::new(wrapper);
        // The wrapper's first argument is supposed to be the path to rustc.
        cmd.arg(rustc);
        cmd
    } else {
        Command::new(rustc)
    };

    cmd.stderr(Stdio::null())
        .arg("--edition=2021")
        .arg("--crate-name=try_backtrace")
        .arg("--crate-type=lib")
        .arg("--emit=metadata")
        .arg("--out-dir")
        .arg(out_dir)
        .arg(probefile)
        .args(rustflags::from_env()
            .filter(|flag| matches!(flag, Flag::Cfg{..} | Flag::Z(_)))
            .flatten())
        .status()
        .ok()
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
