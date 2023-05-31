use token::Token;
use parser;
use utils::dprint;


/// Not applicable for first char in name, where only letters are allowed
fn is_legal_namechar(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || (c == '_')
}


fn read_word(tokens: &mut Vec<parser::Token>, chars: &[char], start: usize) -> usize {
    let mut len: usize = 0;
    let mut sym = String::from("");

    while start + len < chars.len() {
        let nc: char = chars[start + len];
        if is_legal_namechar(nc) {
            sym.push(nc);
            len += 1;
            continue;
        }
        break;
    }

    if &sym == "import" {
        tokens.push(parser::Token::Import);
    }
    else if &sym == "true" {
        tokens.push(parser::Token::Bool(true));
    }
    else if &sym == "false" {
        tokens.push(parser::Token::Bool(false));
    }
    else if &sym == "if" {
        tokens.push(parser::Token::If);
    }
    else if &sym == "else" {
        tokens.push(parser::Token::Else);
    }
    else if &sym == "return" {
        tokens.push(parser::Token::Return);
    }
    else if &sym == "class" {
        tokens.push(parser::Token::Class);
    }
    else {
        tokens.push(parser::Token::Name(sym));
    }
    return len;
}



pub fn lex(input: &str) -> Vec<parser::Token> {

    dprint(" ");
    dprint("LEX");
    dprint(" ");

    let mut tokens: Vec<parser::Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let inp_length = chars.len();
    let mut i: usize = 0;
    let mut c: char;


    while i < inp_length {

        c = chars[i];

        match c {

            ' ' | '\n' => {}

            '"' => {
                let mut k = 1;
                let mut s = String::new();
                let mut closed = false;

                while i+k < inp_length {

                    let nc = input.get(i+k .. i+k+1).unwrap();

                    if nc == "\"" {
                        closed = true;
                        k += 1;
                        break;
                    }

                    s.push((input.get(i + k..i + k + 1)).unwrap().chars().next().unwrap());
                    k += 1;
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(parser::Token::Str(s));
                i += k;
                continue;
            }

            '/' => {
                i = i + 1;
                if inp_length > i {
                    if chars[i] == '/' {
                        i += 1;
                        while i < inp_length  {
                            if chars[i] == '\n' {
                                i += 1;
                                break;
                            }
                            i += 1;
                        }
                    }
                    else {
                        tokens.push(parser::Token::Div);
                    }
                }
                else {
                    panic!("Unexpected end of input: '/'");
                }
                continue;
            }

            '\'' => {
                let mut k = 1;
                let mut s = String::new();
                let mut closed = false;

                while i+k < inp_length {

                    let nc = input.get(i+k .. i+k+1).unwrap();

                    if nc == "'" {
                        closed = true;
                        k += 1;
                        break;
                    }

                    s.push((input.get(i + k..i + k + 1)).unwrap().chars().next().unwrap());
                    k += 1;
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(parser::Token::Str(s));
                i += k;
                continue;
            }

            '(' => {
                tokens.push(parser::Token::Paren1);
            }

            ')' => {
                tokens.push(parser::Token::Paren2);
            }

            '{' => {
                tokens.push(parser::Token::Block1);
            }

            '}' => {
                tokens.push(parser::Token::Block2);
            }

            '[' => {
                tokens.push(parser::Token::Brack1);
            }

            ']' => {
                tokens.push(parser::Token::Brack2);
            }

            '.' => {
                tokens.push(parser::Token::Access);
            }

            ',' => {
                tokens.push(parser::Token::Comma);
            }

            ';' => {
                tokens.push(parser::Token::EndSt);
            }

            '=' => {
                if chars[i+1] == '=' {
                    tokens.push(parser::Token::Equal);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::Assign);
            }

            '+' => {
                if chars[i+1] == '+' {
                    tokens.push(parser::Token::Increment);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::Add);
            }

            '-' => {
                if chars[i+1] == '-' {
                    tokens.push(parser::Token::Decrement);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::Sub);
            }

            '*' => {
                tokens.push(parser::Token::Mul);
            }

            '<' => {
                if chars[i+1] == '=' {
                    tokens.push(parser::Token::LessOrEq);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::LessThan);
            }

            '>' => {
                if chars[i+1] == '=' {
                    tokens.push(parser::Token::GreaterOrEq);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::GreaterThan);
            }

            '|' => {
                if chars[i+1] == '|' {
                    tokens.push(parser::Token::LogOr);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::BitOr);
            }

            '&' => {
                if chars[i+1] == '&' {
                    tokens.push(parser::Token::LogAnd);
                    i += 2;
                    continue;
                }
                tokens.push(parser::Token::BitAnd);
            }

            '!' => {
                tokens.push(parser::Token::Not);
            }

            x if x.is_digit(10) => {
                let mut nl = 1;
                let mut nc: char;
                let mut is_int: bool = true;

                while i + nl < inp_length {
                    nc = input.get(i + nl .. i + nl + 1).unwrap().chars().next().unwrap();
                    if nc.is_digit(10) {
                        nl += 1;
                        continue;
                    }
                    else if nc == '.' {
                        if input.get(i+nl-1 .. i+nl) == Some(".") {
                            panic!("Unexpected symbol: \".\"");
                        }
                        is_int = false;
                        nl += 1;
                        continue;
                    }
                    break;
                }

                let val: &str = input.get(i .. i + nl).unwrap();
                if is_int {
                    tokens.push(parser::Token::Int(String::from(val)));
                }
                else {
                    tokens.push(parser::Token::Double(String::from(val)));
                }
                i += nl;
                continue;
            }

            x if x.is_alphabetic() => {
                let word_len: usize = read_word(&mut tokens, &chars, i);
                if word_len > 0 {
                    i += word_len;
                    continue;
                }
            }

            _ => {
                panic!("Unrecognized symbol: {}", c);
            }
        }

        i += 1;
    }

    tokens.push(parser::Token::End);
    tokens
}
