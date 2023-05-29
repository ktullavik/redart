use std::fmt;
use utils::{dprint, darterror};


#[derive(PartialEq)]
pub enum Token {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Increment,
    Decrement,
    // Logic
    Not,
    LogOr,
    LogAnd,
    BitOr,
    BitAnd,
    // Relation
    LessThan,
    GreaterThan,
    LessOrEq,
    GreaterOrEq,
    Equal,
    // Primitive
    Int(String),
    Double(String),
    Str(String),
    Bool(bool),
    Name(String),
    // Structure
    Class,
    If,
    Else,
    Paren1,
    Paren2,
    Block1,
    Block2,
    Brack1,
    Brack2,
    Comma,
    // Other
    Assign,
    Access,
    Return,
    Import,
    EndSt,
    End
}


#[derive(Debug)]
#[derive(PartialEq)]
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


impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Arithmetic
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            // Logic
            Token::Not => write!(f, "!"),
            Token::LogOr => write!(f, "||"),
            Token::LogAnd => write!(f, "&&"),
            Token::BitOr => write!(f, "|"),
            Token::BitAnd => write!(f, "&"),
            // Relation
            Token::LessThan    => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessOrEq    => write!(f, "<="),
            Token::GreaterOrEq => write!(f, ">="),
            Token::Equal => write!(f, "=="),
            // Primitive
            Token::Int(s)     => write!(f, "{}", s),
            Token::Double(s)     => write!(f, "{}", s),
            Token::Str(s)  => write!(f, "\"{}\"", s),
            Token::Bool(v)     => write!(f, "{}", v),
            Token::Name(s)    => write!(f, "{}", s),
            // Structure
            Token::Class => write!(f, "class"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Paren1 => write!(f, "("),
            Token::Paren2 => write!(f, ")"),
            Token::Block1 => write!(f, "{{"),
            Token::Block2 => write!(f, "}}"),
            Token::Brack1 => write!(f, "["),
            Token::Brack2 => write!(f, "]"),
            Token::Comma => write!(f, ","),
            // Other
            Token::Assign => write!(f, "="),
            Token::Access => write!(f, "."),
            Token::Return => write!(f, "return"),
            Token::Import => write!(f, "import"),
            Token::EndSt => write!(f, "ENDST"),
            Token::End => write!(f, "END"),
        }
    }
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
            NodeType::BitAnd => write!(f, "&"),
            NodeType::LessThan => write!(f, "<"),
            NodeType::GreaterThan => write!(f, ">"),
            NodeType::LessOrEq => write!(f, "<="),
            NodeType::GreaterOrEq => write!(f, ">="),
            NodeType::Equal => write!(f, "=="),
            NodeType::Access => write!(f, "."),
            NodeType::Int(s)                        => write!(f, "{}", s),
            NodeType::Double(s)                     => write!(f, "{}", s),
            NodeType::Str(s)                     => write!(f, "\"{}\"", s),
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


pub fn parse(tokens: &Vec<Token>) -> Result<Node, String> {
    dprint(" ");
    dprint("PARSE");
    dprint(" ");

    let mut root = Node::new(NodeType::Module);
    let directive_node = directives(tokens, 0);
    let mut i = directive_node.1;
    root.children.push(directive_node.0);


    while i < tokens.len() - 1 {
        let (funnode, new_pos) = fundef(tokens, i);
        root.children.push(funnode);

        dprint(format!("Parse: read len: {}", new_pos));
        i = new_pos;
    }

    if i < tokens.len() - 1 {
        return Err(format!("Expected end of input, found {} at {}", tokens[i], i))
    }
    else if i > tokens.len() - 1 {
        return Err(format!("Index returned beyond end of token array. Index: {}, len: {}", i, tokens.len()))
    }

    dprint(format!("Parse: finished at index: {}", i));
    Ok(root)
}


fn directives(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;
    let mut directives_node = Node::new(NodeType::Directives);

    while i < tokens.len() {

        match &tokens[i] {
            Token::Import => {

                let mut node = Node::new(NodeType::Import);

                i += 1;
                if let Token::Str(s) = &tokens[i] {
                    node.children.push(Node::new(NodeType::Str(s.clone())));

                    i += 1;
                    if let Token::EndSt = tokens[i] {
                        i += 1;

                        directives_node.children.push(node);
                    }
                    else {
                        panic!("Error: Expected ';' after import.")
                    }
                }
                else {
                    panic!("Error: Expected string after 'import'.")
                }
            }
            _  => break
        }
    }

    (directives_node, i)
}


fn fundef(tokens: &Vec<Token>, pos: usize) -> (Node, usize)  {

    let mut i: usize = pos;
    let t: &Token = tokens.get(i).unwrap();
    i += 1;

    match t {
        Token::Name(s) => {
            dprint(format!("fundef found NAME {}", s));

            let t2: &Token = tokens.get(i).unwrap();
            i += 1;

            match t2 {

                Token::Name(fname) => {
                    let mut node = Node::new(NodeType::FunDef(fname.to_string()));
                    dprint("Calling paramlist from fundef");
                    let (params, new_pos) = paramlist(tokens, i);
                    i = new_pos;
                    node.children.push(params);

                    let t3: &Token = tokens.get(i).unwrap();
                    i += 1;

                    match t3 {
                        Token::Block1 => {
                            // Could increment i here. But is it better for block parse to
                            // just expect starting at '{'?
                            let (body, new_pos) = block(tokens, i);
                            node.children.push(body);
                            i = new_pos;
                            dprint(format!("Parse: fundef parsed to {}", new_pos));
                            return (node, i)
                        }

                        _ => {
                            panic!("Expected {{. Got: {}", t3)
                        }
                    }
                }

                _ => {
                    panic!("Expected function name.")
                }
            }
        }

        Token::Class => {
            let (cnode, new_pos) = class(tokens, i);
            dprint(format!("parsed class to pos {}", new_pos));
            return (cnode, new_pos);
        }

        Token::Import => {
            // As Dart.
            panic!("Directives must appear before any declarations.");
        }

        x => {
            panic!("Expected top level declaration. Got {}", x);
        }
    }
}


fn class(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    match &tokens[i] {
        Token::Name(classname) => {

            let mut classnode = Node::new(NodeType::Class(classname.clone()));
            i += 1;

            if let Token::Block1 = tokens[i] {
                i += 1;

                let (members, new_pos) = readmembers(classname.clone(), tokens, i);
                classnode.children = members;
                i = new_pos;
                // }

                if let Token::Block2 = tokens[i] {
                    return (classnode, i + 1)
                }
                panic!("Expected '}' to end class.")
            }
            else {
                panic!("Error: Expected '{' after class name")
            }

        }
        x => {
            panic!("Error: Expected class name. Got: {}", x)
        }
    }
}


fn readmembers(classname: String, tokens: &Vec<Token>, pos: usize) -> (Vec<Node>, usize) {
    // Expecting member declaration - field or method.

    let mut i = pos;
    let mut members : Vec<Node> = Vec::new();

    while i < tokens.len() {

        match &tokens[i] {

            Token::Name(mtype) => {

                if *mtype == classname {
                    // Constructor
                    dprint("Found constructor");

                    i += 1;

                    let mut constructor_node = Node::new(NodeType::Constructor(classname.clone()));
                    let (params, new_pos) = paramlist(tokens, i);
                    let (body, new_pos)  = block(tokens, new_pos + 1);
                    i = new_pos;

                    constructor_node.children.push(params);
                    constructor_node.children.push(body);


                    members.push(constructor_node);
                    continue;
                }

                i += 1;


                match &tokens[i] {
                    Token::Name(fieldname) => {
                        i += 1;

                        match &tokens[i] {
                            Token::Paren1 => {
                                // Method
                                dprint("Found method");

                                let mut method_node = Node::new(NodeType::FunDef(fieldname.clone()));
                                let (param_node, new_pos) = paramlist(tokens, i);
                                i = new_pos;

                                if let Token::Block1 = tokens[i] {
                                    i += 1;
                                    let (body, new_pos) = block(tokens, i);
                                    i = new_pos;

                                    method_node.children.push(param_node);
                                    method_node.children.push(body);

                                    members.push(method_node);
                                    // return (method_node, i)
                                }
                                else {
                                    panic!("Expected opening brace in method declaration: '{'")
                                }
                            }

                            Token::EndSt => {
                                // Uninitialized field declare
                                dprint("Found uninitialized field");

                                let fieldnode = Node::new(NodeType::TypedVar(mtype.clone(), fieldname.clone()));
                                members.push(fieldnode);
                                i += 1;
                            }

                            Token::Assign => {
                                // Initialized field declare
                                dprint("Found initialized field");


                                i += 1;

                                let (val, new_pos) = expression(tokens, i);
                                i = new_pos;

                                if let Token::EndSt = tokens[i] {
                                    dprint("Got endst after field init");
                                    i += 1;
                                }
                                else {
                                    panic!("Expected ';' after field initialization.")
                                }

                                let mut eqnode = Node::new(NodeType::Assign);
                                let fieldnode = Node::new(NodeType::TypedVar(mtype.clone(), fieldname.clone()));

                                eqnode.children.push(fieldnode);
                                eqnode.children.push(val);
                                members.push(eqnode);
                            }

                            Token::Block2 => {
                                break;
                            }

                            x => panic!("Unexpected token when parsing class member: '{}'", x)
                        }
                    }

                    Token::Block2 => {
                        break;
                    }

                    x => panic!("Expected class member declaration. Got: '{}'", x)
                }
            }

            Token::Block2 => {
                break;
            }

            x => panic!("Unexpected first token when parsing class member: {}", x)
        }
    }

    (members, i)
}


fn paramlist(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Paramlist on {}", tokens[pos]));

    let mut i = pos;

    if let Token::Paren1 = &tokens[i] {

        let mut node = Node::new(NodeType::ParamList);
        let mut expect_comma = false;
        i += 1;

        while i < tokens.len() {

            match &tokens[i] {

                Token::Paren2 => {
                    return (node, i + 1);
                }

                Token::Comma => {
                    if !expect_comma {
                        panic!("Error: Unexpected separator in parameter list: ','.");
                    }
                    i += 1;
                    expect_comma = false;
                }

                Token::Name(s) => {
                    let paramnode= Node::new(NodeType::Name(s.to_string()));
                    node.children.push(paramnode);
                    expect_comma = true;
                    i += 1;
                }

                _ => {
                    panic!("Unexpected token when reading parameters: {}", &tokens[i])
                }
            }
        }
    }
    else {
        darterror("A function declaration needs an explicit list of parameters.")
    }
    panic!("Error when reading param list.")
}


fn arglist(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    if let Token::Paren1 = &tokens[i] {

        let mut node = Node::new(NodeType::ArgList);
        let mut expect_comma = false;
        i += 1;

        while i < tokens.len() {

            match &tokens[i] {

                Token::Paren2 => {
                    return (node, i + 1);
                }

                Token::Comma => {
                    if !expect_comma {
                        panic!("Error: Unexpected separator in arg list: ','.");
                    }
                    i += 1;
                    expect_comma = false;
                }

                _ => {
                    if expect_comma {
                        panic!("Error: Expected separator in arg list.");
                    }
                    let (arg, new_pos) = expression(tokens, i);
                    node.children.push(arg);
                    i = new_pos;
                    expect_comma = true;
                }
            }
        }
    }
    else {
        panic!("Error: Expected start of arglist: '('. Found: {}", &tokens[i])
    }
    panic!("Error when reading arg list.")
}


/// Parse a series of statements.
///
/// Expects first token after block started by {.
/// Consumes the end-block token }.
fn block(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut node = Node::new(NodeType::Block);
    let mut i = pos;

    while i < tokens.len() {
        dprint(format!("Parse: block loop at: {}, token: {}", i, &tokens[i]));

        if tokens[i] == Token::Block2 {
            dprint(String::from("Parse: token is end-of-block, breaking."));
            i += 1;
            break;
        }
        if tokens[i] == Token::End {
            dprint(String::from("Parse: token is end, breaking."));
            break;
        }

        let (snode, new_pos) = statement(tokens, i);
        node.children.push(snode);

        i = new_pos;

        match &tokens[i] {

            Token::Block2 => {
                // i += 1;
                continue;
            }
            Token::EndSt => {
                // ENDST should be consumed by statement?
                i += 1;
                continue;
            }
            _ => continue
        }
    }

    return (node, i)
}


fn statement(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: statement: {}", &tokens[pos]));

    let mut i = pos;

    match &tokens[i] {

        Token::Name(s) => {

            i = i + 1;
            let t2 = &tokens[i];

            match t2 {

                Token::Name(name) => {
                    // Two names in a row indicate a typed variable or function definition.
                    i += 1;
                    let t3 = &tokens[i];
                    i += 1;


                    let typed_var = Node::new(NodeType::TypedVar(s.to_string(), name.to_string()));

                    match t3 {
                        Token::Assign => {
                            let mut ass_node = Node::new(NodeType::Assign);
                            ass_node.children.push(typed_var);
                            let (right_node, i) = expression(tokens, i);
                            ass_node.children.push(right_node);
                            dprint(format!("Parse: returning statement at token {}", i));
                            return (ass_node, i)
                        }

                        _ => panic!("Unexpected token in statement. Expected: =. Got: {}", t3)
                    }
                }

                Token::Assign => {
                    i += 1;
                    let mut ass_node = Node::new(NodeType::Assign);

                    let var = Node::new(NodeType::Name(s.to_string()));
                    let (right_node, i) = expression(tokens, i);

                    ass_node.children.push(var);
                    ass_node.children.push(right_node);

                    return (ass_node, i)
                }

                Token::Paren1 => {
                    // Function call.
                    // These are also handled in term. Maybe we can just pass this along?
                    let (args_node, new_pos) = arglist(tokens, i);
                    i = new_pos;
                    let mut funcall_node = Node::new(NodeType::FunCall(s.to_string()));
                    funcall_node.nodetype = NodeType::FunCall(s.to_string());
                    funcall_node.children.push(args_node);
                    return (funcall_node, i)
                }

                Token::Access => {

                    i += 1;
                    let t3 = &tokens[i];

                    match t3 {

                        Token::Name(acc_name) => {


                            i += 1;
                            let t4 = &tokens[i];

                            return match t4 {
                                Token::Paren1 => {

                                    // method call
                                    let (args, new_pos) = arglist(tokens, i);
                                    i = new_pos;
                                    let mut methcall_node = Node::new(NodeType::MethodCall(s.to_string(), acc_name.to_string()));
                                    methcall_node.children.push(args);

                                    if tokens[i] != Token::EndSt {
                                        panic!("Unexpected token at pos {}: {}", i, tokens[i]);
                                    }
                                    i += 1;

                                    (methcall_node, i)
                                }

                                _ => {
                                    let mut acc_node = Node::new(NodeType::Access);
                                    let obj_node = Node::new(NodeType::Name(s.to_string()));
                                    let member_node = Node::new(NodeType::Name(acc_name.to_string()));
                                    acc_node.children.push(obj_node);
                                    acc_node.children.push(member_node);

                                    (acc_node, i)
                                }
                            }
                        }

                        _ => {
                            panic!("Unexpected token following '.': {}", t3)
                        }
                    }
                }
                _ => return expression(tokens, pos)
            }
        }

        Token::If => {
            dprint("Parse: if");

            let mut condnode = Node::new(NodeType::Conditional);

            let (condpart, next_pos) = conditional(tokens, i);
            condnode.children.push(condpart);
            i = next_pos;


            loop {

                let next_token = &tokens[i];

                match next_token {

                    Token::Else => {
                        dprint("Parse: if-else");

                        let (lastcond, last_pos) = conditional(tokens, i);
                        condnode.children.push(lastcond);
                        i = last_pos;
                    }

                    _ => {
                        break;
                    }
                }
            }
            return (condnode, i)
        }

        Token::Return => {
            let (val, new_pos) = expression(tokens, i + 1);
            let mut ret = Node::new(NodeType::Return);
            ret.children.push(val);
            return (ret, new_pos);
        }

        _ => {
            return expression(tokens, pos);
        }
    }
}


fn conditional(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    match &tokens[i] {

        Token::If => {

            i += 1;

            match tokens[i] {
                Token::Paren1 => {
                    i += 1;
                    let (boolnode, new_pos) = expression(tokens, i);

                    match tokens[new_pos] {
                        Token::Paren2 => {
                            i = new_pos + 1;

                            match tokens[i] {
                                Token::Block1 => {
                                    i += 1;
                                    let (bodynode, new_pos) = block(tokens, i);

                                    i = new_pos;

                                    let mut ifnode = Node::new(NodeType::If);
                                    ifnode.children.push(boolnode);
                                    ifnode.children.push(bodynode);
                                    return (ifnode, i)
                                }
                                _ => panic!("Expected body of conditional")
                            }
                        }
                        _ => panic!("Expected closing paren after conditional expression")
                    }
                }
                _ => panic!("Unexpected token after 'if'")
            }
        }
        Token::Else => {

            i += 1;

            match &tokens[i] {

                Token::If => {

                    i += 1;

                    match tokens[i] {
                        Token::Paren1 => {
                            i += 1;
                            let (boolnode, new_pos) = expression(tokens, i);

                            match tokens[new_pos] {
                                Token::Paren2 => {
                                    i = new_pos + 1;

                                    match tokens[i] {
                                        Token::Block1 => {
                                            i += 1;
                                            let (bodynode, new_pos) = block(tokens, i);
                                            i = new_pos;

                                            let mut elseifnode = Node::new(NodeType::ElseIf);
                                            elseifnode.children.push(boolnode);
                                            elseifnode.children.push(bodynode);

                                            return (elseifnode, i)
                                        }
                                        _ => panic!("Expected body of conditional")
                                    }
                                }
                                _ => panic!("Expected closing paren after conditional expression")
                            }
                        }
                        _ => panic!("Unexpected token after 'if'")
                    }
                }

                Token::Block1 => {

                    i += 1;

                    let (bodynode, new_pos) = block(tokens, i);
                    i = new_pos;

                    let mut elsenode = Node::new(NodeType::Else);
                    elsenode.children.push(bodynode);

                    return (elsenode, i)
                }

                x => {
                    panic!("Unexpeced token afer 'else': {}", x)
                }
            }
        }

        _ => {
            panic!("Expected conditional")
        }
    }
}


fn expression(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: expression: {}", &tokens[pos]));

    // disjunction(tokens, pos)
    equality(tokens, pos)
}


fn equality(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    let (left, next_pos) = disjunction(tokens, pos);
    let t: &Token = &tokens[next_pos];

    return match t {
        Token::Equal => {

            let (right, i) = disjunction(tokens, next_pos + 1);

            let mut eqnode = Node::new(NodeType::Equal);
            eqnode.children.push(left);
            eqnode.children.push(right);

            (eqnode, i)
        }

        _ => (left, next_pos)
    }
}


fn disjunction(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let (left, next_pos) = conjunction(tokens, pos);
    let t: &Token = &tokens[next_pos];

    return match t {
        Token::LogOr => {

            let (right, i) = disjunction(tokens, next_pos + 1);

            let mut disnode = Node::new(NodeType::LogOr);
            disnode.children.push(left);
            disnode.children.push(right);

            (disnode, i)
        }

        _ => (left, next_pos)
    }
}


fn conjunction(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    // let (left, next_pos) = sum(tokens, pos);
    let (left, next_pos) = comparison(tokens, pos);
    let t: &Token = &tokens[next_pos];

    return match t {
        Token::LogAnd => {

            let (right, i) = conjunction(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::LogAnd);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }

        _ => (left, next_pos)
    }
}


fn comparison(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    let (left, next_pos) = sum(tokens, pos);
    let t: &Token = &tokens[next_pos];

    return match t {
        Token::LessThan => {

            let (right, i) = sum(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::LessThan);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }
        Token::GreaterThan => {

            let (right, i) = sum(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::GreaterThan);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }
        Token::LessOrEq => {

            let (right, i) = sum(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::LessOrEq);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }
        Token::GreaterOrEq => {

            let (right, i) = sum(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::GreaterOrEq);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }

        _ => (left, next_pos)
    }
}


fn sum(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: sum: {}", &tokens[pos]));

    let (left, next_pos) = product(tokens, pos);
    let c: &Token = tokens.get(next_pos).unwrap();

    return match c {
        Token::Add => {
            let mut sumnode = Node::new(NodeType::Add);
            sumnode.children.push(left);

            let (right, i) = sum(tokens, next_pos + 1);
            sumnode.children.push(right);

            dprint(format!("Parse: assembled sum: {}", &sumnode));
            (sumnode, i)
        },

        Token::Sub => {
            let mut sumnode = Node::new(NodeType::Sub);
            sumnode.children.push(left);

            let (right, i) = sum(tokens, next_pos + 1);
            sumnode.children.push(right);

            dprint(format!("Parse: assembled sum: {}", &sumnode));
            (sumnode, i)
        }

        _ => (left, next_pos)
    }
}


fn product(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: product: {}", &tokens[pos]));

    let (left, mut i) = term(tokens, pos);
    let t: &Token = tokens.get(i).unwrap();

    return match t {
        Token::Mul => {
            let mut prod = Node::new(NodeType::Mul);
            prod.children.push(left);

            i += 1;

            let (right, i) = product(tokens, i);
            prod.children.push(right);
            (prod, i)
        }
        Token::Div => {
            let mut div = Node::new(NodeType::Div);
            div.children.push(left);

            i += 1;

            let (right, i) = product(tokens, i);
            div.children.push(right);
            (div, i)
        }

        _ => {
            (left, i)
        }
    }
}


fn term(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: term: {}", &tokens[pos]));

    let t: &Token = tokens.get(pos).expect("No token for term!");

    match t {

        &Token::Int(ref s) => {
            let node = Node::new(NodeType::Int(s.clone()));
            return (node, pos+1)
        }

        &Token::Double(ref s) => {
            let node = Node::new(NodeType::Double(s.clone()));
            return (node, pos+1)
        }

        &Token::Add => {
            // As Dart.
            darterror("Error: '+' is not a prefix operator.");
        }

        &Token::Sub => {
            // This handles unary minus.
            let mut unary = Node::new(NodeType::Sub);
            let (next, new_pos) = term(tokens, pos+1);
            unary.children.push(next);
            return (unary, new_pos)
        }

        &Token::Not => {
            let mut notnode = Node::new(NodeType::Not);
            let (next, new_pos) = term(tokens, pos+1);
            notnode.children.push(next);
            return (notnode, new_pos)
        }

        &Token::Str(ref s) => {
            let node = Node::new(NodeType::Str(s.clone()));
            return (node, pos+1)
        }

        &Token::Bool(v) => {
            let node = Node::new(NodeType::Bool(v));
            return (node, pos+1)
        }

        &Token::Name(ref s) => {

            // Postfixed inc/dec should be bound tightly, so handle
            // it here rather than in expression.
            if let Token::Increment = tokens[pos+1] {
                let mut incnode = Node::new(NodeType::PostIncrement);
                let node = Node::new(NodeType::Name(s.clone()));
                incnode.children.push(node);
                return (incnode, pos + 2);
            }
            if let Token::Decrement = tokens[pos+1] {
                let mut decnode = Node::new(NodeType::PostDecrement);
                let node = Node::new(NodeType::Name(s.clone()));
                decnode.children.push(node);
                return (decnode, pos + 2);
            }


            if let Token::Paren1 = tokens[pos+1] {
                // Function call.
                let (args_node, new_pos) = arglist(tokens, pos + 1);
                let mut funcall_node = Node::new(NodeType::FunCall(s.to_string()));
                funcall_node.nodetype = NodeType::FunCall(s.to_string());
                funcall_node.children.push(args_node);
                return (funcall_node, new_pos)
            }



            let node = Node::new(NodeType::Name(s.clone()));
            return (node, pos+1)
        }

        &Token::Increment => {

            let next = &tokens[pos+1];
            return match next {
                Token::Name(s) => {
                    let namenode = Node::new(NodeType::Name(s.clone()));
                    let mut incnode = Node::new(NodeType::PreIncrement);
                    incnode.children.push(namenode);
                    (incnode, pos + 2)
                }
                _ => panic!("Invalid operand for increment: {}", next)
            }
        }

        &Token::Decrement => {

            let next = &tokens[pos+1];
            return match next {
                Token::Name(s) => {
                    let namenode = Node::new(NodeType::Name(s.clone()));
                    let mut incnode = Node::new(NodeType::PreDecrement);
                    incnode.children.push(namenode);
                    (incnode, pos + 2)
                }
                _ => panic!("Invalid operand for decrement: {}", next)
            }
        }

        &Token::Paren1 => {
            let (wnode, new_pos) = expression(tokens, pos+1);
            if let &Token::Paren2 = &tokens[new_pos] {
                return (wnode, new_pos + 1)
            }
            else {
                panic!("Expected closing parenthesis at {} but found {}", new_pos, tokens[new_pos])
            }
        }

        &Token::Brack1 => {

            let mut i = pos + 1;
            let mut list_node = Node::new(NodeType::List);
            let mut expect_sep = false;

            if tokens[i] == Token::Brack2 {
                return (list_node, i + 1)
            }

            while i < tokens.len() {

                if expect_sep {
                    match &tokens[i] {

                        Token::Comma => {
                            if !expect_sep {
                                panic!("Expected an identifier, but got ','");
                            }
                            i += 1;
                            expect_sep = false;
                            continue;
                        }

                        Token::Brack2 => {
                            i += 1;
                            break;
                        }
                        _ => panic!("Unexpected token when parsing list: {}", &tokens[i])
                    }
                }
                expect_sep = true;
                let (entry, new_pos) = expression(tokens, i);
                list_node.children.push(entry);
                i = new_pos;
            }

            return (list_node, i)
        }

        _ => {
            panic!("Unexpected token {}, expected paren or number.", {t})
        }
    }
}

