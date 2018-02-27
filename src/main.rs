extern crate backtrace;
use backtrace::Backtrace;

mod lineinfo;
use lineinfo::LineInfo;


fn short_header(li: &LineInfo) {
    println!("{num} | {func}()", num = li.lineno(), func = li.funcname());
}

fn full_header(li: &LineInfo) {
    println!("{num} | {module}::{func}",
             module = li.modname(),
             num = li.lineno(),
             func = li.funcname());
}

fn fullfull_header(li: &LineInfo) {
    println!("{num} | {file}::{module}::{func}",
             file = li.filename(),
             module = li.modname(),
             num = li.lineno(),
             func = li.funcname());
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

macro_rules! ic {
    () => ( ic_header!(); );
    (f) => ( ic_header!(f); );
    (ff) => ( ic_header!(ff); );

    ($x:ident) => (
        ic_header!(f);
        let varname = stringify!($x);
        println!("{} = {:?}", varname, $x);
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
