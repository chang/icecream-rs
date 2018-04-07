#[macro_use]
extern crate icecream;
extern crate gag;

use std::io::Read;
use gag::BufferRedirect;

// Captures the stdout from a macro call (ic! or ice!) and assert equality with a &str.
macro_rules! assert_stdout_eq {
    // Wrap expanded code in a block so variables go out of scope.
    ($macro_call:expr, $assertion:expr) => ({
        let mut buf = BufferRedirect::stdout().unwrap(); $macro_call;  // The wrapped macro call must be evaluated on the same line.
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(output.trim(), $assertion);
    })
}


#[test]
fn smoke_test() {
    assert_stdout_eq!(ice!(), "22 | smoke_test()");

    let x = 99;
    assert_stdout_eq!(ice!(x), vec!["25 | smoke_test()", "> x = 99"].join("\n"));
}


#[cfg(test)]
mod a_module {
    #[test]
    fn some_function() {
        // ice!();

        // let x = 99;
        // ice!(x);
    }
}
