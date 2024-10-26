use std::fmt;
use crate::node::Node;
use crate::objsys::RefKey;



#[derive(Debug)]
#[derive(Clone)]
pub struct ParamObj {
    pub typ: String,
    pub name: String,
    pub fieldinit: bool,
}


impl fmt::Display for ParamObj {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Param({}, {}, {})", self.typ, self.name, self.fieldinit)
    }

}


// These are the primitive objects. They should
// be passed by value and cloning should be allowed.
// That is, they can't be made to depend on references
// that may refer to stale copies of the object.
#[derive(Clone)]
pub enum Object {
    Int(i64),
    Double(f64),
    Bool(bool),
    String(String),
    // funcname, filename, body, params
    Function(String, String, Node, Vec<ParamObj>),    // funcname, filename, body, params
    Constructor(String, String, Vec<ParamObj>, Node, Node), // consname, filename, params, initlist, body
    Reference(RefKey),
    Null,
    Return(Box<Object>)
}



impl fmt::Display for Object {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            Object::Int(i) => write!(f, "{}", i),
            Object::Double(x) => write!(f, "{}", x),
            Object::Bool(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Function(_, _, _, _) => {
                // Dart prints a function signature, like: (int) => String.
                // But since the function will turn into a closure, it really prints
                // Closure: (int) => String
                // TODO
                write!(f, "() => ?")
            },
            Object::Constructor(_, _, _, _, _) => {
                // TODO
                write!(f, "() => ?")
            },
            Object::Reference(s) => {
                // TODO, need lookup, dont have access.
                write!(f, "Reference<{}>", s)
            },
            Object::Null => write!(f, "null"),
            Object::Return(_) => panic!("Tried to display Return Object")
        }
    }
}
