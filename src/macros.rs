extern crate backtrace;


#[macro_export]
macro_rules! header {
    () => ({
        println!("Placeholder.");
    });
}


#[macro_export]
macro_rules! ic {
    // It's important to wrap the macro in curly braces. This way the expanded
    // macro can be matched as an $expr in other macros.
    () => {{
        let line = line!();
        let file = file!();
        let printer = &$crate::PRINTER.read().unwrap();
        println!("{}", printer.ic(line, file));
    }};

    // ($x:ident) => {{
    //     let bt = $crate::Backtrace::new();
    //     let li = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
    //     let printer = $crate::PRINTER.read().unwrap();

    //     // TODO: This is really weird. If I write it as
    //     // let printer = $crate::PRINTER.read().unwrap();
    //     // printer.print_variable(&$x, stringify!($x));
    //     // The test_assert_eq() macro doesn't match correctly. Not sure why.
    //     println!("{}", printer.ice_variable(&$x, stringify!($x), li));
    // }};
}

#[macro_export]
macro_rules! ice {
    // It's important to wrap the macro in curly braces. This way the expanded
    // macro can be matched as an $expr in other macros.
    () => {{
        let bt = $crate::Backtrace::new();
        let li = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
        let printer = &$crate::PRINTER.read().unwrap();
        println!("{}", printer.ice(li));
    }};

    // ($x:ident) => {{
    //     let bt = $crate::Backtrace::new();
    //     let li = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
    //     let printer = $crate::PRINTER.read().unwrap();

    //     // TODO: This is really weird. If I write it as
    //     // let printer = $crate::PRINTER.read().unwrap();
    //     // printer.print_variable(&$x, stringify!($x));
    //     // The test_assert_eq() macro doesn't match correctly. Not sure why.
    //     println!("{}", printer.ice_variable(&$x, stringify!($x), li));
    // }};
}
