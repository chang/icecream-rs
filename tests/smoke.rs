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
    // Wrap expanded code in a block so variables go out of scope.
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
    assert_stdout_eq!(ice!(), "smoke.rs::smoke::test_plain_call::43 ❯");
}

#[test]
fn test_ident_match() {
    let x = 99;
    assert_stdout_eq!(ic!(x), format!("smoke.rs:{} ❯ x = 99", line!()));
    assert_stdout_eq!(ice!(x), format!("smoke.rs::smoke::test_ident_match::{} ❯ x = 99", line!()));
}

#[test]
fn test_expr_match() {
    fn a_function(x: i32) -> i32 {
        x + 2
    }
    assert_stdout_eq!(ic!(a_function(2)), format!("smoke.rs:{} ❯ a_function(2) = 4", line!()));
    assert_stdout_eq!(ice!(a_function(2)), format!("smoke.rs::smoke::test_expr_match::{} ❯ a_function(2) = 4", line!()));
}

#[test]
fn test_set_separator_symbol() {
    icecream::set_arrow_symbol("TEST");
    assert_stdout_eq!(ic!(), format!("smoke.rs:{}TEST", line!()));
    icecream::set_arrow_symbol(" ❯ ");
}

#[test]
fn test_set_equals_symbol() {
    let x = 99;
    icecream::set_equals_symbol("TEST");
    assert_stdout_eq!(ic!(x), format!("smoke.rs:{} ❯ xTEST99", line!()));
    icecream::set_equals_symbol(" = ");
}
