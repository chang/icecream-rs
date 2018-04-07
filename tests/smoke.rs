#[macro_use]
extern crate icecream;


#[test]
fn smoke_test() {
    ice!();

    let x = 99;
    ice!(x);
}


#[cfg(test)]
mod a_module {
    #[test]
    fn some_function() {
        ice!();

        let x = 99;
        ice!(x);
    }
}
