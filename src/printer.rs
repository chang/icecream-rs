use std::fmt::Debug;
use std::path::Path;

use super::parsed_backtrace::ParsedBacktrace;


pub struct Printer {
    pub sep: String,
    pub arrow: String,
    pub eq: String,
}


impl Printer {

    pub fn ic(&self, line: u32, file_path: &str) -> String {
        format!("{file_name}:{line}{arrow}",
                file_name = self.file_name(file_path),
                line = line,
                arrow = self.arrow)
    }

    pub fn ic_expr<T: Debug>(&self, val: &T, expr: &str, line: u32, file_path: &str) -> String {
        format!("{header}{expr}",
                header = self.ic(line, file_path),
                expr = self.expr_string(val, expr))
    }

    // ice!() - parse backtraces for additional information (like calling function).ParsedBacktrace

    pub fn ice(&self, li: ParsedBacktrace) -> String {
        format!("{file}::{module}::{func}::{line}{arrow}",
                file = li.filename(),
                module = li.modname(),
                line = li.lineno(),
                func = li.funcname(),
                arrow = self.arrow)
    }

    pub fn ice_expr<T: Debug>(&self, var: &T, name: &str, li: ParsedBacktrace) -> String {
        format!("{header}{expr}",
                header = self.ice(li),
                expr = self.expr_string(var, name))
    }

    fn file_name(&self, file_path: &str) -> String {
        Path::new(file_path)
            .file_name()
            .expect("file!() didn't return a valid file.")
            .to_str()
            .unwrap()
            .to_string()
    }

    fn expr_string<T: Debug>(&self, var: &T, name: &str) -> String {
        format!("{name}{eq}{value:?}",
                name = name,
                eq = self.eq,
                value = var)
    }

}
