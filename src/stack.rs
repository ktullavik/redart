use std::collections::HashMap;
use evaluator::Object;


pub struct Stack {
    pub stack: Vec<HashMap<String, Object>>,
    pub level: usize
}

impl Stack {

    pub fn new() -> Stack {
        Stack {
            stack: Vec::new(),
            level: 0
        }
    }

    // Remove the top stack frame.
    pub fn pop(&mut self) {
        if self.level > 0 {
            self.stack.pop();
            self.level = self.level - 1;
        }
    }


    // Add a new stack frame.
    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
        self.level = self.level + 1;
    }


    // Add a key/object pair to the current stack frame.
    pub fn add(&mut self, s: &str, v: Object) {

        if self.level < 1 {
            panic!("No stack frames in stack!")
        }

        let frame = self.stack.last_mut().unwrap();
        frame.insert(String::from(s), v);
    }


    pub fn has(&self, s: &str) -> bool {

        let mut lv = self.level;
        while lv > 0 {
            let frame : &HashMap<String, Object>  = self.stack.get(lv - 1).unwrap();
            if frame.contains_key(s) {
                return true
            }
            lv = lv - 1;
        }
        false
    }


    // Lookup possibly stored value. Resolving backwards through stack frames.
    pub fn get(&self, s: &str) -> &Object {

        let mut lv = self.level;
        while lv > 0 {
            let frame : &HashMap<String, Object>  = self.stack.get(lv - 1).unwrap();
            if frame.contains_key(s) {
                return frame.get(s).unwrap();
            }
            lv = lv - 1;
        }

        panic!("Undefined variable: {}", s)
    }
}

