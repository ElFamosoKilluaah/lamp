use crate::lexer::{Lexer, TokenType, TokenizedLine};
use lamp_common::op::Opcode;

#[test]
fn test_parse() {
    let to_parse = "LOAD #30, #20, #16";
    let lexer = Lexer::new();

    let res = lexer.tokenize_line(to_parse, 1);

    assert_eq!(res.is_ok(), true);

    let expected = TokenizedLine {
        opcode: Opcode::LOAD,
        operands: vec![
            TokenType::Num8(30),
            TokenType::Num8(20),
            TokenType::Num8(16),
        ],
    };

    assert_eq!(res.unwrap(), expected);
}

#[test]
fn test_parse_error() {
    let to_parse = "ZIZI #30, '0, #caca";
    let lexer = Lexer::new();

    let res = lexer.tokenize_line(to_parse, 1);

    assert_eq!(res.is_err(), true);
}
