macro_rules! opcodes {
  { $($variant: ident = $value: expr, op = $operands: expr),+$(,)? } =>
    {
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum Opcode {
            $($variant = $value),+,
        }
        pub const OPCODES: &'static [Opcode] = &[$(Opcode::$variant),*];
        pub const OPCODES_OPERANDS: &'static [(Opcode, usize)] = &[$((Opcode::$variant , $operands)),*];
        pub const OPCODES_STRINGS: &'static [(Opcode, &str)] = &[$((Opcode::$variant , stringify!(Opcode::$variant))),*];
    }
}
// First number is the mnemonic's ID.
// op is the number of operands the mnemonic needs
opcodes! {
    // Arithmetical instructions
    ADD = 1, op = 3,
    SUB = 2, op = 3,
    MUL = 3, op = 3,
    MOD = 4, op = 3,
    INC = 5, op = 1,
    DEC = 6, op = 1,
    // Control Flow instructions
    EQ =  7, op = 2,
    NEQ = 8, op = 2,
    GT =  9, op = 2,
    GTE = 10, op = 2,
    LT =  11, op = 2,
    LTE = 12, op = 2,
    HLT = 13, op = 0,
    NOP = 14, op = 0,
    LOAD = 15, op = 3,
    JMP = 16, op = 1,
    // Takes the last modulo's remaining and put it into the specified register
    MODR = 17, op = 1,
    // Put in the register the specified value
    // MOV = 18,
}

pub fn decode_opcode(val: u8) -> Option<Opcode> {
    for code in OPCODES {
        if *code as u8 == val {
            return Some(*code);
        }
    }
    None
}

pub fn get_op<'a>(base: String) -> Result<Opcode, &'a str> {
    for (opcode, name) in OPCODES_STRINGS {
        if base.to_uppercase().contains(name) {
            return Ok(*opcode);
        }
    }

    Err("No opcode found.")
}
