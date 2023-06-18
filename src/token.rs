use std::fmt;

// The pair of usize fields in Token are respectively
// the line number and the position of first char of
// the token within the line.

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Token {
  // Arithmetic
  Add(usize, usize),
  Sub(usize, usize),
  Mul(usize, usize),
  Div(usize, usize),
  Increment(usize, usize),
  Decrement(usize, usize),
  // Logic
  Not(usize, usize),
  LogOr(usize, usize),
  LogAnd(usize, usize),
  BitOr(usize, usize),
  BitXor(usize, usize),
  BitAnd(usize, usize),
  // Relation
  LessThan(usize, usize),
  GreaterThan(usize, usize),
  LessOrEq(usize, usize),
  GreaterOrEq(usize, usize),
  Equal(usize, usize),
  // Primitive
  Int(String, usize, usize),
  Double(String, usize, usize),
  Str(String, Vec<Vec<Token>>, usize, usize),
  Bool(bool, usize, usize),
  Name(String, usize, usize),
  // Structure
  Class(usize, usize),
  If(usize, usize),
  Else(usize, usize),
  While(usize, usize),
  Do(usize, usize),
  For(usize, usize),
  Paren1(usize, usize),
  Paren2(usize, usize),
  Block1(usize, usize),
  Block2(usize, usize),
  Brack1(usize, usize),
  Brack2(usize, usize),
  Comma(usize, usize),
  // Other
  Assign(usize, usize),
  Access(usize, usize),
  Return(usize, usize),
  Import(usize, usize),
  EndSt(usize, usize),
  End
}


impl fmt::Display for Token {

  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      // Arithmetic
      Token::Add(_, _) => write!(f, "+"),
      Token::Sub(_, _) => write!(f, "-"),
      Token::Mul(_, _) => write!(f, "*"),
      Token::Div(_, _) => write!(f, "/"),
      Token::Increment(_, _) => write!(f, "++"),
      Token::Decrement(_, _) => write!(f, "--"),
      // Logic
      Token::Not(_, _) => write!(f, "!"),
      Token::LogOr(_, _) => write!(f, "||"),
      Token::LogAnd(_, _) => write!(f, "&&"),
      Token::BitOr(_, _) => write!(f, "|"),
      Token::BitXor(_, _) => write!(f, "^"),
      Token::BitAnd(_, _) => write!(f, "&"),
      // Relation
      Token::LessThan(_, _)    => write!(f, "<"),
      Token::GreaterThan(_, _) => write!(f, ">"),
      Token::LessOrEq(_, _)    => write!(f, "<="),
      Token::GreaterOrEq(_, _) => write!(f, ">="),
      Token::Equal(_, _) => write!(f, "=="),
      // Primitive
      Token::Int(s, _, _)     => write!(f, "{}", s),
      Token::Double(s, _, _)     => write!(f, "{}", s),
      Token::Str(s, interpols, _, _)  => {
        if interpols.len() > 0 {
          write!(f, "\"{}\"", s).ok();
          write!(f, "( ").ok();
          for itp in interpols {
            for t in itp {
              write!(f, "{} ", t).ok();
            }
          }
          write!(f, ")")
        }
        else {
          write!(f, "\"{}\"", s)
        }
      },
      Token::Bool(v, _, _)     => write!(f, "{}", v),
      Token::Name(s, _, _)    => write!(f, "{}", s),
      // Structure
      Token::Class(_, _) => write!(f, "class"),
      Token::If(_, _) => write!(f, "if"),
      Token::Else(_, _) => write!(f, "else"),
      Token::While(_, _) => write!(f, "while"),
      Token::Do(_, _) => write!(f, "do"),
      Token::For(_, _) => write!(f, "for"),
      Token::Paren1(_, _) => write!(f, "("),
      Token::Paren2(_, _) => write!(f, ")"),
      Token::Block1(_, _) => write!(f, "{{"),
      Token::Block2(_, _) => write!(f, "}}"),
      Token::Brack1(_, _) => write!(f, "["),
      Token::Brack2(_, _) => write!(f, "]"),
      Token::Comma(_, _) => write!(f, ","),
      // Other
      Token::Assign(_, _) => write!(f, "="),
      Token::Access(_, _) => write!(f, "."),
      Token::Return(_, _) => write!(f, "return"),
      Token::Import(_, _) => write!(f, "import"),
      Token::EndSt(_, _) => write!(f, "ENDST"),
      Token::End => write!(f, "END"),
    }
  }
}


impl Token {

  pub fn find_token_position(&self) -> (usize, usize) {
    return match self {
      // Arithmetic
      Token::Add(l, i) |
      Token::Sub(l, i) |
      Token::Mul(l, i) |
      Token::Div(l, i) |
      Token::Increment(l, i) |
      Token::Decrement(l, i) |
      // Logic
      Token::Not(l, i) |
      Token::LogOr(l, i) |
      Token::LogAnd(l, i) |
      Token::BitOr(l, i) |
      Token::BitXor(l, i) |
      Token::BitAnd(l, i) |
      // Relation
      Token::LessThan(l, i) |
      Token::GreaterThan(l, i) |
      Token::LessOrEq(l, i) |
      Token::GreaterOrEq(l, i) |
      Token::Equal(l, i) |
      // Primitive
      Token::Int(_, l, i) |
      Token::Double(_, l, i) |
      Token::Str(_, _, l, i) |
      Token::Bool(_, l, i) |
      Token::Name(_, l, i) |
      // Structure
      Token::Class(l, i) |
      Token::If(l, i) |
      Token::Else(l, i) |
      Token::While(l, i) |
      Token::Do(l, i) |
      Token::For(l, i) |
      Token::Paren1(l, i) |
      Token::Paren2(l, i) |
      Token::Block1(l, i) |
      Token::Block2(l, i) |
      Token::Brack1(l, i) |
      Token::Brack2(l, i) |
      Token::Comma(l, i) |
      // Other
      Token::Assign(l, i) |
      Token::Access(l, i) |
      Token::Return(l, i) |
      Token::Import(l, i) |
      Token::EndSt(l, i) => {
        (l.clone(), i.clone())
      },
      Token::End => panic!("Requested position of End token")
    }
  }

}
