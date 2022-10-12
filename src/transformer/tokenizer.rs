use regex::Regex;

#[allow(dead_code)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Bool,
    Char,
    Void,
    Var,
    Static,
    Field,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
}

#[allow(dead_code)]
pub enum Direction {
    Left,
    Right,
}

#[allow(dead_code)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Not,
    Lesser,
    Greater,
}

#[allow(dead_code)]
pub enum Symbol {
    Parenthese(Direction),
    Bracket(Direction),
    Brace(Direction),
    Comma,
    Semicolon,
    Equals,
    Period,
    Operator(Operator),
}

#[allow(dead_code)]
pub enum Token {
    Keyword(Keyword),
    Identifier,
    Symbol,
    String(String),
    Integer(u16),
}

lazy_static! {
    static ref REGEX_IDENTIFIER: Regex = Regex::new("").unwrap();
}

#[allow(dead_code)]
pub fn tokenize(_source: &str) -> Vec<Token> {
    todo!()
}

#[cfg(test)]
mod tests {}
