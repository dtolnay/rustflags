use crate::string::{EnvChar, EnvStr};
use crate::{Flag, RustFlags};
use std::str;

enum FlagConstructor {
    Flag(Flag),
    Opt(fn(&EnvStr) -> Option<Flag>),
    Repeated(fn(&EnvStr) -> Option<(Flag, usize)>),
    Unrecognized,
}

mod opt {
    use crate::string::EnvStr;
    use crate::{
        Color, CrateType, Emit, ErrorFormat, Flag, LibraryKind, LinkKind, LinkModifier,
        LinkModifierPrefix, LintLevel,
    };
    use std::ffi::OsString;
    use std::mem;
    use std::path::PathBuf;

    pub(crate) fn cfg(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        let (name, value) = match arg.split_once('=') {
            Some((name, value)) => {
                let len = value.len();
                if len >= 2 && value.starts_with('"') && value[1..].find('"') == Some(len - 2) {
                    (name, Some(&value[1..len - 1]))
                } else {
                    return None;
                }
            }
            None => (arg, None),
        };
        let name = name.to_owned();
        let value = value.map(str::to_owned);
        Some(Flag::Cfg { name, value })
    }

    pub(crate) fn library_search_path(arg: &EnvStr) -> Option<Flag> {
        let (kind, path) = if let Some((kind, path)) = arg.split_once('=') {
            let kind = match kind.to_str()? {
                "dependency" => LibraryKind::Dependency,
                "crate" => LibraryKind::Crate,
                "native" => LibraryKind::Native,
                "framework" => LibraryKind::Framework,
                "all" => LibraryKind::All,
                _ => return None,
            };
            (kind, path)
        } else {
            (LibraryKind::All, arg)
        };
        let path = PathBuf::from(path);
        Some(Flag::LibrarySearchPath { kind, path })
    }

    pub(crate) fn link(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        let mut modifiers = Vec::new();
        let (kind, name) = match arg.split_once('=') {
            Some((mut kind, name)) => {
                if let Some((modified_kind, comma_separated_modifiers)) = kind.split_once(':') {
                    for modifier in comma_separated_modifiers.split(',') {
                        let prefix = match modifier.chars().next() {
                            Some('+') => LinkModifierPrefix::Enable,
                            Some('-') => LinkModifierPrefix::Disable,
                            _ => continue,
                        };
                        let modifier = match &modifier[1..] {
                            "bundle" => LinkModifier::Bundle,
                            "verbatim" => LinkModifier::Verbatim,
                            "whole-archive" => LinkModifier::WholeArchive,
                            "as-needed" => LinkModifier::AsNeeded,
                            _ => continue,
                        };
                        modifiers.push((prefix, modifier));
                    }
                    kind = modified_kind;
                }
                let kind = match kind {
                    "static" => LinkKind::Static,
                    "framework" => LinkKind::Framework,
                    "dylib" => LinkKind::Dylib,
                    _ => return None,
                };
                (kind, name)
            }
            None => (LinkKind::Dylib, arg),
        };
        let (name, rename) = match name.split_once(':') {
            Some((name, rename)) => (name, Some(rename)),
            None => (name, None),
        };
        let name = name.to_owned();
        let rename = rename.map(str::to_owned);
        Some(Flag::Link {
            kind,
            modifiers,
            name,
            rename,
        })
    }

    pub(crate) fn crate_type(mut arg: &EnvStr) -> Option<(Flag, usize)> {
        while !arg.is_empty() {
            let first = match arg.split_once(',') {
                Some((first, rest)) => {
                    arg = rest;
                    first
                }
                None => mem::take(&mut arg),
            };
            let Some(first) = first.to_str() else {
                continue;
            };
            let crate_type = match first {
                "bin" => CrateType::Bin,
                "lib" => CrateType::Lib,
                "rlib" => CrateType::Rlib,
                "dylib" => CrateType::Dylib,
                "cdylib" => CrateType::Cdylib,
                "staticlib" => CrateType::Staticlib,
                "proc-macro" => CrateType::ProcMacro,
                _ => continue,
            };
            return Some((Flag::CrateType(crate_type), arg.len()));
        }
        None
    }

    pub(crate) fn crate_name(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::CrateName(arg.to_owned()))
    }

    pub(crate) fn edition(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        arg.parse().ok().map(Flag::Edition)
    }

    pub(crate) fn emit(mut arg: &EnvStr) -> Option<(Flag, usize)> {
        while !arg.is_empty() {
            let first = match arg.split_once(',') {
                Some((first, rest)) => {
                    arg = rest;
                    first
                }
                None => mem::take(&mut arg),
            };
            let Some(first) = first.to_str() else {
                continue;
            };
            let emit = match first {
                "asm" => Emit::Asm,
                "llvm-bc" => Emit::LlvmBc,
                "llvm-ir" => Emit::LlvmIr,
                "obj" => Emit::Obj,
                "metadata" => Emit::Metadata,
                "link" => Emit::Link,
                "dep-info" => Emit::DepInfo,
                "mir" => Emit::Mir,
                _ => continue,
            };
            return Some((Flag::Emit(emit), arg.len()));
        }
        None
    }

    pub(crate) fn print(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Print(arg.to_owned()))
    }

    pub(crate) fn out(arg: &EnvStr) -> Option<Flag> {
        Some(Flag::Out(PathBuf::from(arg)))
    }

    pub(crate) fn out_dir(arg: &EnvStr) -> Option<Flag> {
        Some(Flag::OutDir(PathBuf::from(arg)))
    }

    pub(crate) fn explain(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Explain(arg.to_owned()))
    }

    pub(crate) fn target(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Target(arg.to_owned()))
    }

    pub(crate) fn allow(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Allow(arg.to_owned()))
    }

    pub(crate) fn warn(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Warn(arg.to_owned()))
    }

    pub(crate) fn force_warn(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::ForceWarn(arg.to_owned()))
    }

    pub(crate) fn deny(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Deny(arg.to_owned()))
    }

    pub(crate) fn forbid(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Forbid(arg.to_owned()))
    }

    pub(crate) fn cap_lints(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        let level = match arg {
            "allow" => LintLevel::Allow,
            "warn" => LintLevel::Warn,
            "deny" => LintLevel::Deny,
            "forbid" => LintLevel::Forbid,
            _ => return None,
        };
        Some(Flag::CapLints(level))
    }

    pub(crate) fn codegen(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        let (opt, value) = match arg.split_once('=') {
            Some((opt, value)) => (opt, Some(value)),
            None => (arg, None),
        };
        let opt = opt.to_owned();
        let value = value.map(str::to_owned);
        Some(Flag::Codegen { opt, value })
    }

    pub(crate) fn extern_(arg: &EnvStr) -> Option<Flag> {
        let (name, path) = match arg.split_once('=') {
            Some((name, path)) => (name, Some(path)),
            None => (arg, None),
        };
        let name = name.to_str()?.to_owned();
        let path = path.map(PathBuf::from);
        Some(Flag::Extern { name, path })
    }

    pub(crate) fn extern_location(arg: &EnvStr) -> Option<Flag> {
        let (name, location) = arg.split_once('=')?;
        let name = name.to_str()?.to_owned();
        let location = OsString::from(location);
        Some(Flag::ExternLocation { name, location })
    }

    pub(crate) fn sysroot(arg: &EnvStr) -> Option<Flag> {
        Some(Flag::Sysroot(PathBuf::from(arg)))
    }

    pub(crate) fn z(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Z(arg.to_owned()))
    }

    pub(crate) fn error_format(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        let format = match arg {
            "human" => ErrorFormat::Human,
            "json" => ErrorFormat::Json,
            "short" => ErrorFormat::Short,
            _ => return None,
        };
        Some(Flag::ErrorFormat(format))
    }

    pub(crate) fn json(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        Some(Flag::Json(arg.to_owned()))
    }

    pub(crate) fn color(arg: &EnvStr) -> Option<Flag> {
        let arg = arg.to_str()?;
        let color = match arg {
            "auto" => Color::Auto,
            "always" => Color::Always,
            "never" => Color::Never,
            _ => return None,
        };
        Some(Flag::Color(color))
    }

    pub(crate) fn remap_path_prefix(arg: &EnvStr) -> Option<Flag> {
        let (from, to) = arg.split_once('=')?;
        let from = PathBuf::from(from);
        let to = PathBuf::from(to);
        Some(Flag::RemapPathPrefix { from, to })
    }
}

fn lookup_short(ch: char) -> FlagConstructor {
    match ch {
        'h' => FlagConstructor::Flag(Flag::Help),
        'L' => FlagConstructor::Opt(opt::library_search_path),
        'l' => FlagConstructor::Opt(opt::link),
        'g' => FlagConstructor::Flag(Flag::Codegen {
            opt: "debuginfo".to_owned(),
            value: Some("2".to_owned()),
        }),
        'O' => FlagConstructor::Flag(Flag::Codegen {
            opt: "opt-level".to_owned(),
            value: Some("2".to_owned()),
        }),
        'o' => FlagConstructor::Opt(opt::out),
        'A' => FlagConstructor::Opt(opt::allow),
        'W' => FlagConstructor::Opt(opt::warn),
        'D' => FlagConstructor::Opt(opt::deny),
        'F' => FlagConstructor::Opt(opt::forbid),
        'C' => FlagConstructor::Opt(opt::codegen),
        'V' => FlagConstructor::Flag(Flag::Version),
        'v' => FlagConstructor::Flag(Flag::Verbose),
        'Z' => FlagConstructor::Opt(opt::z),
        _ => FlagConstructor::Unrecognized,
    }
}

fn lookup_long(name: &str) -> FlagConstructor {
    match name {
        "help" => FlagConstructor::Flag(Flag::Help),
        "cfg" => FlagConstructor::Opt(opt::cfg),
        "crate-type" => FlagConstructor::Repeated(opt::crate_type),
        "crate-name" => FlagConstructor::Opt(opt::crate_name),
        "edition" => FlagConstructor::Opt(opt::edition),
        "emit" => FlagConstructor::Repeated(opt::emit),
        "print" => FlagConstructor::Opt(opt::print),
        "out-dir" => FlagConstructor::Opt(opt::out_dir),
        "explain" => FlagConstructor::Opt(opt::explain),
        "test" => FlagConstructor::Flag(Flag::Test),
        "target" => FlagConstructor::Opt(opt::target),
        "allow" => FlagConstructor::Opt(opt::allow),
        "warn" => FlagConstructor::Opt(opt::warn),
        "force-warn" => FlagConstructor::Opt(opt::force_warn),
        "deny" => FlagConstructor::Opt(opt::deny),
        "forbid" => FlagConstructor::Opt(opt::forbid),
        "cap-lints" => FlagConstructor::Opt(opt::cap_lints),
        "codegen" => FlagConstructor::Opt(opt::codegen),
        "version" => FlagConstructor::Flag(Flag::Version),
        "verbose" => FlagConstructor::Flag(Flag::Verbose),
        "extern" => FlagConstructor::Opt(opt::extern_),
        "extern-location" => FlagConstructor::Opt(opt::extern_location),
        "sysroot" => FlagConstructor::Opt(opt::sysroot),
        "error-format" => FlagConstructor::Opt(opt::error_format),
        "json" => FlagConstructor::Opt(opt::json),
        "color" => FlagConstructor::Opt(opt::color),
        "remap-path-prefix" => FlagConstructor::Opt(opt::remap_path_prefix),
        _ => FlagConstructor::Unrecognized,
    }
}

pub(crate) fn parse(f: &mut RustFlags) -> Option<Flag> {
    const SEPARATOR: char = '\x1F';

    let mut skip = false;

    while f.pos < f.encoded.len() {
        if skip {
            match f.encoded[f.pos..].find(SEPARATOR) {
                // `nonflag` ...
                Some(i) => f.pos += i + 1,
                // `nonflag`$
                None => f.pos = f.encoded.len(),
            }
            skip = false;
            continue;
        }

        let (constructor, arg) = if let Some((constructor, len)) = f.repeat.take() {
            let arg = &f.encoded[f.pos..f.pos + len];
            f.pos += len;
            (ConstructorFn::Repeated(constructor), arg)
        } else if f.short {
            let ch = match f.encoded[f.pos..].first_char().unwrap() {
                EnvChar::Valid(ch) => {
                    f.pos += ch.len_utf8();
                    if ch == SEPARATOR {
                        f.short = false;
                        continue;
                    }
                    ch
                }
                EnvChar::Invalid => '\0',
            };
            let constructor = match lookup_short(ch) {
                FlagConstructor::Flag(flag) => return Some(flag),
                FlagConstructor::Opt(f) => ConstructorFn::Opt(f),
                FlagConstructor::Repeated(f) => ConstructorFn::Repeated(f),
                FlagConstructor::Unrecognized => {
                    f.short = false;
                    skip = true;
                    continue;
                }
            };
            f.short = false;
            if f.pos == f.encoded.len() {
                break;
            }
            if f.encoded[f.pos..].starts_with(SEPARATOR) {
                // `-X` `arg`
                f.pos += 1;
            }
            let arg = if let Some(i) = f.encoded[f.pos..].find(SEPARATOR) {
                // `-Xarg` ...
                let arg = &f.encoded[f.pos..f.pos + i];
                f.pos += i + 1;
                arg
            } else {
                // `-Xarg`$
                let arg = &f.encoded[f.pos..];
                f.pos = f.encoded.len();
                arg
            };
            (constructor, arg)
        } else if f.encoded[f.pos..].starts_with('-') {
            let Some(first_char) = f.encoded[f.pos + 1..].first_char() else {
                // `-`$
                f.pos += 1;
                continue;
            };
            match first_char {
                // `-` ...
                EnvChar::Valid(SEPARATOR) => {
                    f.pos += 2;
                    continue;
                }
                EnvChar::Valid('-') => {
                    let flag = match f.encoded[f.pos + 2..].find(SEPARATOR) {
                        // `--`
                        Some(0) => {
                            f.pos = f.encoded.len();
                            continue;
                        }
                        Some(i) => {
                            let flag = &f.encoded[f.pos + 2..f.pos + 2 + i];
                            f.pos += i + 3;
                            flag
                        }
                        None => {
                            let flag = &f.encoded[f.pos + 2..];
                            f.pos = f.encoded.len();
                            flag
                        }
                    };
                    let (name, arg) = match flag.split_once('=') {
                        Some((name, arg)) => (name, Some(arg)),
                        None => (flag, None),
                    };
                    let Some(name) = name.to_str() else {
                        continue;
                    };
                    let constructor = match lookup_long(name) {
                        // `--flag`
                        FlagConstructor::Flag(flag) if arg.is_none() => return Some(flag),
                        FlagConstructor::Opt(f) => ConstructorFn::Opt(f),
                        FlagConstructor::Repeated(f) => ConstructorFn::Repeated(f),
                        FlagConstructor::Unrecognized | FlagConstructor::Flag(_) => continue,
                    };
                    let arg = if let Some(arg) = arg {
                        // `--opt=arg`
                        arg
                    } else if let Some(i) = f.encoded[f.pos..].find(SEPARATOR) {
                        // `--opt` `arg` ...
                        let arg = &f.encoded[f.pos..f.pos + i];
                        f.pos += i + 1;
                        arg
                    } else {
                        // `--opt` `arg`$
                        let arg = &f.encoded[f.pos..];
                        f.pos = f.encoded.len();
                        arg
                    };
                    (constructor, arg)
                }
                // `-X`
                EnvChar::Valid(_) | EnvChar::Invalid => {
                    f.pos += 1;
                    f.short = true;
                    continue;
                }
            }
        } else {
            skip = true;
            continue;
        };

        enum ConstructorFn {
            Opt(fn(&EnvStr) -> Option<Flag>),
            Repeated(fn(&EnvStr) -> Option<(Flag, usize)>),
        }

        match constructor {
            ConstructorFn::Opt(constructor) => {
                if let Some(flag) = constructor(arg) {
                    return Some(flag);
                }
            }
            ConstructorFn::Repeated(constructor) => {
                if let Some((flag, repeat)) = constructor(arg) {
                    if repeat > 0 {
                        f.pos -= repeat + f.encoded[..f.pos].ends_with(SEPARATOR) as usize;
                        f.repeat = Some((constructor, repeat));
                    }
                    return Some(flag);
                }
            }
        }
    }

    None
}
