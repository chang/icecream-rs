extern crate backtrace;


#[macro_export]
macro_rules! ic {
    // It's important to wrap the macro in curly braces. This way the expanded
    // macro can be matched as an $expr in other macros.
    () => {{
        let (line, file) = (line!(), file!());
        let printer = &$crate::PRINTER.read().unwrap();
        println!("{}", printer.ic(line, file));
    }};

    ($x:expr) => {{
        let (line, file) = (line!(), file!());
        let printer = &$crate::PRINTER.read().unwrap();
        println!("{}", printer.ic_expr(&$x, stringify!($x), line, file));
    }};
}


#[macro_export]
macro_rules! ice {
    // It's important to wrap the macro in curly braces. This way the expanded
    // macro can be matched as an $expr in other macros.
    () => {{
        let bt = $crate::Backtrace::new();
        let pb = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
        let printer = &$crate::PRINTER.read().unwrap();
        println!("{}", printer.ice(pb));
    }};

    ($x:expr) => {{
        let bt = $crate::Backtrace::new();
        let pb = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
        let printer = &$crate::PRINTER.read().unwrap();
        println!("{}", printer.ice_expr(&$x, stringify!($x), pb));
    }};
}
