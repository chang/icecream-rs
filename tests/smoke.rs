extern crate gag;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::io::Read;
use gag::BufferRedirect;

#[macro_use]
extern crate icecream;


// These tests must be run single-threaded due to a race condition when capturing stdout,
// but we still need a mutex for mutability.
lazy_static! {
    pub static ref STDOUT: Mutex<BufferRedirect> = {
        Mutex::new(BufferRedirect::stdout().unwrap())
    };
}


// Captures the stdout from a macro call (ic! or ice!) and assert equality with a &str.
#[macro_export]
macro_rules! assert_stdout_eq {
    ($macro_call:expr, $expected:expr) => {{
        let mut buf = STDOUT.lock().unwrap();
        let mut output = String::new();

        buf.read_to_string(&mut output).unwrap();
        output.clear();
        $macro_call;
        buf.read_to_string(&mut output).unwrap();

        assert_eq!(output.trim(), $expected);
    }};
}


#[test]
fn test_plain_call() {
    assert_stdout_eq!(ic!(), "smoke.rs:42 ❯");
    assert_stdout_eq!(ice!(), "smoke.rs::smoke::test_plain_call:43 ❯");
}

#[test]
fn test_ident_match() {
    let x = 99;
    assert_stdout_eq!(ic!(x), format!("smoke.rs:{} ❯ x = 99", line!()));
    assert_stdout_eq!(ice!(x), format!("smoke.rs::smoke::test_ident_match:{} ❯ x = 99", line!()));
    assert_eq!(ic!(x), 99);
    assert_eq!(ice!(x), 99);

    let y = Some(0);
    assert_stdout_eq!(ic!(y), format!("smoke.rs:{} ❯ y = Some(0)", line!()));
    assert_stdout_eq!(ice!(y), format!("smoke.rs::smoke::test_ident_match:{} ❯ y = Some(0)", line!()));
    assert_eq!(ic!(y), Some(0));
    assert_eq!(ice!(y), Some(0));
}

#[test]
fn test_expr_match() {
    fn a_function(x: i32) -> i32 {
        x + 2
    }

    assert_stdout_eq!(ic!(a_function(2)), format!("smoke.rs:{} ❯ a_function(2) = 4", line!()));
    assert_stdout_eq!(ice!(a_function(2)), format!("smoke.rs::smoke::test_expr_match:{} ❯ a_function(2) = 4", line!()));
    assert_stdout_eq!(ic!(a_function(2) + 1), format!("smoke.rs:{} ❯ a_function(2) + 1 = 5", line!()));
    assert_stdout_eq!(ice!(a_function(2) + 1), format!("smoke.rs::smoke::test_expr_match:{} ❯ a_function(2) + 1 = 5", line!()));

    assert_eq!(ic!(a_function(2)), 4);
    assert_eq!(ice!(a_function(2)), 4);
    assert_eq!(ic!(a_function(2) + 1), 5);
    assert_eq!(ice!(a_function(2) + 1), 5);
}

#[test]
fn test_annotation_match() {
    let x = Some(8);
    assert_stdout_eq!(ic!("Note to self.", x), format!("smoke.rs:{} ❯ Note to self.\nx = Some(8)", line!()));
    assert_stdout_eq!(ice!("Note to self.", x), format!("smoke.rs::smoke::test_annotation_match:{} ❯ Note to self.\nx = Some(8)", line!()));
    // assert_eq!(ic!("Note to self.", x), Some(8));
    // assert_eq!(ice!("Note to self.", x), Some(8));
}

#[test]
fn test_set_arrow_symbol() {
    icecream::set_arrow_symbol("TEST");
    assert_stdout_eq!(ic!(), format!("smoke.rs:{}TEST", line!()));
    icecream::set_arrow_symbol(" ❯ ");
}

#[test]
fn test_set_sep_symbol() {
    icecream::set_separator_symbol("TEST");
    assert_stdout_eq!(ic!(), format!("smoke.rsTEST{} ❯", line!()));
    icecream::set_separator_symbol(":");
}

#[test]
fn test_set_equals_symbol() {
    let x = 99;
    icecream::set_equals_symbol("TEST");
    assert_stdout_eq!(ic!(x), format!("smoke.rs:{} ❯ xTEST99", line!()));
    icecream::set_equals_symbol(" = ");
}
