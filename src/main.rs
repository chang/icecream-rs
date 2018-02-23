extern crate backtrace;


fn main() {
    backtrace::trace(|frame| {
        let ip = frame.ip();
        println!("{:?}", ip);
        let symbol_address = frame.symbol_address();
        println!("{:?}", symbol_address);

        // Resolve this instruction pointer to a symbol name
        backtrace::resolve(ip, |symbol| {
            if let Some(name) = symbol.name() {
                println!("{:?}", name);
            }
            if let Some(filename) = symbol.filename() {
                println!("{:?}", filename);
            }
        });

        true
    });

    println!("Hello, world!");
}
