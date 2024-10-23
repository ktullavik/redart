use std::process;
use crate::object::Object;
use crate::token::Token;
use crate::State;
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


pub fn err_arg_count(
    fname: &str,
    expected: usize,
    got: usize,
    fnode: &Node,
    state: &State) -> ! {

    evalerror(
        format!("Expected {} argument(s) for {}(). Got: {}", got, fname, expected),
        state,
        fnode)
}


pub fn err_arg_type(
    fname: &str,
    expected: &str,
    got: &Object,
    argnode: &Node,
    state: &State) -> ! {

    evalerror(
        format!("Illegal argument '{}' for {}(). Expected type: {}", got, fname, expected),
        state,
        argnode);
}


pub fn check_argc(
    fname: &str,
    expected: usize,
    got: usize,
    fnode: &Node,
    state: &State) {

    if expected != got {
        err_arg_count(fname, expected, got, fnode, state)
    }
}

