extern crate backtrace;


#[macro_export]
macro_rules! ic {
    () => {{
        let (line, file) = (line!(), file!());
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ic(line, file));
    }};

    ($x:expr) => {{
        let (line, file) = (line!(), file!());
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ic_expr(&$x, stringify!($x), line, file));
    }};
}


#[macro_export]
macro_rules! ice {
    // It's important to wrap the macro in curly braces. This way the expanded
    // macro can be matched as an $expr in other macros.
    () => {{
        let backtrace = $crate::Backtrace::new();
        let parsed = $crate::parsed_backtrace::ParsedBacktrace::new(&backtrace);
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ice(parsed));
    }};

    ($x:expr) => {{
        let backtrace = $crate::Backtrace::new();
        let parsed = $crate::parsed_backtrace::ParsedBacktrace::new(&backtrace);
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ice_expr(&$x, stringify!($x), parsed));
    }};
}
