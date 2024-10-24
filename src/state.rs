use std::time::Instant;
use std::collections::HashMap;
use crate::stack::Stack;
use crate::objsys::ObjSys;
use crate::objsys::RefKey;
use crate::node::Node;


// filepath:     Name of the file we are currently executing in.
// globals:      Where all the top level nodes (functions, constructors, ...)
//               are actually stored.
// looktables:   Allows, given a filename, to look up the top level nodes that
//               are accessible by the file. It gives an index into globals.
// stack:        Combined call-stack and lexical stack.
// constructing: Tracks references to objects currently being constructed, so
//               we can tell the garbage collector to not delete them.
// eval_var:     When evaluating a top level variable, we must store the name
//               in order to detect cycles.
// in_const:     True when we are evaluating the value of a const variable,
//               so we can avoid stuff that are not allowed.
// start_time:   Timestamp when we started the program, so we can measure time.
// last_gc:      Timestamp when the garbage collector last ran.
// debug:        Enable debug messages.
pub struct State {
    pub filepath: String,
    pub globals: Vec<Node>,
    pub looktables: HashMap<String, HashMap<String, usize>>,
    pub stack: Stack,
    pub objsys: ObjSys,
    pub constructing: Vec<RefKey>,
    pub eval_var: String,
    pub in_const: bool,
    pub start_time: Instant,
    pub last_gc: Instant,
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
            eval_var: String::from(""),
            in_const: false,
            start_time: Instant::now(),
            last_gc: Instant::now(),
            debug: false
        }
    }


    pub fn has_global(&self, name: &str) -> bool {
        self.looktables[&self.filepath].contains_key(name)
    }


    pub fn get_global(&self, name: &str) -> Node {
        self.globals[self.looktables[&self.filepath].get(name).unwrap().clone()].clone()
    }


    pub fn get_global_ref(&self, name: &str) -> &Node {
        &self.globals[self.looktables[&self.filepath].get(name).unwrap().clone()]
    }


    pub fn set_global(&mut self, name: &str, val: Node) {
        self.globals[self.looktables[&self.filepath].get(name).unwrap().clone()] = val;
    }


}
