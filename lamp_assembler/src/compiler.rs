use lamp_asm_parser::lexer::{Lexer, LexerError, TokenType, TokenizedLine};

pub struct Compiler {
    origin: Vec<String>,
    pub result_buffer: Vec<u8>,
}

impl Compiler {
    pub fn new(origin: Vec<String>) -> Self {
        Self {
            origin,
            result_buffer: vec![],
        }
    }

    pub fn compile(&mut self) -> Result<usize, Vec<LexerError>> {
        let tokenization = Lexer::new().tokenize(&self.origin);

        match tokenization {
            Ok(tokenized_lines) => {
                for line in tokenized_lines {
                    let buff = self.compile_token(line);

                    for byte in buff {
                        self.result_buffer.push(byte);
                    }
                }
            }
            Err(e) => return Err(e),
        }
        Ok(self.result_buffer.len())
    }

    pub fn compile_token(&mut self, token: TokenizedLine) -> Vec<u8> {
        let mut returned_vec = Vec::<u8>::new();

        returned_vec.push(token.opcode as u8);
        // Should be changed later
        for operand in token.operands {
            match operand {
                TokenType::Num8(n) => returned_vec.push(n),
                TokenType::Ptr8(n) => returned_vec.push(n),
                TokenType::Opcode(_) => panic!("Should never happen: Unexpected opcode found"),
            }
        }
        returned_vec
    }
}
