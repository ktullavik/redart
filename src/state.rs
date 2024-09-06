use stack::Stack;
use objsys::ObjSys;
use objsys::RefKey;
use node::Node;
use std::collections::HashMap;



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
