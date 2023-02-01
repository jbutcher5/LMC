use lmc::{eval::*, parser::parser, *};

fn main() {
    use Instr::*;

    let tokens = parser(
        "
INP
STA x
x DAT 5
        "
        .to_string(),
    )
    .unwrap();
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

    let mut interp = Interpreter::new(encoded, 20).unwrap();
    interp.execute(reserved_memory).unwrap();

    println!("RAM: {:?}", interp.ram);
}
