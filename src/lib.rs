pub mod eval;
pub mod parser;

#[derive(Debug, Copy, Clone)]
pub enum LMCError {
    NotEnoughRAM,
    InvalidInstruction,
}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    Add(i32),
    Sub(i32),
    Lda(i32),
    Sta(i32),
    Brp(i32),
    Brz(i32),
    Bra(i32),
    Dat(i32),
    Inp,
    Out,
    Hlt,
}
