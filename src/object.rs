use node::Node;
use objsys::RefKey;
use std::fmt;



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


// #[derive(Debug)]
#[derive(Clone)]
pub enum Object {
    Int(i64),
    Double(f64),
    Bool(bool),
    String(String),
    __InternalList(Vec<Object>),
    // funcname, filename, body, params
    Function(String, String, Node, Vec<ParamObj>),    // funcname, filename, body, params
    Constructor(String, String, Node, Vec<ParamObj>), // consname, filename, body, params
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
            Object::__InternalList(ilist) => {
                let len = ilist.len();
                if len == 0 {
                    return write!(f, "[]");
                }
                let mut i = 0;
                _ = write!(f, "[");
                while i < len - 1 {
                    _ = write!(f, "{}", ilist[i]);
                    _ = write!(f, ",");
                    i += 1;
                }
                _ = write!(f, "{}", ilist[i]);
                return write!(f, "]")
            },
            Object::Function(_, _, _, _) => {
                // Dart prints a function signature, like: (int) => String.
                // But since the function will turn into a closure, it really prints
                // Closure: (int) => String
                // TODO
                write!(f, "() => ?")
            },
            Object::Constructor(_, _, _, _) => {
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
