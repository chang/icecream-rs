extern crate backtrace;

#[macro_export]
macro_rules! ic_header {
    () => (
        let bt = $crate::Backtrace::new();
        let li = $crate::lineinfo::LineInfo::new(&bt);
        $crate::short_header(&li);
    );

    // short for "full"
    (f) => (
        let bt = $crate::Backtrace::new();
        let li = $crate::lineinfo::LineInfo::new(&bt);
        $crate::full_header(&li);
    );

    (ff) => (
        let bt = $crate::Backtrace::new();
        let li = $crate::lineinfo::LineInfo::new(&bt);
        $crate::fullfull_header(&li);
    );
}

#[macro_export]
macro_rules! ic_debug_print {
    ($x:ident) => (
        $crate::print_variable(&$x);
    );
}

#[macro_export]
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
