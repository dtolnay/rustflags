use std::ffi::OsStr;
use std::ops::{Deref, Index, RangeBounds};
use std::str::Chars;

pub(crate) struct EnvString(String);

#[repr(transparent)]
pub(crate) struct EnvStr(str);

impl EnvString {
    pub fn new(encoded: String) -> Self {
        EnvString(encoded)
    }
}

impl Deref for EnvString {
    type Target = EnvStr;

    fn deref(&self) -> &Self::Target {
        EnvStr::new(&self.0)
    }
}

impl EnvStr {
    fn new(encoded: &str) -> &Self {
        unsafe { &*(encoded as *const str as *const EnvStr) }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find(&self, ch: char) -> Option<usize> {
        self.0.find(ch)
    }

    pub fn starts_with(&self, ch: char) -> bool {
        self.0.starts_with(ch)
    }

    pub fn ends_with(&self, ch: char) -> bool {
        self.0.ends_with(ch)
    }

    pub fn chars(&self) -> Chars {
        self.0.chars()
    }

    pub fn split_once(&self, ch: char) -> Option<(&EnvStr, &EnvStr)> {
        let (first, rest) = self.0.split_once(ch)?;
        Some((EnvStr::new(first), EnvStr::new(rest)))
    }

    pub fn to_str(&self) -> Option<&str> {
        Some(&self.0)
    }
}

impl<R> Index<R> for EnvStr
where
    R: RangeBounds<usize>,
{
    type Output = EnvStr;

    fn index(&self, index: R) -> &Self::Output {
        let start = index.start_bound().cloned();
        let end = index.end_bound().cloned();
        EnvStr::new(&self.0[(start, end)])
    }
}

impl AsRef<OsStr> for EnvStr {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl Default for &EnvStr {
    fn default() -> Self {
        EnvStr::new("")
    }
}
