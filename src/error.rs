use std::process;
use token::Token;
use State;
use crate::node::Node;


pub fn lexerror<S: Into<String>>(msg: S, line: usize, column: usize, filepath: &str) -> ! {
    println!("{}:{}:{}: Error: {}", filepath, line, column, msg.into());
    process::exit(1);
}


pub fn parseerror<S: Into<String>>(msg: S, state: &State, tok: Token) -> ! {

    let (linenum, symnum) = tok.find_token_position();

    if state.debug {
        panic!("{}:{}:{}: Error: {}", state.filepath, linenum, symnum, msg.into());
    }
    else {
        println!("{}:{}:{}: Error: {}", state.filepath, linenum, symnum, msg.into());
        process::exit(1);
    }
}


pub fn evalerror<S: Into<String>>(msg: S, state: &State, node: &Node) -> ! {

    let (linenum, symnum) = node.find_node_position();

    if state.debug {
        panic!("{}:{}:{}: Error: {}", state.filepath, linenum, symnum, msg.into());
    }
    else {
        println!("{}:{}:{}: Error: {}", state.filepath, linenum, symnum, msg.into());
        process::exit(1);
    }
}
