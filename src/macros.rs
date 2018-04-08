extern crate backtrace;


#[macro_export]
macro_rules! header {
    () => ({
        println!("Placeholder.");
    });
}

#[macro_export]
macro_rules! full_header {
    // The full header parses a backtrace, so may be significantly slower.
    () => ({
        let bt = $crate::Backtrace::new();
        let li = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
        let printer = $crate::PRINTER.lock().unwrap();
        printer.short_header(&li);
    });
}

#[macro_export]
macro_rules! ice {
    // It's important to wrap the macro in curly braces. This way the expanded
    // macro can be matched as an $expr in other macros.
    () => ({
        full_header!();
    });

    ($x:ident) => ({
        full_header!();
        // TODO: This is really weird. If I write it as
        // let printer = $crate::PRINTER.lock().unwrap();
        // printer.print_variable(&$x, stringify!($x));
        // The test_assert_eq() macro doesn't match correctly. Not sure why.
        $crate::PRINTER.lock().unwrap().print_variable(&$x, stringify!($x));
    });
}
