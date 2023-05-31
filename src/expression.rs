use token::Token;
use node::{NodeType, Node};
use parser::arglist;
use utils::{dprint, darterror};
use queues::*;


pub fn expression(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Parse: expression: {}", &tokens[pos]));

    disjunction(tokens, pos)
}


// The following are operators as given by
// https://dart.dev/language/operators
// Ordered from loose to tight.


fn disjunction(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Parse: disjunction: {}", &tokens[pos]));

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
    dprint(format!("Parse: conjunction: {}", &tokens[pos]));

    let (left, next_pos) = equality(tokens, pos);
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


fn equality(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Parse: equality: {}", &tokens[pos]));

    let (left, next_pos) = comparison(tokens, pos);
    let t: &Token = &tokens[next_pos];

    return match t {
        Token::Equal => {

            let (right, i) = comparison(tokens, next_pos + 1);

            let mut eqnode = Node::new(NodeType::Equal);
            eqnode.children.push(left);
            eqnode.children.push(right);

            (eqnode, i)
        }

        _ => (left, next_pos)
    }
}


fn comparison(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Parse: comparison: {}", &tokens[pos]));

    let (left, next_pos) = bitwise(tokens, pos);
    let t: &Token = &tokens[next_pos];

    return match t {
        Token::LessThan => {

            let (right, i) = bitwise(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::LessThan);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }
        Token::GreaterThan => {

            let (right, i) = bitwise(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::GreaterThan);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }
        Token::LessOrEq => {

            let (right, i) = bitwise(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::LessOrEq);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }
        Token::GreaterOrEq => {

            let (right, i) = bitwise(tokens, next_pos + 1);

            let mut connode = Node::new(NodeType::GreaterOrEq);
            connode.children.push(left);
            connode.children.push(right);

            (connode, i)
        }

        _ => (left, next_pos)
    }
}


fn bitwise(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Parse: bitwise: {}", &tokens[pos]));

    let (left, next_pos) = sum(tokens, pos);
    let c: &Token = tokens.get(next_pos).unwrap();

    return match c {
        Token::BitAnd => {
            let mut node = Node::new(NodeType::BitAnd);
            node.children.push(left);

            let (right, i) = bitwise(tokens, next_pos + 1);
            node.children.push(right);

            (node, i)
        }
        Token::BitOr => {
            let mut node = Node::new(NodeType::BitOr);
            node.children.push(left);

            let (right, i) = bitwise(tokens, next_pos + 1);
            node.children.push(right);

            (node, i)
        }

        _ => (left, next_pos)
    }
}


fn sum(tokens: &Vec<Token>, pos: usize) -> (Node, usize) {
    dprint(format!("Parse: sum: {}", &tokens[pos]));

    sum_help(tokens, pos, &mut queue![], &mut queue![])
}


fn sum_help(tokens: &Vec<Token>, pos: usize, righties: &mut Queue<Node>, ops: &mut Queue<Node>) -> (Node, usize) {

    let (n, next_pos) = product(tokens, pos);
    let c: &Token = tokens.get(next_pos).unwrap();

    righties.add(n);

    return match c {

        Token::Add => {

            ops.add(Node::new(NodeType::Add));

            let (deeper, i) = sum_help(tokens, next_pos + 1, righties, ops);

            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());

            (node, i)
        }
        Token::Sub => {

            ops.add(Node::new(NodeType::Sub));

            let (deeper, i) = sum_help(tokens, next_pos + 1, righties, ops);

            let mut node = ops.remove().unwrap();
            node.children.push(deeper);
            node.children.push(righties.remove().unwrap());

            (node, i)
        }

        _ => {
            (righties.remove().unwrap(), next_pos)
        }
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