macro_rules! opcodes {
  { $($variant: ident = $value: expr),+$(,)? } =>
    {
        #[repr(u8)]
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub enum Opcode {
            $($variant = $value),+,
        }
        pub const OPCODES: &'static [Opcode] = &[$(Opcode::$variant),*];
        // pub const OPCODES_MAP: &'static [ (String, Opcode) ] = &[ (format!("{}", $("Opcode::$variant")), $(Opcode::$variant)),* ];
    }
}

opcodes! {
    // Arithmetical instructions
    ADD = 1,
    SUB = 2,
    MUL = 3,
    MOD = 4,
    INC = 5,
    DEC = 6,
    // Control Flow instructions
    EQ = 7,
    NEQ = 8,
    GT = 9,
    GTE = 10,
    LT = 11,
    LTE = 12,
    HLT = 13,
    NOP = 14,
    LOAD = 15,
    JMP = 16,
    // Takes the last modulo's remaining and put it into the specified register
    MODR = 17,
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
    for opcode in OPCODES {
        let dbg = format!("{:?}", opcode);
        if dbg.contains(&base.to_uppercase()) {
            return Ok(*opcode);
        }
    }
    Err("No opcode found.")
}
