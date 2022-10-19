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

impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "&" => Operator::And,
            "|" => Operator::Or,
            "~" => Operator::Not,
            "<" => Operator::Lesser,
            ">" => Operator::Greater,
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

impl TryFrom<&str> for Symbol {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "(" => Symbol::Parenthese(Sided::Left),
            ")" => Symbol::Parenthese(Sided::Right),
            "[" => Symbol::Bracket(Sided::Left),
            "]" => Symbol::Bracket(Sided::Right),
            "{" => Symbol::Brace(Sided::Left),
            "}" => Symbol::Brace(Sided::Right),
            "," => Symbol::Comma,
            ";" => Symbol::Semicolon,
            "=" => Symbol::Equals,
            "." => Symbol::Period,
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

#[allow(dead_code)]
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut start_index: usize = 0;

    while start_index < source.len() {
        let remaining = &source[start_index..];

        if remaining.starts_with(' ') {
            start_index += 1;
            continue;
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn identifier() {}
}
