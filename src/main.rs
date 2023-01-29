use lmc::*;

fn main() {
    use Instr::*;

    let tokens = vec![Inp, Sta(2), Dat(0), Out, Hlt];

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
        acc: 0,
        ram: encoded,
    };

    interp.execute(reserved_memory).unwrap();

    println!("RAM: {:?}", interp.ram);
}
