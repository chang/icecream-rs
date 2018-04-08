extern crate gag;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use std::io::Read;
use gag::BufferRedirect;

#[macro_use]
extern crate icecream;

// These tests must be run single-threaded due to a race condition when capturing stdout.
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
fn test_ice() {
    assert_stdout_eq!(ice!(), format!("smoke.rs::smoke::test_ice::{} ❯", line!()));

    let x = 99;
    // assert_stdout_eq!(ice!(x), vec!["17 ❯ test_ice()", "❯ x = 99"].join("\n"));
}


#[test]
fn test_ic() {
    assert_stdout_eq!(ic!(), format!("tests/smoke.rs:{} ❯", line!()));

    let x = 99;
    // assert_stdout_eq!(ic!(x), vec!["17 ❯ test_ic()", "❯ x = 99"].join("\n"));
}



#[cfg(test)]
mod a_module {
    use super::*;

    #[test]
    fn some_function() {


    }

}
