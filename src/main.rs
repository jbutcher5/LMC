#[derive(Debug, Copy, Clone)]
enum LMCError {
    NotEnoughRAM,
    InvalidInstruction,
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Add(u16),
    Sub(u16),
    Lda(u16),
    Sta(u16),
    Brp(u16),
    Brz(u16),
    Bra(u16),
    Dat(u16),
    Inp,
    Out,
    Hlt,
}

impl Instr {
    fn encode(self) -> u16 {
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

struct Interpreter {
    pc: usize,
    acc: u16,
    ram: Vec<u16>,
}

impl Interpreter {
    fn decode(&mut self, register: usize) -> Result<u16, LMCError> {
        self.ram
            .get(register)
            .map_or_else(|| Err(LMCError::NotEnoughRAM), |value| Ok(*value))
    }

    fn decode_map(
        &mut self,
        register: usize,
        f: impl FnOnce(&mut u16) -> Result<(), LMCError>,
    ) -> Result<(), LMCError> {
        self.ram
            .get_mut(register)
            .map_or_else(|| Err(LMCError::NotEnoughRAM), f)
    }

    fn execute(&mut self, reserved_memory: Vec<usize>) -> Result<(), LMCError> {
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
            self.decode_map(operand.into(), |x| {
                *x += acc;
                Ok(())
            })?;
        } else if operator == 2 {
            let acc = self.acc;
            self.decode_map(operand.into(), |x| {
                *x -= acc;
                Ok(())
            })?;
        } else if operator == 3 {
            let acc = self.acc;
            self.decode_map(operand.into(), |x| {
                *x = acc;
                Ok(())
            })?;
        } else if operator == 4 {
            self.acc = self.decode(operand.into())?;
        } else if operator == 0 {
            return Ok(());
        }

        self.execute(reserved_memory)
    }
}

fn main() {
    use Instr::*;

    let tokens = vec![Add(1), Dat(12), Hlt];

    let encoded = tokens.clone().into_iter().map(Instr::encode).collect();

    let reserved_memory: Vec<usize> =
        tokens
            .into_iter()
            .enumerate()
            .fold(vec![], |mut acc, (index, x)| {
                if let Dat(_) = x {
                    acc.push(index);
                }
                acc
            });

    let mut interp = Interpreter {
        pc: 0,
        acc: 2,
        ram: encoded,
    };

    interp.execute(reserved_memory).unwrap();
}
