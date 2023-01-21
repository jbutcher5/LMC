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
    Hlt
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
    pc: u8,
    acc: u8,
    ram: Vec<u16>,
}

impl Interpreter {
    fn execute(&self) {
        println!("{}", self.pc);

    }
}

fn main() {
    use Instr::*;
    let mut interp = Interpreter {pc: 0, acc: 0, ram:vec![0;99]};
    interp.execute();

    let ram: Vec<u16> = vec![Add(1), Sub(2), Sta(6), Lda(3), Bra(4), Brz(5),  Brp(7), Out, Inp, Hlt].into_iter().map(Instr::encode).collect();

    interp.ram = ram;

}