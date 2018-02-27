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
