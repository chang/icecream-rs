use std::fmt::Debug;
use std::sync::{Mutex, RwLock, Arc};

#[macro_use]
extern crate lazy_static;

extern crate backtrace;
pub use backtrace::Backtrace;  // "pub use" so it's accessible from the absolute path icecream::Backtrace.

pub mod parsed_backtrace;
use parsed_backtrace::ParsedBacktrace;

pub mod printer;
pub use printer::Printer;

#[macro_use]
mod macros;

/* Possible symbols -> : | ❯ */
lazy_static! {
    pub static ref PRINTER: RwLock<Printer> = {
        RwLock::new(
            Printer {
                sep: String::from(":"),
                arrow: String::from(" ❯ "),
                eq: String::from(" = "),
            }
        )
    };
}

fn set_separator_symbol(symbol: &str) {
    PRINTER.write().unwrap().sep = String::from(symbol);
}

fn set_arrow_symbol(symbol: &str) {
    PRINTER.write().unwrap().arrow = String::from(symbol);
}

fn set_eq_symbol(symbol: &str) {
    PRINTER.write().unwrap().eq = String::from(symbol);
}
