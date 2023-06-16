use token::Token;

pub struct Reader {
    pos: usize,
    tokens: Vec<Token>
}


impl Reader {

    pub fn new(tokens: Vec<Token>) -> Reader {
        Reader {
            pos: 0,
            tokens
        }
    }


    pub fn sym(&self) -> Token {
        if self.pos >= self.tokens.len() {

        }
        return self.tokens[self.pos].clone();
    }

    pub fn next(&mut self) -> Token {
        self.pos += 1;
        return self.tokens[self.pos].clone();
    }

    pub fn peek(&self) -> Token {
        return self.tokens[self.pos + 1].clone();
    }

    pub fn tokens(&self) -> &Vec<Token> {
        return &self.tokens;
    }

    pub fn position(&self) -> usize {
        return self.pos;
    }

    pub fn len(&self) -> usize {
        return self.tokens.len();
    }
 }
