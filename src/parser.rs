use crate::{Instr, Instr::*};
use std::collections::HashMap;

pub fn parser(buffer: String) -> Option<Vec<Instr>> {
    let mut jump_table: HashMap<String, usize> = HashMap::new();
    let mut pc: usize = 0;

    let (result, _, _) = parser2(buffer, jump_table, pc)?;
    Some(result)
}

fn parser2(
    buffer: String,
    mut jump_table: HashMap<String, usize>,
    mut pc: usize,
) -> Option<(Vec<Instr>, HashMap<String, usize>, usize)> {
    if buffer.contains('\n') {
        let mut new = vec![];

        for line in buffer.split('\n') {
            let (instr, _jump_table, _pc) = parser2(line.to_string(), jump_table.clone(), pc)?;

            jump_table = _jump_table;
            pc = _pc;
            new.push(instr);
        }

        Some((
            new.into_iter().flatten().collect::<Vec<Instr>>(),
            jump_table.clone(),
            pc,
        ))
    } else if buffer.is_empty() {
        Some((vec![], jump_table.clone(), pc))
    } else {
        let split_buffer: Vec<&str> = buffer.split(' ').collect();

        if split_buffer.len() == 2 {
            pc += 1;
            Some((
                match (split_buffer[0], split_buffer[1]) {
                    ("add", x) => {
                        vec![Add(x.parse::<i32>().unwrap())]
                    }
                    _ => unimplemented!(),
                },
                jump_table.clone(),
                pc,
            ))
        } else if split_buffer.len() == 3 {
            None
        } else {
            None
        }
    }
}
