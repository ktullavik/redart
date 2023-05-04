/// statement  -> expression
/// expression -> sum
/// sum        -> product + sum | product
/// product    -> num * product | num

use std::fmt;
use utils::dprint;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Increment,
    Decrement,
    // Logical
    LogOr,
    LogAnd,
    BinOr,
    BinAnd,
    // Relational
    LessThan,
    GreaterThan,
    LessOrEq,
    GreaterOrEq,
    Access,
    Comma,
    Assign,
    // Keywords
    If,
    Else,
    Int(String),
    Double(String),
    Str(String),
    Bool(bool),
    Name(String),
    Paren1,
    Paren2,
    Block1,
    Block2,
    Brack1,
    Brack2,
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
    LogOr,
    LogAnd,
    BinOr,
    BinAnd,
    LessThan,
    GreaterThan,
    LessOrEq,
    GreaterOrEq,
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
    Directive,
}


impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Assign => write!(f, "="),
            // Arithmetic
            Token::Add => write!(f, "+"),
            Token::Sub => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            // Logical
            Token::LogOr => write!(f, "||"),
            Token::LogAnd => write!(f, "&&"),
            Token::BinOr => write!(f, "|"),
            Token::BinAnd => write!(f, "&"),
            // Relational
            Token::LessThan    => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessOrEq    => write!(f, "<="),
            Token::GreaterOrEq => write!(f, ">="),
            Token::Access => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Int(s)     => write!(f, "{}", s),
            Token::Double(s)     => write!(f, "{}", s),
            Token::Str(s)  => write!(f, "\"{}\"", s),
            Token::Bool(v)     => write!(f, "{}", v),
            Token::Name(s)    => write!(f, "{}", s),
            Token::Paren1 => write!(f, "("),
            Token::Paren2 => write!(f, ")"),
            Token::Block1 => write!(f, "{{"),
            Token::Block2 => write!(f, "}}"),
            Token::Brack1 => write!(f, "["),
            Token::Brack2 => write!(f, "]"),
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
            NodeType::LogOr => write!(f, "||"),
            NodeType::LogAnd => write!(f, "&&"),
            NodeType::BinOr => write!(f, "|"),
            NodeType::BinAnd => write!(f, "&"),
            NodeType::LessThan => write!(f, "<"),
            NodeType::GreaterThan => write!(f, ">"),
            NodeType::LessOrEq => write!(f, "<="),
            NodeType::GreaterOrEq => write!(f, ">="),
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
            NodeType::ParamList => write!(f, "PARAMLIST"),
            NodeType::ArgList => write!(f, "ARGLIST"),
            NodeType::Conditional => write!(f, "CONDITIONAL"),
            NodeType::If => write!(f, "IF"),
            NodeType::ElseIf => write!(f, "ELSEIF"),
            NodeType::Else => write!(f, "ELSE"),
            NodeType::Block => write!(f, "BLOCK"),
            NodeType::Return => write!(f, "RETURN"),
            NodeType::Module => write!(f, "MODULE"),
            NodeType::Directive => write!(f, "DIRECTIVE"),
            NodeType::Directives => write!(f, "DIRECTIVES"),
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
        let (funnode, readindex) = fundef(tokens, i);
        root.children.push(funnode);

        dprint(format!("Parse: read len: {}", readindex));
        i = readindex;
    }

    if i < tokens.len() - 1 {
        return Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    }
    else if i > tokens.len() - 1 {
        return Err(format!("Index returned beyond end of token array. Index: {}, len: {}", i, tokens.len()))
    }

    dprint(String::from(format!("Parse: finished at index: {}", i)));
    Ok(root)
}


fn directives(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;
    let directives_node = Node::new(NodeType::Directives);

    while i < tokens.len() {

        match &tokens[i] {
            Token::Import => {

                let mut node = Node::new(NodeType::Directive);
                let mut j = i + 1;

                while j < tokens.len() {

                    match &tokens[j] {
                        Token::EndSt => {
                            i = j+1;
                            break;
                        }
                        Token::End => {
                            i = j;
                            break;
                        }
                        Token::Name(s2) => {
                            let child_node = Node::new(NodeType::Name(s2.to_string()));
                            node.children.push(child_node);
                            j += 1;
                        }
                        _ => {
                            panic!("Unknown token in directive: {}", &tokens[j])
                        }
                    }
                }
                continue;
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
        Token::Name(_) => {

            let t2: &Token = tokens.get(i).unwrap();
            i += 1;

            match t2 {

                Token::Name(fname) => {
                    let mut node = Node::new(NodeType::FunDef(fname.to_string()));
                    let (params, new_pos) = paramlist(tokens, i);
                    i = new_pos;
                    node.children.push(params);

                    let t3: &Token = tokens.get(i).unwrap();
                    i += 1;

                    match t3 {
                        Token::Block1 => {
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

        _ => {
            panic!("Expected function definition.")
        }
    }
}


fn paramlist(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    while i < tokens.len() {

        let t = &tokens[i];
        i += 1;

        match t {

            Token::Paren1 => {

                let mut node = Node::new(NodeType::ParamList);
                let mut expect_comma = false;
                let mut j: usize = i;

                while j < tokens.len() {

                    let p = &tokens[j];

                    match p {

                        Token::Name(s) => {
                            let paramnode= Node::new(NodeType::Name(s.to_string()));
                            node.children.push(paramnode);
                            expect_comma = true;
                            j += 1;
                        }

                        Token::Comma => {
                            if !expect_comma {
                                panic!("Unexpected token when reading parameter list: ,");
                            }
                            j += 1;
                            expect_comma = false;
                            continue;
                        }

                        Token::Paren2 => {
                            j += 1;
                            break;
                        }

                        _ => {
                            panic!("Unexpected token when reading parameters: {}", p)
                        }
                    }
                }

                i = j;
                return (node, i);
            }
            _ => {
                panic!("Expected (, starting paramlist. Got: {}", t)
            }
        }
    }
    panic!("Error when reading param list.")
}


fn arglist(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    while i < tokens.len() {

        let t = &tokens[i];

        match t {

            Token::Paren1 => {

                let mut node = Node::new(NodeType::ArgList);
                let mut expect_comma = false;
                let mut j: usize = i + 1;

                while j < tokens.len() {

                    match &tokens[j] {

                        Token::Comma => {
                            if !expect_comma {
                                panic!("Unexpected token when reading arg list: ,");
                            }
                            j += 1;
                            expect_comma = false;
                            continue;
                        }

                        Token::Name(_)   |
                        Token::Str(_) |
                        Token::Int(_)    |
                        Token::Double(_)
                        => {
                            let (arg, new_pos) = expression(tokens, j);

                            node.children.push(arg);
                            j = new_pos;
                            expect_comma = true;
                            continue;
                        }

                        Token::Add |
                        Token::Sub |
                        Token::Mul |
                        Token::Brack1
                        => {
                            let (arg, new_pos) = expression(tokens, j);

                            node.children.push(arg);
                            j = new_pos;
                            expect_comma = true;
                            continue;
                        }

                        Token::Paren2 => {
                            j += 1;
                            break;
                        }

                        x => panic!("Unexpected token in argument list: {}", x)
                    }
                }

                i = j;

                return (node, i);
            }
            _ => {
                panic!("Expected (, starting arglist. Got: {}", t)
            }
        }
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

    disjunction(tokens, pos)
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
            panic!("Unexpected token {:?}, expected paren or number.", {t})
        }
    }
}

