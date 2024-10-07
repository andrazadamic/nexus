pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter(char),
}

pub fn lexer() -> Result<Vec<Token>, LexerError> {
    let result = vec![];
    Ok(result)
}
