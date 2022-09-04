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

pub enum Direction {
	Left,
	Right,
}

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

pub enum Token {
	Keyword(Keyword),
	Identifier,
	Symbol,
	String(String),
	Integer(u16),
}

pub fn tokenize(source: &str) -> Vec<Token> {
	todo!()
}

#[cfg(test)]
mod tests {

}
