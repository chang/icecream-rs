extern crate backtrace;
use backtrace::Backtrace;

mod lineinfo;
use lineinfo::LineInfo;

// TODO: allow these to be configurable by the user
const SEP_SYMBOL: &str = "|";
const PAD_SYMBOL: &str = ">";


fn short_header(li: &LineInfo) {
    println!("{num} {ss} {func}()",
             num = li.lineno(),
             func = li.funcname(),
             ss = SEP_SYMBOL);
}

fn full_header(li: &LineInfo) {
    println!("{num} {ss} {module}::{func}",
             module = li.modname(),
             num = li.lineno(),
             func = li.funcname(),
             ss = SEP_SYMBOL);
}

fn fullfull_header(li: &LineInfo) {
    println!("{num} {ss} {file}::{module}::{func}",
             file = li.filename(),
             module = li.modname(),
             num = li.lineno(),
             func = li.funcname(),
             ss = SEP_SYMBOL);
}

macro_rules! ic_header {
    () => (
        let bt = Backtrace::new();
        let li = LineInfo::new(&bt);
        short_header(&li);
    );

    // short for "full"
    (f) => (
        let bt = Backtrace::new();
        let li = LineInfo::new(&bt);
        full_header(&li);
    );

    (ff) => (
        let bt = Backtrace::new();
        let li = LineInfo::new(&bt);
        fullfull_header(&li);
    );
}

macro_rules! ic_debug_print {
    ($x:ident) => (
        let varname = stringify!($x);
        println!("{} {} = {:?}", PAD_SYMBOL, varname, $x);
    );
}

macro_rules! ic {
    () => ( ic_header!(); );
    (f) => ( ic_header!(f); );
    (ff) => ( ic_header!(ff); );

    ($x:ident) => (
        ic_header!();
        ic_debug_print!($x);
    );

    ($x:ident, f) => (
        ic_header!(f);
        ic_debug_print!($x);
    );

    ($x:ident, ff) => (
        ic_header!(ff);
        ic_debug_print!($x);
    );
}

fn main() {
    ic!();
    ic!(f);
    ic!(ff);

    let test = 1;
    ic!(test);

    // Compiler built-in macros for inspection.
    // let this_file = file!();
    // println!("test");
    // let this_line = line!();
    // println!("File: {}, Line: {}", this_file, this_line);
    // let some_variable = 1;
    // println!("Variable: {}", stringify!(some_variable));
}
