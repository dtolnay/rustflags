mod parse;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn from_env() -> RustFlags {
    let encoded = env::var_os("CARGO_ENCODED_RUSTFLAGS")
        .unwrap_or_default()
        .into_string()
        .unwrap_or_else(|s| s.to_string_lossy().into_owned());
    RustFlags {
        encoded,
        pos: 0,
        repeat: None,
        short: false,
    }
}

pub struct RustFlags {
    encoded: String,
    pos: usize,
    repeat: Option<(fn(&str) -> Option<(Flag, usize)>, usize)>,
    short: bool,
}

impl Iterator for RustFlags {
    type Item = Flag;

    fn next(&mut self) -> Option<Self::Item> {
        parse::parse(self)
    }
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Flag {
    /// `-h`, `--help`
    ///
    /// Display help message.
    Help,

    /// `--cfg SPEC`
    ///
    /// Configure the compilation environment.
    Cfg { name: String, value: Option<String> },

    /// `-L [KIND=]PATH`
    ///
    /// Add a directory to the library search path.
    LibrarySearchPath { kind: LibraryKind, path: PathBuf },

    /// `-l [KIND[:MODIFIERS]=]NAME[:RENAME]`
    ///
    /// Link the generated crate(s) to the specified native library NAME.
    /// Optional comma separated MODIFIERS may be specified each with a prefix
    /// of either '+' to enable or '-' to disable.
    Link {
        kind: LinkKind,
        modifiers: Vec<(LinkModifierPrefix, LinkModifier)>,
        name: String,
        rename: Option<String>,
    },

    /// `--crate-type [bin|lib|rlib|dylib|cdylib|staticlib|proc-macro]`
    ///
    /// Comma separated list of types of crates for the compiler to emit.
    CrateType(CrateType),

    /// `--crate-name NAME`
    ///
    /// Specify the name of the crate being built.
    CrateName(String),

    /// `--edition 2015|2018|2021`
    ///
    /// Specify which edition of the compiler to use when compiling code.
    Edition(u16),

    /// `--emit [asm|llvm-bc|llvm-ir|obj|metadata|link|dep-info|mir]`
    ///
    /// Comma separated list of types of output for the compiler to emit.
    Emit(Emit),

    /// `--print [crate-name|file-names|sysroot|target-libdir|cfg|target-list|target-cpus|target-features|relocation-models|code-models|tls-models|target-spec-json|native-static-libs|stack-protector-strategies]`
    ///
    /// Compiler information to print on stdout.
    Print(String),

    /// `-o FILENAME`
    ///
    /// Write output to \<filename\>.
    Out(PathBuf),

    /// `--out-dir DIR`
    ///
    /// Write output to compiler-chosen filename in \<dir\>.
    OutDir(PathBuf),

    /// `--explain OPT`
    ///
    /// Provide a detailed explanation of an error message.
    Explain(String),

    /// `--test`
    ///
    /// Build a test harness.
    Test,

    /// `--target TARGET`
    ///
    /// Target triple for which the code is compiled.
    Target(String),

    /// `-A`, `--allow LINT`
    ///
    /// Set lint allowed.
    Allow(String),

    /// `-W`, `--warn LINT`
    ///
    /// Set lint warnings.
    Warn(String),

    /// `--force-warn LINT`
    ///
    /// Set lint force-warn.
    ForceWarn(String),

    /// `-D`, `--deny LINT`
    ///
    /// Set lint denied.
    Deny(String),

    /// `-F`, `--forbid LINT`
    ///
    /// Set lint forbidden.
    Forbid(String),

    /// `--cap-lints LEVEL`
    ///
    /// Set the most restrictive lint level. More restrictive lints are capped
    /// at this level.
    CapLints(LintLevel),

    /// `-C`, `--codegen OPT[=VALUE]`
    ///
    /// Set a codegen option.
    Codegen { opt: String, value: Option<String> },

    /// `-V`, `--version`
    ///
    /// Print version info and exit.
    Version,

    /// `-v`, `--verbose`
    ///
    /// Use verbose output.
    Verbose,

    /// `--extern NAME[=PATH]`
    ///
    /// Specify where an external rust library is located.
    Extern { name: String, path: Option<PathBuf> },

    /// `--extern-location NAME=LOCATION`
    ///
    /// Location where an external crate dependency is specified.
    ExternLocation { name: String, location: OsString },

    /// `--sysroot PATH`
    ///
    /// Override the system root.
    Sysroot(PathBuf),

    /// `-Z FLAG`
    ///
    /// Set internal debugging options.
    Z(String),

    /// `--error-format human|json|short`
    ///
    /// How errors and other messages are produced.
    ErrorFormat(ErrorFormat),

    /// `--json CONFIG`
    ///
    /// Configure the JSON output of the compiler.
    Json(String),

    /// `--color auto|always|never`
    ///
    /// Configure coloring of output.
    Color(Color),

    /// `--remap-path-prefix FROM=TO`
    ///
    /// Remap source names in all output (compiler messages and output files).
    RemapPathPrefix { from: PathBuf, to: PathBuf },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LibraryKind {
    /// `dependency`
    Dependency,
    /// `crate`
    Crate,
    /// `native`
    Native,
    /// `framework`
    Framework,
    /// `all` (the default)
    All,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LinkKind {
    /// `static`
    Static,
    /// `framework`
    Framework,
    /// `dylib` (the default)
    Dylib,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LinkModifierPrefix {
    /// `+`
    Enable,
    /// `-`
    Disable,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LinkModifier {
    /// `bundle`
    Bundle,
    /// `verbatim`
    Verbatim,
    /// `whole-archive`
    WholeArchive,
    /// `as-needed`
    AsNeeded,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum CrateType {
    /// `bin`
    Bin,
    /// `lib`
    Lib,
    /// `rlib`
    Rlib,
    /// `dylib`
    Dylib,
    /// `cdylib`
    Cdylib,
    /// `staticlib`
    Staticlib,
    /// `proc-macro`
    ProcMacro,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum Emit {
    /// `asm`
    Asm,
    /// `llvm-bc`
    LlvmBc,
    /// `llvm-ir`
    LlvmIr,
    /// `obj`
    Obj,
    /// `metadata`
    Metadata,
    /// `link`
    Link,
    /// `dep-info`
    DepInfo,
    /// `mir`
    Mir,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LintLevel {
    /// `allow`
    Allow,
    /// `warn`
    Warn,
    /// `deny`
    Deny,
    /// `forbid`
    Forbid,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ErrorFormat {
    /// `human`
    Human,
    /// `json`
    Json,
    /// `short`
    Short,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    /// Colorize, if output goes to a tty (default).
    Auto,
    /// Always colorize output.
    Always,
    /// Never colorize output.
    Never,
}
