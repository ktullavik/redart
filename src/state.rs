use stack::Stack;
use objsys::ObjSys;
use objsys::RefKey;
use node::Node;
use std::collections::HashMap;


// globals is where all the top level nodes (functions, constructors)
// are actually stored. looktables allows, given a filename, to look
// up the top level nodes that are accessible by the file. It gives
// an index into globals.
pub struct State {
    pub filepath: String,
    pub globals: Vec<Node>,
    pub looktables: HashMap<String, HashMap<String, usize>>,
    pub stack: Stack,
    pub objsys: ObjSys,
    pub constructing: Vec<RefKey>,
    pub debug: bool
}


impl  State {
    
    pub fn new() -> State {
        State {
            filepath: String::from(""),
            globals: Vec::new(),
            looktables: HashMap::new(),
            stack: Stack::new(),
            objsys: ObjSys::new(),
            constructing: Vec::new(),
            debug: false
        }
    }
}
