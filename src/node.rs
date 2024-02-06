use std::fmt;


#[derive(Debug)]
#[derive(Clone)]
pub enum NodeType {
    Add,
    Sub,
    Mul,
    Div,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
    Not,
    LogOr,
    LogAnd,
    BitOr,
    BitXor,
    BitAnd,
    LessThan,
    GreaterThan,
    LessOrEq,
    GreaterOrEq,
    Equal,
    Assign,
    Int(i64),
    Double(f64),
    Str(String),
    Bool(bool),
    Name(String),
    TypedVar(String, String),
    Conditional,
    If,
    ElseIf,
    Else,
    While,
    DoWhile,
    For,
    Block,
    List,
    FunDef(String, String), // funcname, filename
    FunCall(String),
    MethodCall(String, Box<Node>, String),  // methodname, owner, filename
    ParamList,
    ArgList,
    ThisFieldInit(String),
    Return,
    Constructor(String, String), // consname, filename
    Null,
}


impl fmt::Display for NodeType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Assign => write!(f, "="),
            NodeType::Add => write!(f, "+"),
            NodeType::Sub => write!(f, "-"),
            NodeType::Mul => write!(f, "*"),
            NodeType::Div => write!(f, "/"),
            NodeType::PreIncrement |
            NodeType::PostIncrement => write!(f, "++"),
            NodeType::PreDecrement |
            NodeType::PostDecrement => write!(f, "--"),
            NodeType::Not => write!(f, "!"),
            NodeType::LogOr => write!(f, "||"),
            NodeType::LogAnd => write!(f, "&&"),
            NodeType::BitOr => write!(f, "|"),
            NodeType::BitXor => write!(f, "^"),
            NodeType::BitAnd => write!(f, "&"),
            NodeType::LessThan => write!(f, "<"),
            NodeType::GreaterThan => write!(f, ">"),
            NodeType::LessOrEq => write!(f, "<="),
            NodeType::GreaterOrEq => write!(f, ">="),
            NodeType::Equal => write!(f, "=="),
            NodeType::Int(s)                        => write!(f, "{}", s),
            NodeType::Double(s)                     => write!(f, "{}", s),
            NodeType::Str(s)                        => write!(f, "\"{}\"", s),
            NodeType::Bool(v)                        => write!(f, "{}", v),
            NodeType::Name(s)                       => write!(f, "{}", s),
            NodeType::TypedVar(tp, name)  => write!(f, "{}:{}", name, tp),
            NodeType::FunDef(s, _filename)                     => write!(f, "{}() {{}}", s),
            NodeType::FunCall(s)                    => write!(f, "{}()", s),
            NodeType::MethodCall(name, owner, _) => write!(f, "{}.{}()", name, owner),
            NodeType::List => write!(f, "[]"),
            NodeType::ParamList => write!(f, "ParamList"),
            NodeType::ArgList => write!(f, "ArgList"),
            NodeType::Conditional => write!(f, "Conditional"),
            NodeType::If => write!(f, "If"),
            NodeType::ElseIf => write!(f, "ElseIf"),
            NodeType::Else => write!(f, "Else"),
            NodeType::While => write!(f, "While"),
            NodeType::DoWhile => write!(f, "DoWhile"),
            NodeType::For => write!(f, "For"),
            NodeType::Block => write!(f, "Block"),
            NodeType::ThisFieldInit(s) => write!(f, "this.{}", s),
            NodeType::Return => write!(f, "Return"),
            NodeType::Constructor(name, _filename) => write!(f, "Constructor({})", name),
            NodeType::Null => write!(f, "null"),
        }
    }
}


#[derive(Debug)]
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
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print(f, 0)
    }
}
