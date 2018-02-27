use std::path::Path;

use backtrace::Backtrace;

const UNKNOWN_STRING: &str = "?";


#[derive(Debug)]
pub struct LineInfo<'a> {
    // addr: Option<&'a str>,  // TODO
    modname: Option<&'a str>,
    funcname: Option<&'a str>,
    filepath: Option<&'a Path>,
    lineno: Option<u32>,
}


impl<'a> LineInfo<'a> {
    pub fn new(bt: &'a Backtrace) -> Self {
        // TODO: implement this with Backtrace::new_unresolved(), since:
        // Similar to new except that this does not resolve any symbols, this simply captures the backtrace as a list of addresses.
        // At a later time the resolve function can be called to resolve this backtrace's symbols into readable names.
        // This function exists because the resolution process can sometimes take a significant amount of time whereas any one backtrace may only be rarely printed.
        let frames = bt.frames();

        // I think that the 3rd frame is always the function frame.
        let line_frame = frames.into_iter().nth(3).expect("Something went wrong parsing the backtrace.");

        let symbol_list = line_frame.symbols();
        if symbol_list.len() > 1 {
            println!("Multiple symbols found.");
        }

        // Normally there is only one symbol per frame, but sometimes if a number of functions are
        // inlined into one frame then multiple symbols will be returned.
        // The first symbol listed is the "innermost function", whereas the last symbol is the outermost (last caller).
        let symbol = symbol_list.first().unwrap();

        let symbol_name = match symbol.name() {
            Some(x) => x.as_str(),
            None => None,
        };

        // Parse the module name and function name.
        let modname: Option<&'a str>;
        let funcname: Option<&'a str>;
        if let Some(name) = symbol_name {
            let mut attrs = name.split("::");
            modname = attrs.next();
            funcname = attrs.next();
        } else {
            modname = None;
            funcname = None;
        }

        LineInfo {
            modname,
            funcname,
            filepath: symbol.filename(),
            lineno: symbol.lineno(),
        }
    }

    fn some_or_huh(x: Option<&str>) -> &str {
        match x {
            Some(y) => y,
            None => UNKNOWN_STRING,
        }
    }

    pub fn modname(&self) -> &'a str {
        LineInfo::some_or_huh(self.modname)
    }

    pub fn funcname(&self) -> &'a str {
        LineInfo::some_or_huh(self.funcname)
    }

    pub fn filename(&self) -> &'a str {
        match self.filepath {
            Some(path) => {
                let name = path.file_name().expect("Couldn't get file name.");
                LineInfo::some_or_huh(name.to_str())
            }
            None => UNKNOWN_STRING,
        }
    }

    // TODO: make the return type consistent with the others? Ownership issue
    pub fn lineno(&self) -> String {
        match self.lineno {
            Some(no) => no.to_string(),
            None => UNKNOWN_STRING.to_string(),
        }
    }
}


pub fn print_backtrace_info(bt: Backtrace) {
    let frames = bt.frames();
    let mut i = 0;

    for frame in frames {
        println!("Frame: {}", i);
        let symbols = frame.symbols();

        for sym in symbols {
            println!("name: {:?}", sym.name());
            println!("addr: {:?}", sym.addr());
            println!("filename: {:?}", sym.filename());
            println!("lineno: {:?}", sym.lineno());
            println!();
        }
        i += 1;
    }
}
