use super::opcodes::decode_opcode;
use lamp_common::op::Opcode;
use log::{error, info, warn};

pub struct VM {
    // The binary VM has to execute
    bin: Vec<u8>,
    // The program counter, it's utility is to remind where we are in the program
    pc: usize,
    // Registers used to store i32 values the program needs
    registers: [i32; 32],
    // When modulo operation is done, the remainder is pushed here
    modulo_remainder: i32,
    // When an eq test is done, the result is pushed here
    eq_flag: bool,
}

pub type VMResult = Result<i32, VMError>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VMError {
    InvalidOpcodeError,
    UnexpectedTokenError,
    // More errors will be added
}

impl VM {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            bin: binary,
            pc: 0,
            registers: [0; 32],
            modulo_remainder: 0,
            eq_flag: false,
        }
    }

    // Runs the full binary.
    pub fn run(&mut self) -> VMResult {
        info!("Running a lamp VM. Total binary size: {}", self.bin.len());
        let mut result;
        loop {
            result = self.cycle();

            if self.pc >= self.bin.len() {
                warn!(
                    "Program Counter greater than binary's length. Consider it like a finished job.
                    \nProgram counter: {}\n
                    Binary's length: {}",
                    self.pc,
                    self.bin.len()
                );
                break;
            }
        }
        info!("VM Stopped. Exit code: {:?}", result);
        result
    }

    // One VM's cycle.
    // It does:
    // -The opcode decoding
    // -The opcode execution
    // - Error handling
    pub fn cycle(&mut self) -> VMResult {
        let opcode = self.next_8_bits();
        let outcoming_result: VMResult;

        match decode_opcode(opcode) {
            Ok(opcode) => outcoming_result = self.execute_instruction(opcode),
            Err(e) => {
                error!("VM's error happened. Aborting. \n {:?}", e);
                return Err(e);
            }
        }
        outcoming_result
    }

    // Executes the given opcode.
    // This function is just a giant match.
    pub fn execute_instruction(&mut self, opcode: Opcode) -> VMResult {
        match opcode {
            // Arithmetical instructions
            Opcode::ADD => {
                let val_1 = self.registers[self.next_8_bits() as usize];
                let val_2 = self.registers[self.next_8_bits() as usize];
                let result_register = self.next_8_bits();
                self.set_register_value(result_register, val_1 + val_2);
            }
            Opcode::SUB => {
                let val_1 = self.registers[self.next_8_bits() as usize];
                let val_2 = self.registers[self.next_8_bits() as usize];
                let result_register = self.next_8_bits();
                self.set_register_value(result_register, val_1 - val_2);
            }
            Opcode::MUL => {
                let val_1 = self.registers[self.next_8_bits() as usize];
                let val_2 = self.registers[self.next_8_bits() as usize];
                let result_register = self.next_8_bits();
                self.set_register_value(result_register, val_1 * val_2);
            }
            Opcode::MOD => {
                let val_1 = self.registers[self.next_8_bits() as usize];
                let val_2 = self.registers[self.next_8_bits() as usize];
                let result_register = self.next_8_bits();
                self.set_register_value(result_register, val_1 / val_2);
                self.modulo_remainder = val_1 % val_2;
            }
            Opcode::INC => {
                let to_inc = self.next_8_bits() as usize;
                self.registers[to_inc] += 1;
            }
            Opcode::DEC => {
                let to_dec = self.next_8_bits() as usize;
                self.registers[to_dec] -= 1;
            }
            // Control Flow instructions
            Opcode::EQ => {
                let reg_1 = self.next_8_bits() as usize;
                let reg_2 = self.next_8_bits() as usize;
                self.eq_flag = self.registers[reg_1] == self.registers[reg_2];
            }
            Opcode::NEQ => {
                let reg_1 = self.next_8_bits() as usize;
                let reg_2 = self.next_8_bits() as usize;
                self.eq_flag = self.registers[reg_1] != self.registers[reg_2];
            }
            Opcode::GT => {
                let reg_1 = self.next_8_bits() as usize;
                let reg_2 = self.next_8_bits() as usize;
                self.eq_flag = self.registers[reg_1] > self.registers[reg_2];
            }
            Opcode::GTE => {
                let reg_1 = self.next_8_bits() as usize;
                let reg_2 = self.next_8_bits() as usize;
                self.eq_flag = self.registers[reg_1] >= self.registers[reg_2];
            }
            Opcode::LT => {
                let reg_1 = self.next_8_bits() as usize;
                let reg_2 = self.next_8_bits() as usize;
                self.eq_flag = self.registers[reg_1] < self.registers[reg_2];
            }
            Opcode::LTE => {
                let reg_1 = self.next_8_bits() as usize;
                let reg_2 = self.next_8_bits() as usize;
                self.eq_flag = self.registers[reg_1] <= self.registers[reg_2];
            }
            Opcode::HLT => {
                // I literally don't know what should I do here
            }
            Opcode::NOP => {
                info!("NOP Opcode encountered, doing nothing.");
                self.pc += 1;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits();
                let value = self.next_16_bits() as i32;
                self.set_register_value(register, value);
            }
            Opcode::JMP => {
                let register = self.next_8_bits();
                let addr = *self.get_register(register);
                self.pc = addr as usize;
            }

            Opcode::MODR => {
                let register = self.next_8_bits();
                self.set_register_value(register, self.modulo_remainder);
            }
        }
        Ok(0)
    }

    // Sets the value of a register.
    pub fn set_register_value(&mut self, index: u8, val: i32) {
        let ptr = self.get_register_mut(index);
        *ptr = val;
    }

    // Gives a mutable register's reference, and verifies if the given register's index is valid.
    pub fn get_register_mut(&mut self, index: u8) -> &mut i32 {
        if index < 32 {
            &mut self.registers[index as usize]
        } else {
            error!(
                "Register out of bounds. Expected 0 <= register_index < 32; got {}",
                index
            );
            panic!(
                "Register out of bounds. Expected 0 <= register_index < 32; got {}",
                index
            );
        }
    }

    pub fn get_register(&self, index: u8) -> &i32 {
        if index < 32 {
            &self.registers[index as usize]
        } else {
            error!(
                "Register out of bounds. Expected 0 <= register_index < 32; got {}",
                index
            );
            panic!(
                "Register out of bounds. Expected 0 <= register_index < 32; got {}",
                index
            );
        }
    }
    // Grabs next 8 bits of the VM's binary
    pub fn next_8_bits(&mut self) -> u8 {
        self.pc += 1;
        self.bin[self.pc - 1]
    }

    // Grabs next 16 bytes of the VM's binary
    fn next_16_bits(&mut self) -> u16 {
        self.pc += 2;
        ((u16::from(self.bin[self.pc - 2])) << 8) | u16::from(self.bin[self.pc - 1])
    }

    pub fn set_pc(&mut self, new_pc: usize) -> usize {
        if new_pc > self.bin.len() {
            self.pc = new_pc;
            0
        } else {
            error!(
                "Tried to set the program counter to {} where the binary\'s total size is {} bytes",
                new_pc,
                self.bin.len()
            );
            1
        }
    }
}
