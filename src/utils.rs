use std::process;
use token::Token;


pub fn dprint<S: Into<String>>(s: S) {
    let debug = true;

    if debug {
        println!("{}", s.into());
    }
}


pub fn dart_parseerror<S: Into<String>>(msg: S, filename: S, tokens: &Vec<Token>, index: usize) -> ! {
    let debug = false;

    let (linenum, symnum) = &tokens[index].find_token_position(tokens, index);

    if debug {
        panic!("{}", msg.into())
    }
    else {
        println!("{}:{}:{}: Error: {}", filename.into(), linenum, symnum, msg.into());
        process::exit(1);
    }
}


pub fn dart_evalerror<S: Into<String>>(msg: S) -> ! {
    let debug = false;

    if debug {
        panic!("{}", msg.into())
    }
    else {
        println!("{}", msg.into());
        process::exit(1);
    }
}
