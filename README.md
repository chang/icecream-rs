# icecream-rs

Print debugging with inspection for Rust, inspired by [icecream](https://github.com/gruns/icecream) for Python.

I tend to use a lot of print debugging when writing Rust. `icecream` provides the `ic!()` macro to make print debugging more convenient by formatting print statements with helpful information like:
- line number
- calling function
- module name
- file name


### Debugging with `ic!()`

`src/example.rs`

```rust
#[macro_use]
extern crate icecream;

mod a_module {
    fn some_function() {
        let x = Some(99);
        ic!();
    }
}
```

`ic!()` with no parameters prints the line number and calling function:
```
7 | some_function
```

`ic!(f)` (full) prints more information:
```
7 | a_module::some_function
```

`ic!(ff)` (full) prints even more information:
```
7 | example.rs::a_module::some_function
```

`ic!(x)` with a variable prints the name of the variable and the value formatted with `std::fmt::Debug`.
```
7 | some_function
> x = Some(99)
```
