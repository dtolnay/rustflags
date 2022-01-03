use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Flag {
    Help,
    Cfg { name: String, value: Option<String> },
    LibrarySearchPath { kind: LibraryKind, path: PathBuf },
    Link {
        kind: LinkKind,
        modifiers: Vec<(LinkModifierPrefix, LinkModifier)>,
        name: String,
        rename: Option<String>,
    },
    CrateType(CrateType),
    CrateName(String),
    Edition(u16),
    Emit(Emit),
    Print(String),
    Out(PathBuf),
    OutDir(PathBuf),
    Explain(String),
    Test,
    Target(String),
    Allow(String),
    Warn(String),
    ForceWarn(String),
    Deny(String),
    Forbid(String),
    CapLints(LintLevel),
    Codegen { opt: String, value: Option<String> },
    Version,
    Verbose,
    Extern { name: String, path: Option<PathBuf> },
    ExternLocation { name: String, location: OsString },
    Sysroot(PathBuf),
    Z(String),
    ErrorFormat(ErrorFormat),
    Json(String),
    Color(Color),
    RemapPathPrefix { from: PathBuf, to: PathBuf },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LibraryKind {
    Dependency,
    Crate,
    Native,
    Framework,
    All,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LinkKind {
    Static,
    Framework,
    Dylib,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LinkModifierPrefix {
    Enable,
    Disable,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum LinkModifier {
    Bundle,
    Verbatim,
    WholeArchive,
    AsNeeded,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum CrateType {
    Bin,
    Lib,
    Rlib,
    Dylib,
    Cdylib,
    Staticlib,
    ProcMacro,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum Emit {
    Asm,
    LlvmBc,
    LlvmIr,
    Obj,
    Metadata,
    Link,
    DepInfo,
    Mir,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LintLevel {
    Allow,
    Warn,
    Deny,
    Forbid,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ErrorFormat {
    Human,
    Json,
    Short,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Auto,
    Always,
    Never,
}
