use std::path::Path;

use backtrace::{Backtrace, BacktraceSymbol};

const UNKNOWN_STRING: &str = "?";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_info_creation() {
        let bt = Backtrace::new();  // line 14 of parsed_backtrace.rs
        let li = ParsedBacktrace::new(&bt);

        assert_eq!(li.modname, Some("tests"));
        assert_eq!(li.funcname, Some("test_line_info_creation"));
        assert_eq!(li.filepath, Some(Path::new("src/parsed_backtrace.rs")));
        assert_eq!(li.lineno, Some(14));
    }

    #[test]
    fn test_line_info_parsing() {
        let bt = Backtrace::new();
        let li = ParsedBacktrace::new(&bt);

        assert_eq!(li.modname(), "tests");
        assert_eq!(li.funcname(), "test_line_info_parsing");
        assert_eq!(li.filename(), "parsed_backtrace.rs");
        assert_eq!(li.lineno(), "25".to_string());
    }
}


#[derive(Debug)]
pub struct ParsedBacktrace<'a> {
    // addr: Option<&'a str>,  // TODO
    modname: Option<&'a str>,
    funcname: Option<&'a str>,
    filepath: Option<&'a Path>,
    lineno: Option<u32>,
}


impl<'a> ParsedBacktrace<'a> {
    pub fn new(bt: &'a Backtrace) -> Self {
        // TODO: implement this with Backtrace::new_unresolved(), since:
        // Similar to new except that this does not resolve any symbols, this simply captures the backtrace as a list of addresses.
        // At a later time the resolve function can be called to resolve this backtrace's symbols into readable names.
        // This function exists because the resolution process can sometimes take a significant amount of time whereas any one backtrace may only be rarely printed.
        let frames = bt.frames();

        // I think that the 3rd frame is always the function frame.
        let line_frame = frames.into_iter().nth(3).expect("Something went wrong parsing the backtrace.");
        let symbol_list = line_frame.symbols();

        // Normally there is only one symbol per frame, but sometimes if a number of functions are
        // inlined into one frame then multiple symbols will be returned.
        // The first symbol listed is the "innermost function", whereas the last symbol is the outermost (last caller).
        let symbol = symbol_list.first().expect("There should always be at least one symbol.");
        let (modname, funcname) = ParsedBacktrace::parse_module_and_function(&symbol);

        ParsedBacktrace {
            modname,
            funcname,
            filepath: symbol.filename(),
            lineno: symbol.lineno(),
        }
    }

    // Parse the module and function name from a backtrace symbol.
    fn parse_module_and_function(symbol: &'a BacktraceSymbol) -> (Option<&'a str>, Option<&'a str>) {
        let symbol_name = match symbol.name() {
            Some(x) => x.as_str(),
            None => None,
        };

        match symbol_name {
            None => (None, None),
            Some(name) => {
                // The immediate function and module name should be parsed in 'reverse' order. Example:
                // "icecream::parsed_backtrace::tests::test_line_info_parsing::h80b501ffc06f3b37"
                let mut attrs = name.split(':').rev();
                attrs.next();  // Strip mem address
                attrs.next();
                let funcname = attrs.next();  // TODO: splitting on ':' leaves a whitespace. Remove it in a cleaner way.
                attrs.next();
                let modname = attrs.next();
                (modname, funcname)
            }
        }
    }

    fn to_str(x: Option<&str>) -> &str {
        match x {
            Some(y) => y,
            None => UNKNOWN_STRING,
        }
    }

    pub fn modname(&self) -> &'a str {
        ParsedBacktrace::to_str(self.modname)
    }

    pub fn funcname(&self) -> &'a str {
        ParsedBacktrace::to_str(self.funcname)
    }

    pub fn filename(&self) -> &'a str {
        match self.filepath {
            Some(path) => {
                let name = path.file_name().expect("Couldn't get file name.");
                ParsedBacktrace::to_str(name.to_str())
            }
            None => UNKNOWN_STRING,
        }
    }

    // TODO: make the return type consistent with the others. Ownership issue.
    pub fn lineno(&self) -> String {
        match self.lineno {
            Some(no) => no.to_string(),
            None => UNKNOWN_STRING.to_string(),
        }
    }
}
