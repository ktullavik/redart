/// statement  -> expression
/// expression -> sum
/// sum        -> product + sum | product
/// product    -> num * product | num

use std::fmt;
use utils;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ADD,
    SUB,
    MUL,
    DIV,
    ACCESS,
    COMMA,
    ASSIGN,
    KEYWORD(String),
    NUM(String),
    STRING(String),
    NAME(String),
    PAREN1,
    PAREN2,
    BLOCK1,
    BLOCK2,
    BRACK1,
    BRACK2,
    ENDST,
    END
}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum NodeType {
    ADD,
    SUB,
    MUL,
    DIV,
    ACCESS,
    ASSIGN,
    KEYWORD(String),
    NUM(String),
    STRING(String),
    NAME(String),
    TYPEDVAR(String, String),
//    PAREN1,
//    PAREN2,
    BLOCK,
    LIST,
    MODULE,
    FUNDEF(String),
    FUNCALL(String),
    METHODCALL(String, String),
    PARAMLIST,
    ARGLIST,
    DIRECTIVES,
    DIRECTIVE,
}


impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::ASSIGN     => write!(f, "="),
            Token::ADD        => write!(f, "+"),
            Token::SUB        => write!(f, "-"),
            Token::MUL        => write!(f, "*"),
            Token::DIV        => write!(f, "/"),
            Token::ACCESS     => write!(f, "."),
            Token::COMMA      => write!(f, ","),
            Token::KEYWORD(s) => write!(f, "{}", s),
            Token::NUM(s)     => write!(f, "{}", s),
            Token::STRING(s)  => write!(f, "\"{}\"", s),
            Token::NAME(s)    => write!(f, "{}", s),
            Token::PAREN1     => write!(f, "("),
            Token::PAREN2     => write!(f, ")"),
            Token::BLOCK1     => write!(f, "{{"),
            Token::BLOCK2     => write!(f, "}}"),
            Token::BRACK1     => write!(f, "["),
            Token::BRACK2     => write!(f, "]"),
            Token::ENDST      => write!(f, "ENDST"),
            Token::END        => write!(f, "END"),
        }
    }
}


impl fmt::Display for NodeType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::ASSIGN                        => write!(f, "="),
            NodeType::ADD                           => write!(f, "+"),
            NodeType::SUB                           => write!(f, "-"),
            NodeType::MUL                           => write!(f, "*"),
            NodeType::DIV                           => write!(f, "/"),
            NodeType::ACCESS                        => write!(f, "."),
            NodeType::KEYWORD(s)                    => write!(f, "{}", s),
            NodeType::NUM(s)                        => write!(f, "{}", s),
            NodeType::STRING(s)                     => write!(f, "\"{}\"", s),
            NodeType::NAME(s)                       => write!(f, "{}", s),
            NodeType::TYPEDVAR(tp, name)            => write!(f, "{}:{}", name, tp),
            NodeType::FUNDEF(s)                     => write!(f, "{}() {{}}", s),
            NodeType::FUNCALL(s)                    => write!(f, "{}()", s),
            NodeType::METHODCALL(objname, methname) => write!(f, "{}.{}()", objname, methname),
//            NodeType::PAREN1                        => write!(f, "("),
//            NodeType::PAREN2                        => write!(f, ")"),
            NodeType::LIST                          => write!(f, "[]"),
            NodeType::PARAMLIST                     => write!(f, "PARAMLIST"),
            NodeType::ARGLIST                       => write!(f, "ARGLIST"),
            NodeType::BLOCK                         => write!(f, "BLOCK"),
            NodeType::MODULE                        => write!(f, "MODULE"),
            NodeType::DIRECTIVE                     => write!(f, "DIRECTIVE"),
            NodeType::DIRECTIVES                    => write!(f, "DIRECTIVES"),
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
    utils::dprint(" ");
    utils::dprint("PARSE");
    utils::dprint(" ");

    let mut root = Node::new(NodeType::MODULE);
    let directive_node = directives(tokens, 0)?;
    let mut i = directive_node.1;
    root.children.push(directive_node.0);


    while i < tokens.len() - 1 {
        let (funnode, readindex) = fundef(tokens, i).unwrap();
        root.children.push(funnode);

        utils::dprint(format!("Parse: read len: {}", readindex));
        i = readindex;
    }

    if i < tokens.len() - 1 {
        return Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    }
    else if i > tokens.len() - 1 {
        return Err(format!("Index returned beyond end of token array. Index: {}, len: {}", i, tokens.len()))
    }

    utils::dprint(String::from((format!("Parse: finished at index: {}", i))));
    Ok(root)
}


fn directives(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    let mut i = pos;
    let directives_node = Node::new(NodeType::DIRECTIVES);

    while i < tokens.len() {

        match &tokens[i] {
            Token::KEYWORD(s) => match s.as_str() {
                "import" => {

                    let mut node = Node::new(NodeType::DIRECTIVE);
                    let mut j = i + 1;

                    while j < tokens.len() {

                        match &tokens[j] {
                            Token::ENDST => {
                                i = j+1;
                                break;
                            }
                            Token::END => {
                                i = j;
                                break;
                            }
                            Token::NAME(s2) => {
                                let child_node = Node::new(NodeType::NAME(s2.to_string()));
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
                _ => break
            }
            _  => break
        }
    }

    Ok((directives_node, i))
}


fn fundef(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String>  {

    let mut i: usize = pos;
    let t: &Token = tokens.get(i).unwrap();
    i += 1;

    match t {
        Token::NAME(_) => {

            let t2: &Token = tokens.get(i).unwrap();
            i += 1;

            match t2 {

                Token::NAME(fname) => {
                    let mut node = Node::new(NodeType::FUNDEF(fname.to_string()));
                    let (params, new_pos) = paramlist(tokens, i).unwrap();
                    i = new_pos;
                    node.children.push(params);

                    let t3: &Token = tokens.get(i).unwrap();
                    i += 1;

                    match t3 {
                        Token::BLOCK1 => {
                            let (body, new_pos) = block(tokens, i).unwrap();
                            node.children.push(body);
                            i = new_pos;
                            utils::dprint(format!("fundef parsed to {}", new_pos));
                            Ok((node, i))
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


fn paramlist(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    let mut i = pos;

    while i < tokens.len() {

        let t = &tokens[i];
        i += 1;

        match t {

            Token::PAREN1 => {

                let mut node = Node::new(NodeType::PARAMLIST);
                let mut expect_comma = false;
                let mut j: usize = i;

                while j < tokens.len() {

                    let p = &tokens[j];

                    match p {

                        Token::NAME(s) => {
                            let paramnode= Node::new(NodeType::NAME(s.to_string()));
                            node.children.push(paramnode);
                            expect_comma = true;
                            j += 1;
                        }

                        Token::COMMA => {
                            if !expect_comma {
                                panic!("Unexpected token when reading parameter list: ,");
                            }
                            j += 1;
                            expect_comma = false;
                            continue;
                        }

                        Token::PAREN2 => {
                            j += 1;
                            break;
                        }

                        _ => {
                            panic!("Unexpected token when reading parameters: {}", p)
                        }
                    }
                }

                i = j;
                return Ok((node, i));
            }
            _ => {
                panic!("Expected (, starting paramlist. Got: {}", t)
            }
        }
    }
    Err(String::from("Error when reading param list."))
}


fn arglist(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    let mut i = pos;

    while i < tokens.len() {

        let t = &tokens[i];

        match t {

            Token::PAREN1 => {

                let mut node = Node::new(NodeType::ARGLIST);
                let mut expect_comma = false;
                let mut j: usize = i + 1;

                while j < tokens.len() {

                    match &tokens[j] {

                        Token::COMMA => {
                            if !expect_comma {
                                panic!("Unexpected token when reading arg list: ,");
                            }
                            j += 1;
                            expect_comma = false;
                            continue;
                        }

                        Token::NAME(_)   |
                        Token::STRING(_) |
                        Token::NUM(_)
                        => {
                            let (arg, new_pos) = expression(tokens, j)?;

                            node.children.push(arg);
                            j = new_pos;
                            expect_comma = true;
                            continue;
                        }

                        Token::ADD       |
                        Token::SUB       |
                        Token::MUL       |
                        Token::BRACK1
                        => {
                            let (arg, new_pos) = expression(tokens, j)?;

                            node.children.push(arg);
                            j = new_pos;
                            expect_comma = true;
                            continue;
                        }

                        Token::PAREN2 => {
                            j += 1;
                            break;
                        }

                        x => return Err(format!("Unexpected token in argument list: {}", x))
                    }
                }

                i = j;

                return Ok((node, i));
            }
            _ => {
                panic!("Expected (, starting arglist. Got: {}", t)
            }
        }
    }
    Err(String::from("Error when reading arg list."))
}


/// Parse a series of statements.
///
/// Expects first token after block started by {.
/// Consumes the end-block token }.
fn block(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    let mut node = Node::new(NodeType::BLOCK);
    let mut i = pos;

    while i < tokens.len() {
        utils::dprint(format!("Parse: block loop at: {}, token: {}", i, &tokens[i]));

        if tokens[i] == Token::BLOCK2 {
            utils::dprint(String::from("token is end-block, breaking."));
            i += 1;
            break;
        }

        let (snode, new_pos) = statement(tokens, i)?;
        node.children.push(snode);

        i = new_pos;

        match &tokens[i] {

            Token::ENDST => {
                // ENDST should be consumed by statement?
                i += 1;
                continue;
            }
            Token::NAME(_) => {
                continue;
            }
            _ => return Err(format!("Unexpected token at pos {} when parsing block: {}", i, &tokens[i]))
        }
    }

    Ok((node, i))
}


fn statement(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    utils::dprint(format!("statement: {}", &tokens[pos]));

    // Can be
    // assignment: var i = 2
    //
    let (node, new_pos) = expression(tokens, pos)?;
    let mut i = new_pos;

    match &node.nodetype {

        NodeType::NAME(s) => {

            let t2 = &tokens[i];

            match t2 {

                Token::NAME(name) => {
                    // Two names in a row indicate a typed variable or function definition.
                    i += 1;
                    let t3 = &tokens[i];
                    i += 1;


                    let typed_var = Node::new(NodeType::TYPEDVAR(s.to_string(), name.to_string()));

                    match t3 {
                        Token::ASSIGN => {
                            let mut ass_node = Node::new(NodeType::ASSIGN);
                            ass_node.children.push(typed_var);
                            let (right_node, i) = expression(tokens, i)?;
                            ass_node.children.push(right_node);
                            utils::dprint(format!("returning statement at token {}", i));
                            return Ok((ass_node, i))
                        }

                        _ => panic!("Unexpected token in statement. Expected: =. Got: {}", t3)
                    }
                }

                Token::PAREN1 => {
                    // Function call.
                    let (args_node, new_pos) = arglist(tokens, i)?;
                    i = new_pos;
                    let mut funcall_node = Node::new(NodeType::FUNCALL(s.to_string()));
                    funcall_node.nodetype = NodeType::FUNCALL(s.to_string());
                    funcall_node.children.push(args_node);
                    return Ok((funcall_node, i))
                }

                Token::ACCESS => {

                    i += 1;
                    let t3 = &tokens[i];

                    match t3 {

                        Token::NAME(acc_name) => {


                            i += 1;
                            let t4 = &tokens[i];

                            match t4 {

                                Token::PAREN1 => {

                                    // method call
                                    let (args, new_pos) = arglist(tokens, i)?;
                                    i = new_pos;
                                    let mut methcall_node = Node::new(NodeType::METHODCALL(s.to_string(), acc_name.to_string()));
                                    methcall_node.children.push(args);

                                    if tokens[i] != Token::ENDST {
                                        return Err(format!("Unexpected token at pos {}: {}", i, tokens[i]));
                                    }
                                    i += 1;

                                    return Ok((methcall_node, i));
                                }

                                _ => {
                                    let mut acc_node = Node::new(NodeType::ACCESS);
                                    let obj_node = Node::new(NodeType::NAME(s.to_string()));
                                    let member_node = Node::new(NodeType::NAME(acc_name.to_string()));
                                    acc_node.children.push(obj_node);
                                    acc_node.children.push(member_node);
                                    return Ok((acc_node, i));
                                }
                            }
                        }

                        _ => {
                            return Err(format!("Unexpected token following '.': {}", t3));
                        }
                    }
                }
                _ => panic!("Unexpected token in statement: {}", t2)
            }
        }

        _ => return Err(String::from("Unexpected token when reading statement"))
    };
}


fn expression(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    utils::dprint(format!("Parse: expression: {}", &tokens[pos]));

    let mut next_pos: usize;

    let (left, i) = sum(tokens, pos)?;
    next_pos = i;

    let c: &Token = tokens.get(next_pos).unwrap();
    match c {
        Token::ASSIGN => {

            match &left.nodetype {

                NodeType::NAME(name) => {

                    let mut assigment = Node::new(NodeType::ASSIGN);
                    let name_node = Node::new(NodeType::NAME(name.to_string()));

                    assigment.children.push(name_node);

                    let (right, i) = expression(tokens, next_pos + 1)?;
                    next_pos = i;

                    assigment.children.push(right);

                    return Ok((assigment, next_pos));
                },
                _ => panic!("Invalid name for assignment: {}", left.nodetype)
            }
        }

        _ => {
            utils::dprint(format!("Parse: returning expression at token {}", next_pos));
            return Ok((left, next_pos));
        }
    }
}


fn sum(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    utils::dprint(format!("Parse: sum: {}", &tokens[pos]));

    let (left, next_pos) = product(tokens, pos)?;
    let c: &Token = tokens.get(next_pos).unwrap();

    match c {
        Token::ADD => {
            let mut sum = Node::new(NodeType::ADD);
            sum.children.push(left);

            let (right, i) = expression(tokens, next_pos + 1)?;
            sum.children.push(right);

            utils::dprint(format!("Parse: assembled sum: {}", &sum));
            Ok((sum, i))
        },

        Token::SUB => {
            let mut sum = Node::new(NodeType::SUB);
            sum.children.push(left);

            let (right, i) = expression(tokens, next_pos + 1)?;
            sum.children.push(right);

            utils::dprint(format!("Parse: assembled sum: {}", &sum));
            Ok((sum, i))
        }

        _ => Ok((left, next_pos))
    }
}


fn product(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    utils::dprint(format!("Parse: product: {}", &tokens[pos]));

    let (left, mut i) = term(tokens, pos)?;
    let t: &Token = tokens.get(i).unwrap();

    match t {
        Token::MUL => {
            let mut prod = Node::new(NodeType::MUL);
            prod.children.push(left);

            i += 1;

            let (right, i) = product(tokens, i)?;
            prod.children.push(right);
            Ok((prod, i))
        }
        Token::DIV => {
            let mut div = Node::new(NodeType::DIV);
            div.children.push(left);

            i += 1;

            let (right, i) = product(tokens, i)?;
            div.children.push(right);
            Ok((div, i))
        }

        _ => {
            Ok((left, i))
        }
    }
}


fn term(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    utils::dprint(format!("Parse: term: {}", &tokens[pos]));

    let t: &Token = tokens.get(pos).expect("No token for term!");

    match t {
        &Token::NUM(ref s) => {
            let node = Node::new(NodeType::NUM(s.clone()));
            Ok((node, pos+1))
        }

        &Token::STRING(ref s) => {
            let node = Node::new(NodeType::STRING(s.clone()));
            Ok((node, pos+1))
        }

        &Token::NAME(ref s) => {
            let node = Node::new(NodeType::NAME(s.clone()));
            Ok((node, pos+1))
        }

        &Token::PAREN1 => {
            expression(tokens, pos+1).and_then(|wnode: (Node, usize)| {
                if let Some(&Token::PAREN2) = tokens.get(wnode.1) {
                    Ok((wnode.0, wnode.1 + 1))
                }
                else {
                    Err(format!("Expected closing parenthesis at {} but found {:?}", wnode.1, tokens.get(wnode.1)))
                }
            })
        }

        &Token::BRACK1 => {

            let mut i = pos + 1;
            let mut list_node = Node::new(NodeType::LIST);
            let mut expect_sep = false;
            let mut closed = false;

            if tokens[i] == Token::BRACK2 {
                closed = true;
                return Ok((list_node, i + 1))
            }

            while i < tokens.len() {

                if expect_sep {
                    match &tokens[i] {

                        Token::COMMA => {
                            if !expect_sep {
                                panic!("Expected an identifier, but got ','");
                            }
                            i += 1;
                            expect_sep = false;
                            continue;
                        }

                        Token::BRACK2 => {
                            closed = true;
                            i += 1;
                            break;
                        }
                        _ => return Err(format!("Unexpected token when parsing list: {}", &tokens[i]))
                    }
                }
                expect_sep = true;
                let (entry, new_pos) = expression(tokens, i)?;
                list_node.children.push(entry);
                i = new_pos;
            }

            Ok((list_node, i))
        }

        _ => {
            Err(format!("Unexpected token {:?}, expected paren or number.", {t}))
        }
    }
}

