use crate::write::WriteFmt;
use crate::{Flag, LibraryKind, LinkKind};
use std::ffi::{OsStr, OsString};

impl IntoIterator for Flag {
    type Item = OsString;
    type IntoIter = iter::Iter;

    fn into_iter(self) -> Self::IntoIter {
        let mut flags = Vec::new();

        match self {
            Flag::Help => {
                flags.push(OsString::from("--help"));
            }

            Flag::Cfg { name, value } => {
                flags.push(OsString::from("--cfg"));
                if let Some(value) = value {
                    flags.push(OsString::from(format!("{}=\"{}\"", name, value)));
                } else {
                    flags.push(OsString::from(name));
                }
            }

            Flag::LibrarySearchPath { kind, path } => {
                flags.push(OsString::from("-L"));
                if kind == LibraryKind::All {
                    flags.push(OsString::from(path));
                } else {
                    let mut flag = OsString::new();
                    write!(flag, "{}=", kind);
                    flag.push(path);
                    flags.push(flag);
                }
            }

            Flag::Link {
                kind,
                modifiers,
                name,
                rename,
            } => {
                flags.push(OsString::from("-l"));
                let mut flag = OsString::new();
                if kind != LinkKind::default() || !modifiers.is_empty() {
                    write!(flag, "{}", kind);
                }
                for (i, (prefix, modifier)) in modifiers.iter().enumerate() {
                    flag.push(if i == 0 { ":" } else { "," });
                    write!(flag, "{}{}", prefix, modifier);
                }
                if !flag.is_empty() {
                    flag.push("=");
                }
                flag.push(name);
                if let Some(rename) = rename {
                    flag.push(":");
                    flag.push(rename);
                }
                flags.push(flag);
            }

            Flag::CrateType(crate_type) => {
                flags.push(OsString::from("--crate-type"));
                flags.push(OsString::from(crate_type.to_string()));
            }

            Flag::CrateName(crate_name) => {
                flags.push(OsString::from("--crate-name"));
                flags.push(OsString::from(crate_name));
            }

            Flag::Edition(edition) => {
                flags.push(OsString::from("--edition"));
                flags.push(OsString::from(edition.to_string()));
            }

            Flag::Emit(emit) => {
                flags.push(OsString::from("--emit"));
                flags.push(OsString::from(emit.to_string()));
            }

            Flag::Print(print) => {
                flags.push(OsString::from("--print"));
                flags.push(OsString::from(print));
            }

            Flag::Out(filename) => {
                flags.push(OsString::from("-o"));
                flags.push(OsString::from(filename));
            }

            Flag::OutDir(dir) => {
                flags.push(OsString::from("--out-dir"));
                flags.push(OsString::from(dir));
            }

            Flag::Explain(code) => {
                flags.push(OsString::from("--explain"));
                flags.push(OsString::from(code));
            }

            Flag::Test => {
                flags.push(OsString::from("--test"));
            }

            Flag::Target(target) => {
                flags.push(OsString::from("--target"));
                flags.push(OsString::from(target));
            }

            Flag::Allow(lint) => {
                flags.push(OsString::from("--allow"));
                flags.push(OsString::from(lint));
            }

            Flag::Warn(lint) => {
                flags.push(OsString::from("--warn"));
                flags.push(OsString::from(lint));
            }

            Flag::ForceWarn(lint) => {
                flags.push(OsString::from("--force-warn"));
                flags.push(OsString::from(lint));
            }

            Flag::Deny(lint) => {
                flags.push(OsString::from("--deny"));
                flags.push(OsString::from(lint));
            }

            Flag::Forbid(lint) => {
                flags.push(OsString::from("--forbid"));
                flags.push(OsString::from(lint));
            }

            Flag::CapLints(lint_level) => {
                flags.push(OsString::from("--cap-lints"));
                flags.push(OsString::from(lint_level.to_string()));
            }

            Flag::Codegen { opt, value } => {
                flags.push(OsString::from("-C"));
                if let Some(value) = value {
                    flags.push(OsString::from(format!("{}={}", opt, value)));
                } else {
                    flags.push(OsString::from(opt));
                }
            }

            Flag::Version => {
                flags.push(OsString::from("--version"));
            }

            Flag::Verbose => {
                flags.push(OsString::from("--verbose"));
            }

            Flag::Extern { name, path } => {
                flags.push(OsString::from("--extern"));
                if let Some(path) = path {
                    flags.push(kv(name, path));
                } else {
                    flags.push(OsString::from(name));
                }
            }

            Flag::ExternLocation { name, location } => {
                flags.push(OsString::from("--extern-location"));
                flags.push(kv(name, location));
            }

            Flag::Sysroot(sysroot) => {
                flags.push(OsString::from("--sysroot"));
                flags.push(OsString::from(sysroot));
            }

            Flag::Z(flag) => {
                flags.push(OsString::from("-Z"));
                flags.push(OsString::from(flag));
            }

            Flag::ErrorFormat(error_format) => {
                flags.push(OsString::from("--error-format"));
                flags.push(OsString::from(error_format.to_string()));
            }

            Flag::Json(json) => {
                flags.push(OsString::from("--json"));
                flags.push(OsString::from(json));
            }

            Flag::Color(color) => {
                flags.push(OsString::from("--color"));
                flags.push(OsString::from(color.to_string()));
            }

            Flag::RemapPathPrefix { from, to } => {
                flags.push(OsString::from("--remap-path-prefix"));
                flags.push(kv(from, to));
            }
        }

        iter::Iter {
            items: flags.into_iter(),
        }
    }
}

fn kv(k: impl AsRef<OsStr>, v: impl AsRef<OsStr>) -> OsString {
    let k = k.as_ref();
    let v = v.as_ref();
    let mut string = OsString::with_capacity(k.len() + 1 + v.len());
    string.push(k);
    string.push("=");
    string.push(v);
    string
}

mod iter {
    use std::ffi::OsString;
    use std::vec;

    pub struct Iter {
        pub(crate) items: vec::IntoIter<OsString>,
    }

    impl Iterator for Iter {
        type Item = OsString;

        fn next(&mut self) -> Option<Self::Item> {
            self.items.next()
        }
    }
}
