use std::ffi::OsString;
use std::fmt;

pub(crate) trait WriteFmt {
    fn write_fmt(&mut self, args: fmt::Arguments);
}

impl WriteFmt for OsString {
    fn write_fmt(&mut self, args: fmt::Arguments) {
        fmt::Write::write_fmt(self, args).unwrap();
    }
}
