use std::fmt::Debug;

extern crate backtrace;
pub use backtrace::Backtrace;  // Must "pub use" backtrace so it's accessible from the absolute path
                               // icecream::Backtrace

pub mod lineinfo;
use lineinfo::LineInfo;

#[macro_use]
mod macros;

// TODO: allow these to be configurable by the user
const SEP_SYMBOL: &str = "|";
const PAD_SYMBOL: &str = ">";


pub fn short_header(li: &LineInfo) {
    println!("{num} {ss} {func}()",
             num = li.lineno(),
             func = li.funcname(),
             ss = SEP_SYMBOL);
}

pub fn full_header(li: &LineInfo) {
    println!("{num} {ss} {module}::{func}",
             module = li.modname(),
             num = li.lineno(),
             func = li.funcname(),
             ss = SEP_SYMBOL);
}

pub fn fullfull_header(li: &LineInfo) {
    println!("{num} {ss} {file}::{module}::{func}",
             file = li.filename(),
             module = li.modname(),
             num = li.lineno(),
             func = li.funcname(),
             ss = SEP_SYMBOL);
}

pub fn print_variable<T: Debug>(var: &T) {
    let varname = stringify!(var);
    println!("{} {} = {:?}", PAD_SYMBOL, varname, var);
}
