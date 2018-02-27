#[macro_use]
extern crate icecream;


#[test]
fn smoke_test() {
    ic!();
    ic!(f);
    ic!(ff);

    let x = 99;
    ic!(x);
    ic!(x, f);
    ic!(x, ff);
}


#[cfg(test)]
mod a_module {
    #[test]
    fn some_function() {
        ic!();
        ic!(f);
        ic!(ff);

        let x = 99;
        ic!(x);
        ic!(x, f);
        ic!(x, ff);
    }
}
