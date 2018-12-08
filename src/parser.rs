/// statement  -> expression
/// expression -> sum
/// sum        -> product + sum | product
/// product    -> num * product | num

use std::fmt;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ADD,
    SUB,
    MUL,
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
    NEWLINE,
    ENDST,
    END
}


pub enum NodeType {
    ADD,
    SUB,
    MUL,
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
            Token::NEWLINE    => write!(f, "NEWLINE"),
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
            NodeType::ACCESS                        => write!(f, "."),
            NodeType::KEYWORD(s)                    => write!(f, "{}", s),
            NodeType::NUM(s)                        => write!(f, "{}", s),
            NodeType::STRING(s)                     => write!(f, "\"{}\"", s),
            NodeType::NAME(s)                       => write!(f, "{}", s),
            NodeType::TYPEDVAR(tp, name)            => write!(f, "{}:{}", name, tp),
            NodeType::FUNDEF(s)                     => write!(f, "{}() {{}}", s),
            NodeType::FUNCALL(s)                    => write!(f, "{}()", s),
            NodeType::METHODCALL(objname, methname) => write!(f, "{}.{}()", objname, methname),
            NodeType::PAREN1                        => write!(f, "("),
            NodeType::PAREN2                        => write!(f, ")"),
            NodeType::LIST                          => write!(f, "[]"),
            NodeType::PARAMLIST                     => write!(f, "PARAMLIST"),
            NodeType::ARGLIST                       => write!(f, "PARAMLIST"),
            NodeType::BLOCK                         => write!(f, "BLOCK"),
            NodeType::MODULE                        => write!(f, "MODULE"),
            NodeType::DIRECTIVE                     => write!(f, "DIRECTIVE"),
            NodeType::DIRECTIVES                    => write!(f, "DIRECTIVES"),
        }
    }
}


pub struct Node {
    pub nodetype: NodeType,
    pub children: Vec<Node>
}


impl Node {

    pub fn new(nodetype: NodeType) -> Node {
        Node {
            children: Vec::new(),
            nodetype
        }
    }


    pub fn print(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {

        match self.children.len() {

            0 => writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype),

            1 => {
                writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                self.children[0].print(f,depth + 1)
            },

            2 => {
                writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                self.children[0].print(f,depth + 1);
                self.children[1].print(f,depth + 1)
            }

            x => {
                writeln!(f, "{1:0$} {2}", depth * 2, "", self.nodetype);
                for i in 0 .. x {
                    self.children[i].print(f, depth + 1);
                }
                write!(f, "")
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

    let mut root = Node::new(NodeType::MODULE);
    let directive_node = directives(tokens, 0)?;
    let mut i = directive_node.1;
    root.children.push(directive_node.0);

    let (funnode, i) = fundef(tokens, i).unwrap();
    &root.children.push(funnode);

    if i != tokens.len() - 1 {
        return Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    }
    Ok(root)
}


fn directives(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    let mut i = pos;
    let mut directives_node = Node::new(NodeType::DIRECTIVES);

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
                                let mut child_node = Node::new(NodeType::NAME(s2.to_string()));
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
        Token::NAME(s) => {

            let t2: &Token = tokens.get(i).unwrap();
            i += 1;

            match t2 {

                Token::NAME(fname) => {
                    let mut node = Node::new(NodeType::FUNDEF(fname.to_string()));
                    let (params, new_pos) = paramlist(tokens, i).unwrap();
                    i = new_pos;
                    let t3: &Token = tokens.get(i).unwrap();
                    i += 1;

                    match t3 {
                        Token::BLOCK1 => {
                            let (body, new_pos) = block(tokens, i).unwrap();
                            node.children.push(body);
                            println!("pos after block: {}", new_pos);
                            i = new_pos;
                            Ok((node, i))
                        }

                        _ => {
                            panic!(format!("Expected {{. Got: {}", t3))
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
                            let mut paramnode= Node::new(NodeType::NAME(s.to_string()));
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
                            panic!(format!("Unexpected token when reading parameters: {}", p))
                        }
                    }
                }

                i = j;
                return Ok((node, i));
            }
            _ => {
                panic!(format!("Expected (, starting paramlist. Got: {}", t))
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

                        Token::NAME(s)   |
                        Token::STRING(s) |
                        Token::NUM(s)
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
                panic!(format!("Expected (, starting arglist. Got: {}", t))
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
        println!("BLOCK LOOP at: {}, token: {}", i, tokens[i]);

        if tokens[i] == Token::BLOCK2 {
            println!("token is end-block, breaking.");
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
            Token::NAME(s) => {
                continue;
            }
            _ => return Err(format!("Unexpected token at pos {} when parsing block: {}", i, &tokens[i]))
        }

        i += 1;
    }

    Ok((node, i))
}


fn statement(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    println!("statement: {}", tokens[pos]);

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


                    let mut typed_var = Node::new(NodeType::TYPEDVAR(s.to_string(), name.to_string()));

                    match t3 {
                        Token::ASSIGN => {
                            let mut ass_node = Node::new(NodeType::ASSIGN);
                            ass_node.children.push(typed_var);
                            let (right_node, i) = expression(tokens, i)?;
                            ass_node.children.push(right_node);
                            println!("returning statement at token {}", i);
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
                                    let mut obj_node = Node::new(NodeType::NAME(s.to_string()));
                                    let mut member_node = Node::new(NodeType::NAME(acc_name.to_string()));
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

    println!("returning statement at token {}", i);
    Ok((node, i))
}


fn expression(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    println!("expression: {}", tokens[pos]);

    let mut next_pos: usize;

    let (left, i) = sum(tokens, pos)?;
    next_pos = i;

    let c: &Token = tokens.get(next_pos).unwrap();
    match c {
        Token::ASSIGN => {

            match &left.nodetype {

                NodeType::NAME(name) => {

                    let mut assigment = Node::new(NodeType::ASSIGN);
                    let mut name_node = Node::new(NodeType::NAME(name.to_string()));

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
            println!("returning expression at token {}", next_pos);
            return Ok((left, next_pos));
        }
    }
}


fn sum(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    println!("sum: {}", tokens[pos]);

    let (left, next_pos) = product(tokens, pos)?;
    let c: &Token = tokens.get(next_pos).unwrap();

    match c {
        Token::ADD => {
            let mut sum = Node::new(NodeType::ADD);
            sum.children.push(left);

            let (right, i) = expression(tokens, next_pos + 1)?;
            sum.children.push(right);

            println!("sum assembled sum: {}", sum);
            Ok((sum, i))
        },

        Token::SUB => {
            let mut sum = Node::new(NodeType::SUB);
            sum.children.push(left);

            let (right, i) = expression(tokens, next_pos + 1)?;
            sum.children.push(right);

            println!("sum assembled sum: {}", sum);
            Ok((sum, i))
        }

        _ => Ok((left, next_pos))
    }
}


fn product(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    println!("product: {}", tokens[pos]);

    let (left, mut i) = term(tokens, pos)?;
    let c: &Token = tokens.get(i).unwrap();

    match c {
        Token::MUL => {
            let mut prod = Node::new(NodeType::MUL);
            prod.children.push(left);

            i += 1;

            let (right, i) = product(tokens, i)?;
            prod.children.push(right);
            Ok((prod, i))
        }

        _ => {
            Ok((left, i))
        }
    }
}


fn term(tokens: &Vec<Token>, pos: usize) -> Result<(Node, usize), String> {

    println!("term: {}", tokens[pos]);

    let t: &Token = tokens.get(pos).expect("shit");

    match t {
        &Token::NUM(ref t) => {
            let mut node = Node::new(NodeType::NUM(t.to_string()));
            Ok((node, pos+1))
        }

        &Token::STRING(ref s) => {
            let mut node = Node::new(NodeType::STRING(t.to_string()));
            Ok((node, pos+1))
        }

        &Token::NAME(ref c) => {
            let mut node = Node::new(NodeType::NAME(t.to_string()));
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

