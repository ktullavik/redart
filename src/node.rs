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
    Access,
    Assign,
    Int(String),
    Double(String),
    Str(String),
    Bool(bool),
    Name(String),
    TypedVar(String, String),
    Conditional,
    If,
    ElseIf,
    Else,
    Block,
    List,
    Module,
    FunDef(String),
    FunCall(String),
    MethodCall(String, String),
    ParamList,
    ArgList,
    Return,
    Directives,
    Import,
    Class(String),
    Constructor(String),
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
            NodeType::Access => write!(f, "."),
            NodeType::Int(s)                        => write!(f, "{}", s),
            NodeType::Double(s)                     => write!(f, "{}", s),
            NodeType::Str(s)                        => write!(f, "\"{}\"", s),
            NodeType::Bool(v)                        => write!(f, "{}", v),
            NodeType::Name(s)                       => write!(f, "{}", s),
            NodeType::TypedVar(tp, name)  => write!(f, "{}:{}", name, tp),
            NodeType::FunDef(s)                     => write!(f, "{}() {{}}", s),
            NodeType::FunCall(s)                    => write!(f, "{}()", s),
            NodeType::MethodCall(objname, methname) => write!(f, "{}.{}()", objname, methname),
            NodeType::List => write!(f, "[]"),
            NodeType::ParamList => write!(f, "ParamList"),
            NodeType::ArgList => write!(f, "ArgList"),
            NodeType::Conditional => write!(f, "Conditional"),
            NodeType::If => write!(f, "If"),
            NodeType::ElseIf => write!(f, "ElseIf"),
            NodeType::Else => write!(f, "Else"),
            NodeType::Block => write!(f, "Block"),
            NodeType::Return => write!(f, "Return"),
            NodeType::Module => write!(f, "Module"),
            NodeType::Import => write!(f, "import"),
            NodeType::Directives => write!(f, "Directives"),
            NodeType::Class(s) => write!(f, "Class({})", s),
            NodeType::Constructor(name) => write!(f, "Constructor({})", name),
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub nodetype: NodeType,
    pub children: Vec<Node>
}

