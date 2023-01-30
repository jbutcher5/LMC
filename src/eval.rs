use crate::{Instr, LMCError};

use std::{io, io::Write};

impl Instr {
    pub fn encode(self) -> i32 {
        use Instr::*;
        match self {
            Add(x) => 100 + x,
            Sub(x) => 200 + x,
            Sta(x) => 300 + x,
            Lda(x) => 500 + x,
            Bra(x) => 600 + x,
            Brz(x) => 700 + x,
            Brp(x) => 800 + x,
            Inp => 901,
            Out => 902,
            Hlt => 0,
            Dat(x) => x,
        }
    }
}

pub struct Interpreter {
    pc: usize,
    acc: i32,
    pub ram: Vec<i32>,
}

impl Interpreter {
    pub fn new(mut ram: Vec<i32>, ram_size: usize) -> Result<Self, LMCError> {
        if ram.len() > ram_size {
            Err(LMCError::NotEnoughRAM)
        } else {
            ram.resize(ram_size, 0);

            Ok(Self { pc: 0, acc: 0, ram })
        }
    }

    pub fn decode(&mut self, register: usize) -> Result<i32, LMCError> {
        self.ram
            .get(register)
            .map_or_else(|| Err(LMCError::NotEnoughRAM), |value| Ok(*value))
    }

    pub const fn able_to_branch(&self, operator: i32) -> bool {
        return (operator == 6)
            || (operator == 7 && self.acc == 0)
            || (operator == 8 && self.acc >= 0);
    }

    pub fn decode_map(
        &mut self,
        register: usize,
        f: impl FnOnce(&mut i32) -> Result<(), LMCError>,
    ) -> Result<(), LMCError> {
        self.ram
            .get_mut(register)
            .map_or_else(|| Err(LMCError::NotEnoughRAM), f)
    }

    pub fn execute(&mut self, reserved_memory: Vec<usize>) -> Result<(), LMCError> {
        use LMCError::*;

        let cir = self.decode(self.pc)?;

        if cir >= 999 {
            return Err(InvalidInstruction);
        };

        let (operator, operand) = self
            .decode(self.pc)
            .map(|instr| (instr / 100, instr % 100))?;

        self.pc += 1 + reserved_memory.contains(&(self.pc + 1)) as usize;

        if operator == 1 {
            let acc = self.acc;
            self.decode_map(operand as usize, |x| {
                *x += acc;
                Ok(())
            })?;
        } else if operator == 2 {
            let acc = self.acc;
            self.decode_map(operand as usize, |x| {
                *x -= acc;
                Ok(())
            })?;
        } else if operator == 3 {
            let acc = self.acc;
            self.decode_map(operand as usize, |x| {
                *x = acc;
                Ok(())
            })?;
        } else if operator == 4 {
            self.acc = self.decode(operand as usize)?;
        } else if self.able_to_branch(operator) {
            self.pc = operator as usize;
        } else if cir == 901 {
            fn user_input() -> i32 {
                let mut buffer: String = String::new();
                print!("> ");
                std::io::stdout()
                    .flush()
                    .expect("[ERROR] Failed to flush the buffer");
                if io::stdin().read_line(&mut buffer).is_ok() {
                    if let Ok(x) = buffer.trim().parse::<i32>() {
                        x
                    } else {
                        println!("[ERROR] Failed to convert input to 32 bit signed int");
                        user_input()
                    }
                } else {
                    println!("[ERROR] Failed to get input");
                    user_input()
                }
            }

            self.acc = user_input();
        } else if cir == 902 {
            println!("{}", self.acc);
        } else if operator == 0 {
            return Ok(());
        }

        self.execute(reserved_memory)
    }
}
