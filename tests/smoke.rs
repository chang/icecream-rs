extern crate gag;
use std::io::Read;
use gag::BufferRedirect;

#[macro_use]
extern crate icecream;

#[macro_use]
mod assert_stdout_eq;


#[test]
fn test_ice() {
    assert_stdout_eq!(ice!(), "14 | test_ice()");

    let x = 99;
    assert_stdout_eq!(ice!(x), vec!["17 | test_ice()", "> x = 99"].join("\n"));
}


#[cfg(test)]
mod a_module {
    use super::*;

    #[test]
    fn some_function() {


    }

}
