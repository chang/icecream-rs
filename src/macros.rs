extern crate backtrace;


#[macro_export]
macro_rules! header {
    () => (
        println!("Placeholder.");
    );
}

#[macro_export]
macro_rules! full_header {
    // The full header parses a backtrace, so may be significantly slower.
    () => (
        let bt = $crate::Backtrace::new();
        let li = $crate::parsed_backtrace::ParsedBacktrace::new(&bt);
        $crate::short_header(&li);
    );
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
        $crate::print_variable(&$x, stringify!($x));
    });
}
