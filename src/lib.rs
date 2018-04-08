use std::fmt::Debug;
use std::sync::Mutex;

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
    pub static ref PRINTER: Mutex<Printer> = {
        Mutex::new(
            Printer {
                sep: String::from("|"),
                pad: String::from(">"),
                eq: String::from("="),
            }
        )
    };
}

fn set_separator_symbol(symbol: String) {
    PRINTER.lock().unwrap().sep = symbol;
}

fn set_padding_symbol(symbol: String) {
    PRINTER.lock().unwrap().pad = symbol;
}

fn set_eq_symbol(symbol: String) {
    PRINTER.lock().unwrap().eq = symbol;
}
