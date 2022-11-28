use std::str::FromStr;

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

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "class" => Keyword::Class,
            "method" => Keyword::Method,
            "function" => Keyword::Function,
            "constructor" => Keyword::Constructor,
            "int" => Keyword::Int,
            "bool" => Keyword::Bool,
            "char" => Keyword::Char,
            "void" => Keyword::Void,
            "var" => Keyword::Var,
            "static" => Keyword::Static,
            "field" => Keyword::Field,
            "let" => Keyword::Let,
            "do" => Keyword::Do,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "return" => Keyword::Return,
            "true" => Keyword::True,
            "false" => Keyword::False,
            "null" => Keyword::Null,
            "this" => Keyword::This,
            _ => return Err(()),
        })
    }
}

#[allow(dead_code)]
pub enum Sided {
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

impl TryFrom<char> for Operator {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '+' => Operator::Add,
            '-' => Operator::Subtract,
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            '&' => Operator::And,
            '|' => Operator::Or,
            '~' => Operator::Not,
            '<' => Operator::Lesser,
            '>' => Operator::Greater,
            _ => return Err(()),
        })
    }
}

#[allow(dead_code)]
pub enum Symbol {
    Parenthese(Sided),
    Bracket(Sided),
    Brace(Sided),
    Comma,
    Semicolon,
    Equals,
    Period,
    Operator(Operator),
}

impl TryFrom<char> for Symbol {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '(' => Symbol::Parenthese(Sided::Left),
            ')' => Symbol::Parenthese(Sided::Right),
            '[' => Symbol::Bracket(Sided::Left),
            ']' => Symbol::Bracket(Sided::Right),
            '{' => Symbol::Brace(Sided::Left),
            '}' => Symbol::Brace(Sided::Right),
            ',' => Symbol::Comma,
            ';' => Symbol::Semicolon,
            '=' => Symbol::Equals,
            '.' => Symbol::Period,
            s => Symbol::Operator(Operator::try_from(s)?),
        })
    }
}

#[allow(dead_code)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Symbol(Symbol),
    String(String),
    Integer(u16),
}

lazy_static! {
    static ref RE_IDENTIFIER: Regex =
        Regex::new(r"^[a-zA-Z]([a-zA-Z]|\d)*").expect("Failed to build identifier regex");
    static ref RE_STRING_CONSTANT: Regex =
        Regex::new("^(?:\")(.*)(?:\")").expect("Failed to build string constant regex");
    static ref RE_INTEGER_CONSTANT: Regex =
        Regex::new(r"^\d+").expect("Failed to build integer constant regex");
}

#[allow(dead_code)]
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut start_index: usize = 0;
    let mut tokens = Vec::<Token>::new();

    while start_index < source.len() {
        let remaining = &source[start_index..];

        // Forward past whitespace
        if remaining.starts_with(' ') {
            start_index += 1;
            continue;
        }

        // Check if first char is symbol
        let first = remaining.chars().next().unwrap();
        if let Ok(symbol) = Symbol::try_from(first) {
            tokens.push(Token::Symbol(symbol));
            let next_index = remaining.char_indices().nth(1);
            start_index += match next_index {
                Some((i, _)) => i,
                None => break,
            };
            continue;
        }

        // Check for identifier
        if let Some(m) = RE_IDENTIFIER.find(remaining) {
            // Check for keyword
            if let Ok(keyword) = Keyword::try_from(m.as_str()) {
                tokens.push(Token::Keyword(keyword));
                start_index += m.as_str().len();
                continue;
            }

            // Push identifier
            tokens.push(Token::Identifier(String::from_str(m.as_str()).unwrap()));
            continue;
        }

        // Check for string constant
        if let Some(m) = RE_STRING_CONSTANT.find(remaining) {
            tokens.push(Token::String(String::from(m.as_str())));
            continue;
        }

        // Check for integer constant
        if let Some(_m) = RE_INTEGER_CONSTANT.find(remaining) {
            continue;
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    #[test]
    fn identifier() {}
}
