use std::collections::HashMap;
use object::Object;


pub struct Stack {
    // pub stack: Vec<HashMap<String, Object>>,

    // Two dimensional storage of "stack" data.
    //
    // First level is call-stack, getting pushed and popped
    // between function calls. Lookups should not cross
    // function-call boundaries.
    // Examples:
    //  - callé should not have access to the vars of its callers.
    //  - caller should have restored acces to its local vars
    //    after calling a function.
    //
    // Second level is lexical scope, getting pushed and popped
    // on blocks within a local context. Lexial lookups should
    // traverse up the lex-stack.
    // Examples:
    //  - loop-block within a local function/method.
    //  - function def within function.
    //
    // A 'frame' means current lex-frame in the current call-frame.
    pub stack: Vec<Vec<HashMap<String, Object>>>,
    pub call_level: usize,
    pub lex_level: usize
}


impl Stack {

    pub fn new() -> Stack {
        Stack {
            stack: Vec::new(),
            call_level: 0,
            lex_level: 0
        }
    }


    // Add a new frame to the call-stack - a new function scope (lex-stack).
    pub fn push_call(&mut self) {
        let mut lexstack = Vec::new();
        lexstack.push(HashMap::new());
        self.stack.push(lexstack);
        self.call_level += 1;
        self.lex_level = 1;
    }


    // Remove the top frame from call-stack.
    pub fn pop_call(&mut self) {
        if self.call_level > 0 {
            self.stack.pop();
            self.call_level -= 1;
            if !self.stack.is_empty() {
                self.lex_level = self.stack.last().unwrap().len();
            }
        }
    }


    // Add a new frame to the lex-stack - a new lexical scope.
    pub fn push_lex(&mut self) {
        let lexframe : HashMap<String, Object> = HashMap::new();
        let callframe = self.stack.last_mut().unwrap();
        callframe.push(lexframe);
        self.lex_level += 1;
    }


    // Remove the top frame from lexical stack.
    pub fn pop_lex(&mut self) {
        if self.lex_level > 1 {
            self.stack.last_mut().unwrap().pop();
        }
        else {
            panic!("Tried to pop last lex-frame!");
        }
        self.lex_level -= 1;
    }


    // Add a new key-value pair to the current frame.
    pub fn add(&mut self, s: &str, v: Object) {
        let callframe = self.stack.last_mut().unwrap();
        let lexframe = callframe.last_mut().unwrap();
        lexframe.insert(String::from(s), v);
    }


    // Searches backwards through current lexical stack frames to find s.
    pub fn has(&self, s: &str) -> bool {
        let callframe = self.stack.last().unwrap();

        let mut ll = self.lex_level;

        while ll > 0 {
            let lexframe : &HashMap<String, Object> = callframe.get(ll - 1).unwrap();
            if lexframe.contains_key(s) {
                return true
            }
            ll = ll - 1;
        }
        return false
    }


    pub fn has_in_lexscope(&self, s: &str) -> bool {
        self.stack.last().unwrap().last().unwrap().contains_key(s)
    }


    // Searches backwards through current lexical stack frames to find and return s.
    pub fn get(&self, s: &str) -> &Object {
        let callframe = self.stack.last().unwrap();

        let mut ll = self.lex_level;
        while ll > 0 {
            let lexframe : &HashMap<String, Object>  = callframe.get(ll - 1).unwrap();
            if lexframe.contains_key(s) {
                return lexframe.get(s).unwrap();
            }
            ll = ll - 1;
        }

        panic!("Undefined variable: {}", s)
    }


    pub fn printstack(&self) {
        println!();
        println!("STACKSTORE:");
        println!();
        let mut cl = self.call_level;

        while cl > 0 {
            let callframe = self.stack.get(cl - 1).unwrap();
            let mut ll = callframe.len();
            while ll > 0 {
                let lexframe = callframe.get(ll - 1).unwrap();

                println!("level {},{}:", cl, ll);
                for (k, v) in lexframe {
                    println!("{} : {:?}", k, v);
                }
                ll -= 1;
            }
            cl -= 1;
        }

    }
}

