/// statement  -> expression
/// expression -> sum
/// sum        -> product + sum | product
/// product    -> num * product | num

use std::fmt;
use utils::dprint;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ADD,
    SUB,
    MUL,
    DIV,
    INCREMENT,
    DECREMENT,
    ACCESS,
    COMMA,
    ASSIGN,
    IF,
    ELSE,
    INT(String),
    DOUBLE(String),
    STRING(String),
    BOOL(bool),
    NAME(String),
    PAREN1,
    PAREN2,
    BLOCK1,
    BLOCK2,
    BRACK1,
    BRACK2,
    IMPORT,
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
    PREINCREMENT,
    POSTINCREMENT,
    PREDECREMENT,
    POSTDECREMENT,
    ACCESS,
    ASSIGN,
    INT(String),
    DOUBLE(String),
    STRING(String),
    BOOL(bool),
    NAME(String),
    TYPEDVAR(String, String),
    CONDITIONAL,
    IF,
    ELSEIF,
    ELSE,
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
            Token::INCREMENT  => write!(f, "++"),
            Token::DECREMENT  => write!(f, "--"),
            Token::ACCESS     => write!(f, "."),
            Token::COMMA      => write!(f, ","),
            Token::IF         => write!(f, "if"),
            Token::ELSE       => write!(f, "else"),
            Token::INT(s)     => write!(f, "{}", s),
            Token::DOUBLE(s)     => write!(f, "{}", s),
            Token::STRING(s)  => write!(f, "\"{}\"", s),
            Token::BOOL(v)     => write!(f, "{}", v),
            Token::NAME(s)    => write!(f, "{}", s),
            Token::PAREN1     => write!(f, "("),
            Token::PAREN2     => write!(f, ")"),
            Token::BLOCK1     => write!(f, "{{"),
            Token::BLOCK2     => write!(f, "}}"),
            Token::BRACK1     => write!(f, "["),
            Token::BRACK2     => write!(f, "]"),
            Token::IMPORT     => write!(f, "import"),
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
            NodeType::PREINCREMENT |
            NodeType::POSTINCREMENT                 => write!(f, "++"),
            NodeType::PREDECREMENT |
            NodeType::POSTDECREMENT                 => write!(f, "--"),
            NodeType::ACCESS                        => write!(f, "."),
            NodeType::INT(s)                        => write!(f, "{}", s),
            NodeType::DOUBLE(s)                     => write!(f, "{}", s),
            NodeType::STRING(s)                     => write!(f, "\"{}\"", s),
            NodeType::BOOL(v)                        => write!(f, "{}", v),
            NodeType::NAME(s)                       => write!(f, "{}", s),
            NodeType::TYPEDVAR(tp, name)  => write!(f, "{}:{}", name, tp),
            NodeType::FUNDEF(s)                     => write!(f, "{}() {{}}", s),
            NodeType::FUNCALL(s)                    => write!(f, "{}()", s),
            NodeType::METHODCALL(objname, methname) => write!(f, "{}.{}()", objname, methname),
            NodeType::LIST                          => write!(f, "[]"),
            NodeType::PARAMLIST                     => write!(f, "PARAMLIST"),
            NodeType::ARGLIST                       => write!(f, "ARGLIST"),
            NodeType::CONDITIONAL                   => write!(f, "CONDITIONAL"),
            NodeType::IF                            => write!(f, "IF"),
            NodeType::ELSEIF                        => write!(f, "ELSEIF"),
            NodeType::ELSE                          => write!(f, "ELSE"),
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
    dprint(" ");
    dprint("PARSE");
    dprint(" ");

    let mut root = Node::new(NodeType::MODULE);
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
    let directives_node = Node::new(NodeType::DIRECTIVES);

    while i < tokens.len() {

        match &tokens[i] {
            Token::IMPORT => {

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
        Token::NAME(_) => {

            let t2: &Token = tokens.get(i).unwrap();
            i += 1;

            match t2 {

                Token::NAME(fname) => {
                    let mut node = Node::new(NodeType::FUNDEF(fname.to_string()));
                    let (params, new_pos) = paramlist(tokens, i);
                    i = new_pos;
                    node.children.push(params);

                    let t3: &Token = tokens.get(i).unwrap();
                    i += 1;

                    match t3 {
                        Token::BLOCK1 => {
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
                        Token::INT(_)    |
                        Token::DOUBLE(_)
                        => {
                            let (arg, new_pos) = expression(tokens, j);

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
                            let (arg, new_pos) = expression(tokens, j);

                            node.children.push(arg);
                            j = new_pos;
                            expect_comma = true;
                            continue;
                        }

                        Token::PAREN2 => {
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

    let mut node = Node::new(NodeType::BLOCK);
    let mut i = pos;

    while i < tokens.len() {
        dprint(format!("Parse: block loop at: {}, token: {}", i, &tokens[i]));

        if tokens[i] == Token::BLOCK2 {
            dprint(String::from("Parse: token is end-of-block, breaking."));
            i += 1;
            break;
        }
        if tokens[i] == Token::END {
            dprint(String::from("Parse: token is end, breaking."));
            break;
        }

        let (snode, new_pos) = statement(tokens, i);
        node.children.push(snode);

        i = new_pos;

        match &tokens[i] {

            Token::BLOCK2 => {
                // i += 1;
                continue;
            }
            Token::ENDST => {
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

        Token::NAME(s) => {

            i = i + 1;
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
                            let (right_node, i) = expression(tokens, i);
                            ass_node.children.push(right_node);
                            dprint(format!("Parse: returning statement at token {}", i));
                            return (ass_node, i)
                        }

                        _ => panic!("Unexpected token in statement. Expected: =. Got: {}", t3)
                    }
                }

                Token::ASSIGN => {
                    i += 1;
                    let mut ass_node = Node::new(NodeType::ASSIGN);

                    let var = Node::new(NodeType::NAME(s.to_string()));
                    let (right_node, i) = expression(tokens, i);

                    ass_node.children.push(var);
                    ass_node.children.push(right_node);

                    return (ass_node, i)
                }

                Token::PAREN1 => {
                    // Function call.
                    let (args_node, new_pos) = arglist(tokens, i);
                    i = new_pos;
                    let mut funcall_node = Node::new(NodeType::FUNCALL(s.to_string()));
                    funcall_node.nodetype = NodeType::FUNCALL(s.to_string());
                    funcall_node.children.push(args_node);
                    return (funcall_node, i)
                }

                Token::ACCESS => {

                    i += 1;
                    let t3 = &tokens[i];

                    match t3 {

                        Token::NAME(acc_name) => {


                            i += 1;
                            let t4 = &tokens[i];

                            return match t4 {
                                Token::PAREN1 => {

                                    // method call
                                    let (args, new_pos) = arglist(tokens, i);
                                    i = new_pos;
                                    let mut methcall_node = Node::new(NodeType::METHODCALL(s.to_string(), acc_name.to_string()));
                                    methcall_node.children.push(args);

                                    if tokens[i] != Token::ENDST {
                                        panic!("Unexpected token at pos {}: {}", i, tokens[i]);
                                    }
                                    i += 1;

                                    (methcall_node, i)
                                }

                                _ => {
                                    let mut acc_node = Node::new(NodeType::ACCESS);
                                    let obj_node = Node::new(NodeType::NAME(s.to_string()));
                                    let member_node = Node::new(NodeType::NAME(acc_name.to_string()));
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

        Token::IF => {
            dprint("Parse: if");

            let mut condnode = Node::new(NodeType::CONDITIONAL);

            let (condpart, next_pos) = conditional(tokens, i);
            condnode.children.push(condpart);
            i = next_pos;


            loop {

                let next_token = &tokens[i];

                match next_token {

                    Token::ELSE => {
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

        _ => {
            return expression(tokens, pos);
        }
    }
}


fn conditional(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    let mut i = pos;

    match &tokens[i] {

        Token::IF => {

            i += 1;

            match tokens[i] {
                Token::PAREN1 => {
                    i += 1;
                    let (boolnode, new_pos) = expression(tokens, i);

                    match tokens[new_pos] {
                        Token::PAREN2 => {
                            i = new_pos + 1;

                            match tokens[i] {
                                Token::BLOCK1 => {
                                    i += 1;
                                    let (bodynode, new_pos) = block(tokens, i);

                                    i = new_pos;

                                    let mut ifnode = Node::new(NodeType::IF);
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
        Token::ELSE => {

            i += 1;

            match &tokens[i] {

                Token::IF => {

                    i += 1;

                    match tokens[i] {
                        Token::PAREN1 => {
                            i += 1;
                            let (boolnode, new_pos) = expression(tokens, i);

                            match tokens[new_pos] {
                                Token::PAREN2 => {
                                    i = new_pos + 1;

                                    match tokens[i] {
                                        Token::BLOCK1 => {
                                            i += 1;
                                            let (bodynode, new_pos) = block(tokens, i);
                                            i = new_pos;

                                            let mut elseifnode = Node::new(NodeType::ELSEIF);
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

                Token::BLOCK1 => {

                    i += 1;

                    let (bodynode, new_pos) = block(tokens, i);
                    i = new_pos;

                    let mut elsenode = Node::new(NodeType::ELSE);
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

    sum(tokens, pos)
}


fn sum(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: sum: {}", &tokens[pos]));

    let (left, next_pos) = product(tokens, pos);
    let c: &Token = tokens.get(next_pos).unwrap();

    return match c {
        Token::ADD => {
            let mut sum = Node::new(NodeType::ADD);
            sum.children.push(left);

            let (right, i) = expression(tokens, next_pos + 1);
            sum.children.push(right);

            dprint(format!("Parse: assembled sum: {}", &sum));
            (sum, i)
        },

        Token::SUB => {
            let mut sum = Node::new(NodeType::SUB);
            sum.children.push(left);

            let (right, i) = expression(tokens, next_pos + 1);
            sum.children.push(right);

            dprint(format!("Parse: assembled sum: {}", &sum));
            (sum, i)
        }

        _ => (left, next_pos)
    }
}


fn product(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {

    dprint(format!("Parse: product: {}", &tokens[pos]));

    let (left, mut i) = term(tokens, pos);
    let t: &Token = tokens.get(i).unwrap();

    return match t {
        Token::MUL => {
            let mut prod = Node::new(NodeType::MUL);
            prod.children.push(left);

            i += 1;

            let (right, i) = product(tokens, i);
            prod.children.push(right);
            (prod, i)
        }
        Token::DIV => {
            let mut div = Node::new(NodeType::DIV);
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

        &Token::INT(ref s) => {
            let node = Node::new(NodeType::INT(s.clone()));
            return (node, pos+1)
        }

        &Token::DOUBLE(ref s) => {
            let node = Node::new(NodeType::DOUBLE(s.clone()));
            return (node, pos+1)
        }

        &Token::STRING(ref s) => {
            let node = Node::new(NodeType::STRING(s.clone()));
            return (node, pos+1)
        }

        &Token::BOOL(v) => {
            let node = Node::new(NodeType::BOOL(v));
            return (node, pos+1)
        }

        &Token::NAME(ref s) => {

            // Postfixed inc/dec should be bound tightly, so handle
            // it here rather than in expression.
            if let Token::INCREMENT = tokens[pos+1] {
                let mut incnode = Node::new(NodeType::POSTINCREMENT);
                let node = Node::new(NodeType::NAME(s.clone()));
                incnode.children.push(node);
                return (incnode, pos + 2);
            }
            if let Token::DECREMENT = tokens[pos+1] {
                let mut decnode = Node::new(NodeType::POSTDECREMENT);
                let node = Node::new(NodeType::NAME(s.clone()));
                decnode.children.push(node);
                return (decnode, pos + 2);
            }

            let node = Node::new(NodeType::NAME(s.clone()));
            return (node, pos+1)
        }

        &Token::INCREMENT => {

            let next = &tokens[pos+1];
            return match next {
                Token::NAME(s) => {
                    let namenode = Node::new(NodeType::NAME(s.clone()));
                    let mut incnode = Node::new(NodeType::PREINCREMENT);
                    incnode.children.push(namenode);
                    (incnode, pos + 2)
                }
                _ => panic!("Invalid operand for increment: {}", next)
            }
        }

        &Token::DECREMENT => {

            let next = &tokens[pos+1];
            return match next {
                Token::NAME(s) => {
                    let namenode = Node::new(NodeType::NAME(s.clone()));
                    let mut incnode = Node::new(NodeType::PREDECREMENT);
                    incnode.children.push(namenode);
                    (incnode, pos + 2)
                }
                _ => panic!("Invalid operand for decrement: {}", next)
            }
        }

        &Token::PAREN1 => {
            let (wnode, new_pos) = expression(tokens, pos+1);
            if let &Token::PAREN2 = &tokens[new_pos] {
                return (wnode, new_pos + 1)
            }
            else {
                panic!("Expected closing parenthesis at {} but found {}", new_pos, tokens[new_pos])
            }
        }

        &Token::BRACK1 => {

            let mut i = pos + 1;
            let mut list_node = Node::new(NodeType::LIST);
            let mut expect_sep = false;

            if tokens[i] == Token::BRACK2 {
                return (list_node, i + 1)
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

