use token::Token;
use utils::dprint;


/// Not applicable for first char in name, where only letters are allowed
fn is_legal_namechar(c: char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || (c == '_')
}


fn read_word(tokens: &mut Vec<Token>, chars: &[char], start: usize) -> usize {
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
        tokens.push(Token::Import);
    }
    else if &sym == "true" {
        tokens.push(Token::Bool(true));
    }
    else if &sym == "false" {
        tokens.push(Token::Bool(false));
    }
    else if &sym == "if" {
        tokens.push(Token::If);
    }
    else if &sym == "else" {
        tokens.push(Token::Else);
    }
    else if &sym == "return" {
        tokens.push(Token::Return);
    }
    else if &sym == "class" {
        tokens.push(Token::Class);
    }
    else {
        tokens.push(Token::Name(sym));
    }
    return len;
}


pub fn lex(input: &str) -> Vec<Token> {
    let (tokens, pos) = lex_real(input, 0, 0);
    return tokens;
}


pub fn lex_real(input: &str, mut startpos: usize, mut interpol: usize) -> (Vec<Token>, usize) {

    dprint(" ");
    dprint("LEX");
    dprint(" ");

    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let inp_length = chars.len();
    let mut i: usize = startpos;
    let mut c: char;


    while i < inp_length {

        c = chars[i];

        match c {

            ' ' | '\n' => {}

            '"' => {
                println!("Quote");
                let mut k = 1;
                let mut s = String::new();
                let mut closed = false;
                let mut interpolations: isize = 0;
                let mut subs: Vec<Vec<Token>> = Vec::new();

                while i+k < inp_length {

                    let nc = input.get(i+k .. i+k+1).unwrap();

                    if nc == "\"" {
                        closed = true;
                        k += 1;
                        break;
                    }

                    if nc == "$" {
                        println!("found $");
                        s.push(input.get(i+k .. i+k+1).unwrap().chars().next().unwrap());
                        k += 1;
                        interpolations += 1;
                        let nnc = input.get(i+k .. i+k+1).unwrap();
                        if nnc == "{" {

                            let (sublex, new_pos) = lex_real(input, i+k + 1, interpol + 1);
                            subs.push(sublex);
                            k = new_pos - i;

                            println!("Finished interpol on pos: {}", i);
                        }
                    }
                    else {
                        s.push((input.get(i + k..i + k + 1)).unwrap().chars().next().unwrap());
                        k += 1;
                    }
                }
                if !closed {
                    panic!("Unclosed quote!");
                }
                tokens.push(Token::Str(s, subs));
                i += k;
                println!("finished string on pos: {}", i);
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
                        tokens.push(Token::Div);
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
                let mut subs: Vec<Vec<Token>> = Vec::new();

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
                tokens.push(Token::Str(s, subs));
                i += k;
                continue;
            }

            '(' => {
                tokens.push(Token::Paren1);
            }

            ')' => {
                tokens.push(Token::Paren2);
            }

            '{' => {
                tokens.push(Token::Block1);
            }

            '}' => {
                if interpol > 0 {
                    interpol -= 1;
                    return (tokens, i+1);
                }
                tokens.push(Token::Block2);
            }

            '[' => {
                tokens.push(Token::Brack1);
            }

            ']' => {
                tokens.push(Token::Brack2);
            }

            '.' => {
                tokens.push(Token::Access);
            }

            ',' => {
                tokens.push(Token::Comma);
            }

            ';' => {
                tokens.push(Token::EndSt);
            }

            '=' => {
                if chars[i+1] == '=' {
                    tokens.push(Token::Equal);
                    i += 2;
                    continue;
                }
                tokens.push(Token::Assign);
            }

            '+' => {
                if chars[i+1] == '+' {
                    tokens.push(Token::Increment);
                    i += 2;
                    continue;
                }
                tokens.push(Token::Add);
            }

            '-' => {
                if chars[i+1] == '-' {
                    tokens.push(Token::Decrement);
                    i += 2;
                    continue;
                }
                tokens.push(Token::Sub);
            }

            '*' => {
                tokens.push(Token::Mul);
            }

            '<' => {
                if chars[i+1] == '=' {
                    tokens.push(Token::LessOrEq);
                    i += 2;
                    continue;
                }
                tokens.push(Token::LessThan);
            }

            '>' => {
                if chars[i+1] == '=' {
                    tokens.push(Token::GreaterOrEq);
                    i += 2;
                    continue;
                }
                tokens.push(Token::GreaterThan);
            }

            '|' => {
                if chars[i+1] == '|' {
                    tokens.push(Token::LogOr);
                    i += 2;
                    continue;
                }
                tokens.push(Token::BitOr);
            }

            '&' => {
                if chars[i+1] == '&' {
                    tokens.push(Token::LogAnd);
                    i += 2;
                    continue;
                }
                tokens.push(Token::BitAnd);
            }

            '^' => {
                tokens.push(Token::BitXor);
            }

            '!' => {
                tokens.push(Token::Not);
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
                    tokens.push(Token::Int(String::from(val)));
                }
                else {
                    tokens.push(Token::Double(String::from(val)));
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

    tokens.push(Token::End);
    (tokens, i)
}
