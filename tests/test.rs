#![allow(clippy::too_many_lines)]

use rustflags::{
    Color, CrateType, Emit, ErrorFormat, Flag, LibraryKind, LinkKind, LinkModifier,
    LinkModifierPrefix, LintLevel,
};
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

#[track_caller]
fn test(encoded: &str, expected: &[Flag]) {
    let mut iterator = rustflags::from_encoded(OsStr::new(encoded));

    let mut flags = Vec::new();
    for expected in expected {
        let next = iterator.next();
        assert_eq!(Some(expected), next.as_ref());
        for flag in next.unwrap() {
            flags.push(flag);
        }
    }

    assert_eq!(None, iterator.next());

    let re_encoded = flags.join(OsStr::new("\x1F"));
    let mut iterator = rustflags::from_encoded(&re_encoded);

    for expected in expected {
        assert_eq!(Some(expected), iterator.next().as_ref());
    }

    assert_eq!(None, iterator.next());
}

macro_rules! assert_flags {
    ($string:literal $($more_string:literal)* $(, $expected:expr)* $(,)?) => {{
        let encoded = concat!($string $(, '\x1F', $more_string)*);
        test(encoded, &[$($expected),*]);
    }};
}

#[test]
fn test_empty() {
    assert_flags!("");
}

#[test]
fn test_individual() {
    // Flag::Help
    assert_flags!("-h", Flag::Help);
    assert_flags!("--help", Flag::Help);

    // Flag::Cfg
    assert_flags!(
        "--cfg=semver_exempt",
        Flag::Cfg {
            name: "semver_exempt".to_owned(),
            value: None,
        },
    );
    assert_flags!(
        "--cfg=feature=\"std\"",
        Flag::Cfg {
            name: "feature".to_owned(),
            value: Some("std".to_owned()),
        },
    );
    assert_flags!(
        "--cfg" "semver_exempt",
        Flag::Cfg {
            name: "semver_exempt".to_owned(),
            value: None,
        },
    );
    assert_flags!(
        "--cfg" "feature=\"std\"",
        Flag::Cfg {
            name: "feature".to_owned(),
            value: Some("std".to_owned()),
        },
    );

    // Flag::LibrarySearchPath
    assert_flags!(
        "-L" "PATH",
        Flag::LibrarySearchPath {
            kind: LibraryKind::All,
            path: PathBuf::from("PATH"),
        },
    );
    assert_flags!(
        "-L" "native=PATH",
        Flag::LibrarySearchPath {
            kind: LibraryKind::Native,
            path: PathBuf::from("PATH"),
        },
    );

    // Flag::Link
    assert_flags!(
        "-l" "NAME",
        Flag::Link {
            kind: LinkKind::Dylib,
            modifiers: Vec::new(),
            name: "NAME".to_owned(),
            rename: None,
        },
    );
    assert_flags!(
        "-l" "static=NAME",
        Flag::Link {
            kind: LinkKind::Static,
            modifiers: Vec::new(),
            name: "NAME".to_owned(),
            rename: None,
        },
    );
    assert_flags!(
        "-l" "static:+bundle,-whole-archive=NAME",
        Flag::Link {
            kind: LinkKind::Static,
            modifiers: vec![
                (LinkModifierPrefix::Enable, LinkModifier::Bundle),
                (LinkModifierPrefix::Disable, LinkModifier::WholeArchive),
            ],
            name: "NAME".to_owned(),
            rename: None,
        },
    );
    assert_flags!(
        "-l" "NAME:RENAME",
        Flag::Link {
            kind: LinkKind::Dylib,
            modifiers: Vec::new(),
            name: "NAME".to_owned(),
            rename: Some("RENAME".to_owned()),
        },
    );

    // Flag::CrateType
    assert_flags!("--crate-type" "bin", Flag::CrateType(CrateType::Bin));
    assert_flags!(
        "--crate-type" "lib,staticlib",
        Flag::CrateType(CrateType::Lib),
        Flag::CrateType(CrateType::Staticlib),
    );

    // Flag::CrateName
    assert_flags!("--crate-name" "core", Flag::CrateName("core".to_owned()));

    // Flag::Edition
    assert_flags!("--edition" "2021", Flag::Edition(2021));

    // Flag::Emit
    assert_flags!("--emit" "asm", Flag::Emit(Emit::Asm));
    assert_flags!(
        "--emit" "asm,mir",
        Flag::Emit(Emit::Asm),
        Flag::Emit(Emit::Mir),
    );
    assert_flags!("--emit" "unrecognized,mir", Flag::Emit(Emit::Mir));

    // Flag::Print
    assert_flags!("--print" "cfg", Flag::Print("cfg".to_owned()));

    // Flag::Out
    assert_flags!("-o" "FILENAME", Flag::Out(PathBuf::from("FILENAME")));

    // Flag::OutDir
    assert_flags!("--out-dir" "DIR", Flag::OutDir(PathBuf::from("DIR")));

    // Flag::Explain
    assert_flags!("--explain" "E0282", Flag::Explain("E0282".to_owned()));

    // Flag::Test
    assert_flags!("--test", Flag::Test);

    // Flag::Target
    assert_flags!(
        "--target" "x86_64-unknown-linux-gnu",
        Flag::Target("x86_64-unknown-linux-gnu".to_owned()),
    );

    // Flag::Allow
    assert_flags!("-A" "dead_code", Flag::Allow("dead_code".to_owned()));
    assert_flags!("--allow" "dead_code", Flag::Allow("dead_code".to_owned()));

    // Flag::Warn
    assert_flags!("-W" "dead_code", Flag::Warn("dead_code".to_owned()));
    assert_flags!("--warn" "dead_code", Flag::Warn("dead_code".to_owned()));

    // Flag::ForceWarn
    assert_flags!(
        "--force-warn" "dead_code",
        Flag::ForceWarn("dead_code".to_owned()),
    );

    // Flag::Deny
    assert_flags!("-D" "dead_code", Flag::Deny("dead_code".to_owned()));
    assert_flags!("--deny" "dead_code", Flag::Deny("dead_code".to_owned()));

    // Flag::Forbid
    assert_flags!("-F" "dead_code", Flag::Forbid("dead_code".to_owned()));
    assert_flags!("--forbid" "dead_code", Flag::Forbid("dead_code".to_owned()));

    // Flag::CapLints
    assert_flags!("--cap-lints=allow", Flag::CapLints(LintLevel::Allow));

    // Flag::Codegen
    assert_flags!(
        "-C" "embed-bitcode",
        Flag::Codegen {
            opt: "embed-bitcode".to_owned(),
            value: None,
        },
    );
    assert_flags!(
        "-C" "debuginfo=2",
        Flag::Codegen {
            opt: "debuginfo".to_owned(),
            value: Some("2".to_owned()),
        },
    );
    assert_flags!(
        "-g",
        Flag::Codegen {
            opt: "debuginfo".to_owned(),
            value: Some("2".to_owned()),
        },
    );
    assert_flags!(
        "-O",
        Flag::Codegen {
            opt: "opt-level".to_owned(),
            value: Some("2".to_owned()),
        },
    );

    // Flag::Version
    assert_flags!("-V", Flag::Version);
    assert_flags!("--version", Flag::Version);

    // Flag::Verbose
    assert_flags!("-v", Flag::Verbose);
    assert_flags!("--verbose", Flag::Verbose);

    // Flag::Extern
    assert_flags!(
        "--extern" "serde",
        Flag::Extern {
            name: "serde".to_owned(),
            path: None,
        },
    );
    assert_flags!(
        "--extern" "serde=target/debug/deps/libserde.rmeta",
        Flag::Extern {
            name: "serde".to_owned(),
            path: Some(PathBuf::from("target/debug/deps/libserde.rmeta")),
        },
    );

    // Flag::ExternLocation
    assert_flags!(
        "--extern-location" r#"serde=json:{"target":"//third-party:serde"}"#,
        Flag::ExternLocation {
            name: "serde".to_owned(),
            location: OsString::from(r#"json:{"target":"//third-party:serde"}"#),
        },
    );

    // Flag::Sysroot
    assert_flags!(
        "--sysroot" ".rustup/toolchains/nightly",
        Flag::Sysroot(PathBuf::from(".rustup/toolchains/nightly")),
    );

    // Flag::Z
    assert_flags!(
        "-Z" "unstable-options",
        Flag::Z("unstable-options".to_owned()),
    );

    // Flag::ErrorFormat
    assert_flags!("--error-format=json", Flag::ErrorFormat(ErrorFormat::Json));

    // Flag::Json
    assert_flags!(
        "--json" "diagnostic-rendered-ansi",
        Flag::Json("diagnostic-rendered-ansi".to_owned()),
    );

    // Flag::Color
    assert_flags!("--color=always", Flag::Color(Color::Always));

    // Flag::RemapPathPrefix
    assert_flags!(
        "--remap-path-prefix" "FROM=TO",
        Flag::RemapPathPrefix {
            from: PathBuf::from("FROM"),
            to: PathBuf::from("TO"),
        },
    );
}

#[test]
fn test_unrecognized() {
    assert_flags!(
        "-goto",
        Flag::Codegen {
            opt: "debuginfo".to_owned(),
            value: Some("2".to_owned()),
        },
        Flag::Out(PathBuf::from("to")),
    );

    assert_flags!(
        "-gxvto" "-h",
        Flag::Codegen {
            opt: "debuginfo".to_owned(),
            value: Some("2".to_owned()),
        },
        Flag::Help,
    );
}
