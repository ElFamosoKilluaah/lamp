use super::vm::VMError;
use lamp_common::op::{decode_opcode as get_op, Opcode};

pub fn decode_opcode(val: u8) -> Result<Opcode, VMError> {
    match get_op(val) {
        Some(opcode) => Ok(opcode),
        None => Err(VMError::InvalidOpcodeError),
    }
}
