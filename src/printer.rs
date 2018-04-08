use std::fmt::Debug;

use super::parsed_backtrace::ParsedBacktrace;


pub struct Printer {
    pub sep: String,
    pub pad: String,
    pub eq: String,
}


impl Printer {

    pub fn short_header(&self, li: &ParsedBacktrace) {
        println!("{}", self.make_header(li));
    }

    pub fn full_header(&self, li: &ParsedBacktrace) {
        println!("{}", self.make_full_header(li));
    }

    pub fn make_header(&self, li: &ParsedBacktrace) -> String {
        format!("{num} {ss} {func}()",
                num = li.lineno(),
                func = li.funcname(),
                ss = self.sep)
    }

    pub fn make_full_header(&self, li: &ParsedBacktrace) -> String {
        format!("{num} {ss} {file}::{module}::{func}",
                file = li.filename(),
                module = li.modname(),
                num = li.lineno(),
                func = li.funcname(),
                ss = self.pad)
    }

    pub fn print_variable<T: Debug>(&self, var: &T, varname: &str) {
        println!("{} {} = {:?}", self.pad, varname, var);
    }

}
