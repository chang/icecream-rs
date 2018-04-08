
#[macro_use]
extern crate lazy_static;
extern crate backtrace;

#[macro_use]
mod macros;
pub mod parsed_backtrace;
pub mod formatter;

use std::sync::RwLock;
pub use backtrace::Backtrace;
pub use formatter::Formatter;

/* Possible symbols -> : | ❯ */
lazy_static! {
    pub static ref PRINTER: RwLock<Formatter> = {
        RwLock::new(
            Formatter {
                sep: String::from(":"),
                arrow: String::from(" ❯ "),
                eq: String::from(" = "),
            }
        )
    };
}

pub fn set_separator_symbol(symbol: &str) {
    PRINTER.write().unwrap().sep = String::from(symbol);
}

pub fn set_arrow_symbol(symbol: &str) {
    PRINTER.write().unwrap().arrow = String::from(symbol);
}

pub fn set_equals_symbol(symbol: &str) {
    PRINTER.write().unwrap().eq = String::from(symbol);
}
