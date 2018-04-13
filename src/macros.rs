extern crate backtrace;


#[macro_export]
macro_rules! ic {
    () => {{
        let (line, file) = (line!(), file!());
        let formatter = $crate::PRINTER.read().unwrap();
        println!("{}", formatter.ic(line, file));
    }};

    ($x:expr) => {{
        let (line, file) = (line!(), file!());
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ic_expr(&$x, stringify!($x), line, file));
        $x
    }};

    ($annotation:expr, $x:expr) => {{
        let (line, file) = (line!(), file!());
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ic_annotated($annotation, &$x, stringify!($x), line, file));
        $x
    }};
}


#[macro_export]
macro_rules! ice {
    // Wrap the result in a block so the expanded code can be matched as an $expr in the testing macro.
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
        $x
    }};

    ($annotation: expr, $x:expr) => {{
        let backtrace = $crate::Backtrace::new();
        let parsed = $crate::parsed_backtrace::ParsedBacktrace::new(&backtrace);
        let formatter = &$crate::PRINTER.read().unwrap();
        println!("{}", formatter.ice_annotated($annotation, &$x, stringify!($x), parsed));
        $x
    }};
}
