use std::fmt::Debug;
use std::path::Path;

use super::parsed_backtrace::ParsedBacktrace;


// TODO: this should take closures so they're more customizable.
pub struct Formatter {
    pub sep: String,
    pub arrow: String,
    pub eq: String,
}


impl Formatter {

    fn _expr(&self, header: &str, expr: &str) -> String {
        format!("{header}{expr}",
                header = header,
                expr = expr)
    }

    fn _annotated(&self, header: &str, annotation: &str, expr: &str) -> String {
        format!("{header}{annotation}\n{expr}",
                header = header,
                annotation = annotation,
                expr = expr)
    }

    pub fn ic(&self, line: u32, file_path: &str) -> String {
        format!("{file_name}{sep}{line}{arrow}",
                file_name = self.file_name(file_path),
                sep = self.sep,
                line = line,
                arrow = self.arrow)
    }

    pub fn ic_expr<T: Debug>(&self, val: &T, expr: &str, line: u32, file_path: &str) -> String {
        let header = self.ic(line, file_path);
        let expr = self.expr_string(val, expr);
        self._expr(&header, &expr)
    }

    pub fn ic_annotated<T: Debug>(&self, annotation: &str, val: &T, expr: &str, line: u32, file_path: &str) -> String {
        let header = self.ic(line, file_path);
        let expr = self.expr_string(val, expr);
        self._annotated(&header, annotation, &expr)
    }

    pub fn ice(&self, li: ParsedBacktrace) -> String {
        format!("{file}::{module}::{func}{sep}{line}{arrow}",
                file = li.filename(),
                module = li.modname(),
                func = li.funcname(),
                sep = self.sep,
                line = li.lineno(),
                arrow = self.arrow)
    }

    pub fn ice_expr<T: Debug>(&self, var: &T, name: &str, li: ParsedBacktrace) -> String {
        let header = self.ice(li);
        let expr = self.expr_string(var, name);
        self._expr(&header, &expr)
    }

    pub fn ice_annotated<T: Debug>(&self, annotation: &str, var: &T, name: &str, li: ParsedBacktrace) -> String {
        let header = self.ice(li);
        let expr = self.expr_string(var, name);
        self._annotated(&header, annotation, &expr)
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
