# icecream-rs

<p align="left">
  <a href="https://travis-ci.org/chang/icecream-rs">
    <img src="https://img.shields.io/travis/chang/icecream-rs.svg">
  </a>
  <a href="https://crates.io/crates/icecream">
    <img src="https://img.shields.io/crates/v/icecream.svg">
  </a>
</p>

```rust
let x = vec![1, 2, 3];

// regular print debugging...
println!("x = {:?}", x);       // x = [1, 2, 3]

// icecream print debugging.
ic!(x);                        // main.rs:8 ❯ x = [1, 2, 3]
```

Print debugging with inspection for Rust. Inspired by [icecream](https://github.com/gruns/icecream) for Python.

I tend to do a lot of print debugging when writing Rust. `icecream` provides the `ic!()` and `ice!()` macros to make print debugging more convenient by:

1. Using the`std::fmt::Debug` formatter, by default.
2. Displaying helpful information like line number, calling function, module name, and file name.

### Debugging with `ic!()`

```rust
// src/example.rs
#[macro_use]
extern crate icecream;

mod a_module {
    fn some_function() {
        let x = Some(99);
        ic!(x);
    }
}
```

### Basic usage

#### Plain
`ic!()` prints the filename and line number.
```
example.rs:8 ❯
```

#### Matching on an identifier
`ic!(x)` prints the name of the variable and its value.
```
example.rs:9 ❯ x = Some(99)
```

#### Matching on an expression
`ic!(x.unwrap() + 1)` evaluates the inner expression and prints the resulting value.

```
example.rs:10 ❯ x.unwrap() + 1 = 100
```

#### Printing more information
`ice!()` prints a longer output that includes the calling module and function.
```
example.rs::a_module::some_function:11 ❯
```

### Features

#### Returns its argument
`ic!()` returns its argument, so it can be inserted into any expression.
```rust
let x = Some(99);
assert_eq!(ic!(x), Some(99))   // main.rs:8 ❯ x = Some(99)
```

#### Configurable
You can also configure the characters used for symbols in the print output.
```rust
// main.rs
#[macro_use]
extern crate icecream;

fn main() {
    icecream::set_equals_symbol(" -> ");
    let x = vec![1, 2, 3];                // main.rs:7 ❯ x -> [1, 2, 3]
    ic!(x);
}
```

### Tests

Tests must be run single-threaded with the `--nocapture` flag due to a race condition when printing to stdout.

```
RUST_TEST_THREADS=1 cargo test -- --nocapture
```
