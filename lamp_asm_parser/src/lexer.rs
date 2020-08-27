use lamp_common::op::{get_op, Opcode};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // mov for example
    Opcode(Opcode),
    // #15 for example
    Num8(u8),
    // $15 for example
    Ptr8(u8),
}

// Different types of errors the lexer can encounter
#[derive(Debug)]
pub enum LexerError {
    UnexpectedToken(usize, String),
    InvalidMnemonic(usize, String),
    InvalidLine(usize),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            /* at + 1 because at is an array/vec index */
            Self::InvalidLine(at) => {
                write!(f, "Line {} contains one or more invalid tokens", at + 1)
            }
            Self::UnexpectedToken(at, tkn) => {
                write!(f, "Unexpected token at line {}: \'{}\'", at + 1, tkn)
            }
            Self::InvalidMnemonic(at, mnemonic) => {
                write!(f, "Invalid mnemonic at line {}: \'{}\'", at + 1, mnemonic)
            }
        }
    }
}

pub struct Lexer;

#[derive(PartialEq, Debug)]
pub struct TokenizedLine {
    pub opcode: Opcode,
    pub operands: Vec<TokenType>,
}

impl TokenizedLine {
    pub fn empty() -> Self {
        TokenizedLine {
            opcode: Opcode::NOP,
            operands: vec![],
        }
    }
}

impl Lexer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    #[allow(clippy::ptr_arg)]
    pub fn tokenize(&self, content: &Vec<String>) -> Result<Vec<TokenizedLine>, Vec<LexerError>> {
        let mut tokenized = Vec::<TokenizedLine>::new();
        let mut errors = Vec::<LexerError>::new();

        #[allow(clippy::needless_range_loop)]
        for i in 0..content.len() {
            if content[i].starts_with(lamp_common::constants::COMMENT_MARKER) {
                continue;
            }
            match self.tokenize_line(&content[i], i) {
                Ok(tkn) => {
                    if errors.is_empty() {
                        tokenized.push(tkn);
                    }
                }
                Err(e) => errors.push(e),
            }
        }
        if errors.is_empty() {
            return Ok(tokenized);
        }
        Err(errors)
    }

    // This function tokenizes all the line
    pub fn tokenize_line(&self, line: &str, line_num: usize) -> Result<TokenizedLine, LexerError> {
        if let Some(index) = line.find(' ') {
            let to_tokenize = line.split_at(index);
            // The first token of a line MUST be a mnemonic...
            let mut tokenized = TokenizedLine::empty();
            if let Ok(opcode) = get_op(to_tokenize.0.to_owned()) {
                tokenized.opcode = opcode;

                match self.tokenize_operands(to_tokenize.1.to_owned(), line_num) {
                    Ok(tkns) => {
                        tokenized.operands = tkns;
                        Ok(tokenized)
                    }
                    Err(e) => Err(e),
                }
            } else {
                Err(LexerError::InvalidMnemonic(
                    line_num,
                    to_tokenize.0.to_owned(),
                ))
            }
        } else {
            Err(LexerError::InvalidLine(line_num))
        }
    }
    // This function tokenizes all operands given
    pub fn tokenize_operands(
        &self,
        line: String,
        line_num: usize,
    ) -> Result<Vec<TokenType>, LexerError> {
        let to_tokenize: Vec<&str> = line.split(',').collect();
        let mut tokens = Vec::<TokenType>::new();

        for tkn in to_tokenize {
            let tkn = tkn.split_whitespace().next();
            match tkn {
                Some(tkn) => match self.from_number_to_token(tkn.to_string(), line_num) {
                    Ok(tok) => tokens.push(tok),
                    Err(e) => return Err(e),
                },
                None => break,
            }
        }
        Ok(tokens)
    }

    // This line takes a number and converts it into a TokenType if possible, or LexerError if not
    pub fn from_number_to_token(&self, tkn: String, line: usize) -> Result<TokenType, LexerError> {
        let split = &tkn.split_at(1);

        match split.1.parse::<u8>() {
            Ok(number) => match split.0 {
                "#" => Ok(TokenType::Num8(number)),
                "$" => Ok(TokenType::Ptr8(number)),
                other => Err(LexerError::UnexpectedToken(line, other.to_owned())),
            },
            Err(_) => Err(LexerError::UnexpectedToken(line, tkn)),
        }
    }
}
