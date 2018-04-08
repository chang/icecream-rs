use std::fmt::Debug;
use std::path::Path;

use super::parsed_backtrace::ParsedBacktrace;


pub struct Printer {
    pub sep: String,
    pub arrow: String,
    pub eq: String,
}


impl Printer {

    // ic!()

    pub fn ic(&self, line: u32, file_path: &str) -> String {
        format!("{header}{arrow}",
                header = self.header_string(line, file_path),
                arrow = self.arrow)
    }

    // ice!() - parse backtraces for additional information (like calling function).ParsedBacktrace

    pub fn ice(&self, li: ParsedBacktrace) -> String {
        format!("{header}{arrow}",
                header = self.full_header_string(li),
                arrow = self.arrow)
    }

    pub fn ice_variable<T: Debug>(&self, var: &T, name: &str, li: ParsedBacktrace) -> String {
        format!("{arrow} {e}",
                arrow = self.arrow,
                e = self.variable_string(var, name))
    }

    pub fn full_header(&self, li: ParsedBacktrace) -> String {
        format!("{}", self.full_header_string(li))
    }

    fn file_name(&self, file_path: &str) -> String {
        Path::new(file_path)
            .file_name()
            .expect("file!() didn't return a valid file.")
            .to_str()
            .unwrap()
            .to_string()
    }

    fn header_string(&self, line: u32, file_path: &str) -> String {
        format!("{file_name}:{line}",
                file_name = file_path,
                line = line)
    }

    fn full_header_string(&self, li: ParsedBacktrace) -> String {
        format!("{file}::{module}::{func}::{line}",
                file = li.filename(),
                module = li.modname(),
                line = li.lineno(),
                func = li.funcname())
    }

    fn variable_string<T: Debug>(&self, var: &T, name: &str) -> String {
        format!("{name} {eq} {value:?}",
                name = name,
                eq = self.eq,
                value = var)
    }

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_header() {


    }

}
