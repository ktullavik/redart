use std::process;
use token::Token;
use State;


pub fn dprint<S: Into<String>>(s: S) {
    let debug = false;
    if debug {
        println!("{}", s.into());
    }
}


pub fn dart_lexerror<S: Into<String>>(msg: S, line: usize, column: usize, filepath: &str) -> ! {
    println!("{}:{}:{}: Error: {}", filepath, line, column, msg.into());
    process::exit(1);
}


pub fn dart_parseerror<S: Into<String>>(msg: S, state: &State, token: Token) -> ! {

    let (linenum, symnum) = token.find_token_position();

    if state.debug {
        panic!("{}:{}:{}: Error: {}", state.filepath, linenum, symnum, msg.into());
    }
    else {
        println!("{}:{}:{}: Error: {}", state.filepath, linenum, symnum, msg.into());
        process::exit(1);
    }
}


pub fn dart_evalerror<S: Into<String>>(msg: S, state: &State) -> ! {

    if state.debug {
        panic!("{}: Error: {}", state.filepath, msg.into());
    }
    else {
        println!("{}: Error: {}", state.filepath, msg.into());
        process::exit(1);
    }
}
