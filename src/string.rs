use std::cmp;
use std::ffi::{OsStr, OsString};
use std::ops::{Deref, Index, Range, RangeFrom, RangeTo};
use std::str;

pub(crate) struct EnvString(OsString);

#[repr(transparent)]
pub(crate) struct EnvStr(OsStr);

impl EnvString {
    pub fn new(encoded: OsString) -> Self {
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
    fn new(encoded: &OsStr) -> &Self {
        unsafe { &*(encoded as *const OsStr as *const EnvStr) }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn find(&self, ch: char) -> Option<usize> {
        let mut buf = [0u8; 4];
        let ch = ch.encode_utf8(&mut buf).as_bytes();
        for i in 0..self.0.len() {
            if self.0.as_encoded_bytes()[i..].starts_with(ch) {
                return Some(i);
            }
        }
        None
    }

    pub fn starts_with(&self, ch: char) -> bool {
        let mut buf = [0u8; 4];
        let ch = ch.encode_utf8(&mut buf).as_bytes();
        self.0.as_encoded_bytes().starts_with(ch)
    }

    pub fn ends_with(&self, ch: char) -> bool {
        let mut buf = [0u8; 4];
        let ch = ch.encode_utf8(&mut buf).as_bytes();
        self.0.as_encoded_bytes().ends_with(ch)
    }

    pub fn first_char(&self) -> Option<EnvChar> {
        let encoded = self.0.as_encoded_bytes();
        let prefix = cmp::min(encoded.len(), 4);
        match str::from_utf8(&encoded[..prefix]) {
            Ok(valid) => valid.chars().next().map(EnvChar::Valid),
            Err(utf8_error) => {
                let valid_up_to = utf8_error.valid_up_to();
                if valid_up_to == 0 {
                    Some(EnvChar::Invalid)
                } else {
                    let valid = str::from_utf8(&encoded[..valid_up_to]).unwrap();
                    Some(EnvChar::Valid(valid.chars().next().unwrap()))
                }
            }
        }
    }

    pub fn split_once(&self, ch: char) -> Option<(&EnvStr, &EnvStr)> {
        let i = self.find(ch)?;
        Some((&self[..i], &self[i + ch.len_utf8()..]))
    }

    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str()
    }
}

impl Index<RangeFrom<usize>> for EnvStr {
    type Output = EnvStr;

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self[range.start..self.len()]
    }
}

impl Index<RangeTo<usize>> for EnvStr {
    type Output = EnvStr;

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self[0..range.end]
    }
}

impl Index<Range<usize>> for EnvStr {
    type Output = EnvStr;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        fn is_utf8(slice: &[u8]) -> bool {
            str::from_utf8(slice).is_ok()
        }

        let encoded = self.0.as_encoded_bytes();
        for i in [range.start, range.end] {
            assert!(
                i == 0
                    || i == encoded.len()
                    || encoded.get(i..i.wrapping_add(1)).is_some_and(is_utf8)
                    || encoded.get(i..i.wrapping_add(2)).is_some_and(is_utf8)
                    || encoded.get(i..i.wrapping_add(3)).is_some_and(is_utf8)
                    || encoded.get(i..i.wrapping_add(4)).is_some_and(is_utf8)
                    || encoded.get(i.wrapping_sub(1)..i).is_some_and(is_utf8)
                    || encoded.get(i.wrapping_sub(2)..i).is_some_and(is_utf8)
                    || encoded.get(i.wrapping_sub(3)..i).is_some_and(is_utf8)
                    || encoded.get(i.wrapping_sub(4)..i).is_some_and(is_utf8)
            );
        }

        let output = &encoded[range];
        EnvStr::new(unsafe { OsStr::from_encoded_bytes_unchecked(output) })
    }
}

impl AsRef<OsStr> for EnvStr {
    fn as_ref(&self) -> &OsStr {
        &self.0
    }
}

impl Default for &EnvStr {
    fn default() -> Self {
        EnvStr::new(OsStr::new(""))
    }
}

pub(crate) enum EnvChar {
    Valid(char),
    Invalid,
}
