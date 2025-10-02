use std::fmt;
use crate::object::Object;


#[derive(Clone)]
pub enum NodeType {
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    PreIncrement(usize, usize),
    PostIncrement(usize, usize),
    PreDecrement(usize, usize),
    PostDecrement(usize, usize),
    Not(usize, usize),
    LogOr(usize, usize),
    LogAnd(usize, usize),
    BitOr(usize, usize),
    BitXor(usize, usize),
    BitAnd(usize, usize),
    LessThan(usize, usize),
    GreaterThan(usize, usize),
    LessOrEq(usize, usize),
    GreaterOrEq(usize, usize),
    Equal(usize, usize),
    Assign(usize, usize),
    Int(i64, usize, usize),
    Double(f64, usize, usize),
    Str(String, usize, usize),
    Bool(bool, usize, usize),
    Name(String, usize, usize),
    TypedVar(String, String, usize, usize),  // type, name
    TopVar(String, String, Box<Object>, usize, usize), // type, name
    TopVarLazy(String, String, usize, usize),
    ConstTopLazy(String, String, usize, usize),     // type, name
    ConstTopVar(String, String, Box<Object>, usize, usize),
    Conditional(usize, usize),
    If(usize, usize),
    ElseIf(usize, usize),
    Else(usize, usize),
    While(usize, usize),
    DoWhile(usize, usize),
    For(usize, usize),
    Block(usize, usize),
    List(usize, usize),
    CollAccess(usize, usize),
    This(usize, usize),
    Super(usize, usize),
    FunDef(String, String, String, usize, usize), // typename, funcname, filename
    FunCall(String, usize, usize),
    MethodCall(String, Box<Node>, String, usize, usize),  // methodname, owner, filename
    ParamList(usize, usize),
    ArgList(usize, usize),
    ThisFieldInit(String, usize, usize),
    InitList(usize, usize),
    Initializer(usize, usize),
    Return(usize, usize),
    Constructor(String, Box<Node>, Box<Node>, Box<Node>, String, usize, usize), // consname, paramlist, initlist, body, filename
    Null(usize, usize),
}


impl fmt::Display for NodeType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Add(_, _)           => write!(f, "+"),
            NodeType::Sub(_, _)           => write!(f, "-"),
            NodeType::Mul(_, _)           => write!(f, "*"),
            NodeType::Div(_, _)           => write!(f, "/"),
            NodeType::PreIncrement(_, _)  |
            NodeType::PostIncrement(_, _) => write!(f, "++"),
            NodeType::PreDecrement(_, _)  |
            NodeType::PostDecrement(_, _) => write!(f, "--"),
            NodeType::Not(_, _)           => write!(f, "!"),
            NodeType::LogOr(_, _)         => write!(f, "||"),
            NodeType::LogAnd(_, _)        => write!(f, "&&"),
            NodeType::BitOr(_, _)         => write!(f, "|"),
            NodeType::BitXor(_, _)        => write!(f, "^"),
            NodeType::BitAnd(_, _)        => write!(f, "&"),
            NodeType::LessThan(_, _)      => write!(f, "<"),
            NodeType::GreaterThan(_, _)   => write!(f, ">"),
            NodeType::LessOrEq(_, _)      => write!(f, "<="),
            NodeType::GreaterOrEq(_, _)   => write!(f, ">="),
            NodeType::Equal(_, _)         => write!(f, "=="),
            NodeType::Assign(_, _)        => write!(f, "="),
            NodeType::Int(s, _, _)  => write!(f, "{}", s),
            NodeType::Double(s, _, _)  => write!(f, "{}", s),
            NodeType::Str(s, _, _)  => write!(f, "\"{}\"", s),
            NodeType::Bool(v, _, _)   => write!(f, "{}", v),
            NodeType::Name(s, _, _) => write!(f, "{}", s),
            NodeType::TypedVar(typ, name, _, _)                       => write!(f, "{}:{}", name, typ),
            NodeType::TopVar(typ, name, val, _, _)      => write!(f, "TopVar({}, {}, {})", typ, name, val),
            NodeType::TopVarLazy(typ, name, _, _)                     => write!(f, "TopVarLazy({}, {})", typ, name),
            NodeType::ConstTopLazy(typ, name, _, _)                   => write!(f, "ConstLazy({}, {})", name, typ),
            NodeType::ConstTopVar(typ, name, val, _, _) => write!(f, "ConstVar({}, {}, {})", name, typ, val),
            NodeType::Conditional(_, _) => write!(f, "Conditional"),
            NodeType::If(_, _)          => write!(f, "If"),
            NodeType::ElseIf(_, _)      => write!(f, "ElseIf"),
            NodeType::Else(_, _)        => write!(f, "Else"),
            NodeType::While(_, _)       => write!(f, "While"),
            NodeType::DoWhile(_, _)     => write!(f, "DoWhile"),
            NodeType::For(_, _)         => write!(f, "For"),
            NodeType::Block(_, _)       => write!(f, "Block"),
            NodeType::List(_, _)        => write!(f, "[]"),
            NodeType::CollAccess(_, _)  => write!(f, "T[n]"),
            NodeType::This(_, _)        => write!(f, "this"),
            NodeType::Super(_, _)       => write!(f, "super"),
            NodeType::FunDef(_, s, _filename, _, _)          => write!(f, "{}() {{}}", s),
            NodeType::FunCall(s, _, _)                             => write!(f, "{}()", s),
            NodeType::MethodCall(name, owner, _, _, _) => write!(f, "{}.{}()", name, owner),
            NodeType::ParamList(_, _)   => write!(f, "ParamList"),
            NodeType::ArgList(_, _)     => write!(f, "ArgList"),
            NodeType::ThisFieldInit(s, _, _)                      => write!(f, "this.{}", s),
            NodeType::InitList(_, _)                                       => write!(f, "InitList"),
            NodeType::Initializer(_, _)                                    => write!(f, "Initializer"),
            NodeType::Return(_, _)                                         => write!(f, "Return"),
            NodeType::Constructor(name, _, _, _, _, _, _) => write!(f, "Constructor({})", name),
            NodeType::Null(_, _)                                           => write!(f, "null"),
        }
    }
}


// #[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub nodetype: NodeType,
    pub children: Vec<Node>
}


impl Node {

    pub fn new(nodetype: NodeType) -> Node {
        Node {
            nodetype,
            children: Vec::new(),
        }
    }


    pub fn print(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {

        match self.children.len() {

            0 => {
                writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype)
            },

            1 => {
                let res = writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                if let Err(e) = self.children[0].print(f,depth + 1) {
                    println!("Error when printing node children: {}", e.to_string())
                }
                return res
            },

            2 => {
                let res = writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                if let Err(e) = self.children[0].print(f,depth + 1) {
                    println!("Error when printing node children: {}", e.to_string())
                }
                if let Err(e) = self.children[1].print(f,depth + 1) {
                    println!("Error when printing node children: {}", e.to_string())
                }
                return res;
            }

            x => {
                let res = writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                for i in 0 .. x {
                    if let Err(e) = self.children[i].print(f, depth + 1) {
                        println!("Error when printing node children: {}", e.to_string());
                    }
                }
                if let Err(e) = write!(f, "") {
                    println!("Error when printing node: {}", e.to_string())
                }
                return res;
            }
        }
    }


    pub fn find_node_position(&self) -> (usize, usize) {
        return match self.nodetype {
            NodeType::Add(l, i) |
            NodeType::Sub(l, i) |
            NodeType::Mul(l, i) |
            NodeType::Div(l, i) |
            NodeType::PreIncrement(l, i) |
            NodeType::PostIncrement(l, i) |
            NodeType::PreDecrement(l, i) |
            NodeType::PostDecrement(l, i) |
            NodeType::Not(l, i) |
            NodeType::LogOr(l, i) |
            NodeType::LogAnd(l, i) |
            NodeType::BitOr(l, i) |
            NodeType::BitXor(l, i) |
            NodeType::BitAnd(l, i) |
            NodeType::LessThan(l, i) |
            NodeType::GreaterThan(l, i) |
            NodeType::LessOrEq(l, i) |
            NodeType::GreaterOrEq(l, i) |
            NodeType::Equal(l, i) |
            NodeType::Assign(l, i) |
            NodeType::Int(_, l, i) |
            NodeType::Double(_, l, i) |
            NodeType::Str(_, l, i) |
            NodeType::Bool(_, l, i) |
            NodeType::Name(_, l, i) |
            NodeType::TypedVar(_, _, l, i) |
            NodeType::TopVar(_, _, _, l, i) |
            NodeType::TopVarLazy(_, _, l, i) |
            NodeType::ConstTopLazy(_, _, l, i) |
            NodeType::ConstTopVar(_, _, _, l, i) |
            NodeType::Conditional(l, i) |
            NodeType::If(l, i) |
            NodeType::ElseIf(l, i) |
            NodeType::Else(l, i) |
            NodeType::While(l, i) |
            NodeType::DoWhile(l, i) |
            NodeType::For(l, i) |
            NodeType::Block(l, i) |
            NodeType::List(l, i) |
            NodeType::CollAccess(l, i) |
            NodeType::This(l, i) |
            NodeType::Super(l, i) |
            NodeType::FunDef(_, _, _, l, i) |
            NodeType::FunCall(_, l, i) |
            NodeType::MethodCall(_, _, _, l, i) |
            NodeType::ParamList(l, i) |
            NodeType::ArgList(l, i) |
            NodeType::ThisFieldInit(_, l, i) |
            NodeType::InitList(l, i) |
            NodeType::Initializer(l, i) |
            NodeType::Return(l, i) |
            NodeType::Constructor(_, _, _, _, _, l, i) |
            NodeType::Null(l, i)
            => {
                (l.clone(), i.clone())
            }


        }
    }


}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
    }
}
