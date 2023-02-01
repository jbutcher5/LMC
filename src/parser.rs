const INSTRUCTIONS: [&str; 11] = [
    "ADD", "SUB", "LDA", "STA", "BRA", "BRP", "BRZ", "DAT", "HLT", "OUT", "INP",
];

use crate::{Instr, Instr::*};
use std::collections::HashMap;

pub fn parser(buffer: String) -> Option<Vec<Instr>> {
    let mut jump_table: HashMap<String, usize> = HashMap::new();
    let mut pc: usize = 0;
    let mut strip_labels = String::new();

    for line in buffer.split('\n') {
        if line.trim() == "" {
            continue;
        }

        let mut split_line: Vec<&str> = line.trim().split(' ').collect();

        if !INSTRUCTIONS.contains(split_line.first()?) {
            jump_table.insert(split_line.first()?.to_string(), pc);
            split_line.remove(0);
        }

        strip_labels += "\n";
        strip_labels += &split_line.join(" ");
        pc += 1;
    }

    strip_labels = strip_labels.trim().to_string();

    let (result, _) = parser2(strip_labels, &jump_table, 0);
    result
}

fn parser2(
    buffer: String,
    jump_table: &HashMap<String, usize>,
    mut pc: usize,
) -> (Option<Vec<Instr>>, usize) {
    if buffer.contains('\n') {
        let mut new = vec![];

        for line in buffer.split('\n') {
            if line.trim().is_empty() {
                continue;
            }

            let (instr, _pc) = parser2(line.to_string(), jump_table, pc);

            if let Some(instr) = instr {
                pc = _pc;
                new.push(instr);
            } else {
                return (None, _pc);
            }
        }

        (Some(new.into_iter().flatten().collect::<Vec<Instr>>()), pc)
    } else {
        let split_buffer: Vec<&str> = buffer.split(' ').collect();

        if split_buffer.len() == 2 {
            let operator = split_buffer[0];
            let operand = if let Ok(value) = split_buffer[1].parse::<i32>() {
                value
            } else if let Some(value) = jump_table.get(split_buffer[1]) {
                *value as i32
            } else {
                return (None, pc + 1);
            };

            (
                Some(vec![match (operator, operand) {
                    ("ADD", x) => Add(x),
                    ("SUB", x) => Sub(x),
                    ("LDA", x) => Lda(x),
                    ("STA", x) => Sta(x),
                    ("BRA", x) => Bra(x),
                    ("BRZ", x) => Brz(x),
                    ("BRP", x) => Brp(x),
                    ("DAT", x) => Dat(x),
                    _ => unimplemented!(),
                }]),
                pc + 1,
            )
        } else if split_buffer.len() == 1 {
            let instr: Option<Vec<Instr>> = match split_buffer[0] {
                "DAT" => Some(vec![Dat(0)]),
                "HLT" => Some(vec![Hlt]),
                "INP" => Some(vec![Inp]),
                "OUT" => Some(vec![Out]),
                _ => None,
            };

            (instr, pc + 1)
        } else {
            return (None, pc + 1);
        }
    }
}
