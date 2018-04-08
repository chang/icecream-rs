// Captures the stdout from a macro call (ic! or ice!) and assert equality with a &str.
#[macro_export]
macro_rules! assert_stdout_eq {
    // Wrap expanded code in a block so variables go out of scope.
    ($macro_call:expr, $assertion:expr) => ({
        let mut buf = BufferRedirect::stdout().unwrap(); $macro_call;  // The wrapped macro call must be evaluated on the same line.
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!(output.trim(), $assertion);
    })
}
