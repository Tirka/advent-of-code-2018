use std::collections::{HashMap, HashSet};
// use itertools::Itertools;
// use std::iter::FromIterator;

#[derive(Debug)]
pub struct TestSequence;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Input {
    before: [usize; 4],
    op: [usize; 4],
    after: [usize; 4],
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum OperationType {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

mod operations {
    pub fn addr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] + before[op[2]];
        before == after
    }

    pub fn addi(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] + op[2];
        before == after
    }

    pub fn mulr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] * before[op[2]];
        before == after
    }

    pub fn muli(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] * op[2];
        before == after
    }

    pub fn banr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] & before[op[2]];
        before == after
    }

    pub fn bani(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] & op[2];
        before == after
    }

    pub fn borr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] | before[op[2]];
        before == after
    }

    pub fn bori(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]] | op[2];
        before == after
    }

    pub fn setr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = before[op[1]];
        before == after
    }

    pub fn seti(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = op[1];
        before == after
    }

    pub fn gtir(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = if op[1] > before[op[2]] { 1 } else { 0 };
        before == after
    }

    pub fn gtri(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = if before[op[1]] > op[2] { 1 } else { 0 };
        before == after
    }

    pub fn gtrr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = if before[op[1]] > before[op[2]] { 1 } else { 0 };
        before == after
    }

    pub fn eqir(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = if op[1] == before[op[2]] { 1 } else { 0 };
        before == after
    }

    pub fn eqri(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = if before[op[1]] == op[2] { 1 } else { 0 };
        before == after
    }

    pub fn eqrr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
        before[op[3]] = if before[op[1]] == before[op[2]] { 1 } else { 0 };
        before == after
    }
}

type Operation = fn([usize; 4], [usize; 4], [usize; 4]) -> bool;

fn get_operation(op_type: OperationType) -> Operation {
    match op_type {
        OperationType::Addr => operations::addr,
        OperationType::Addi => operations::addi,
        OperationType::Mulr => operations::mulr,
        OperationType::Muli => operations::muli,
        OperationType::Banr => operations::banr,
        OperationType::Bani => operations::bani,
        OperationType::Borr => operations::borr,
        OperationType::Bori => operations::bori,
        OperationType::Setr => operations::setr,
        OperationType::Seti => operations::seti,
        OperationType::Gtir => operations::gtir,
        OperationType::Gtri => operations::gtri,
        OperationType::Gtrr => operations::gtrr,
        OperationType::Eqir => operations::eqir,
        OperationType::Eqri => operations::eqri,
        OperationType::Eqrr => operations::eqrr,
    }
}

fn get_compliant_ops(before: [usize; 4], op: [usize; 4], after: [usize; 4])
    -> HashSet<OperationType> {

    let operations = [
        OperationType::Addr, OperationType::Addi,
        OperationType::Mulr, OperationType::Muli,
        OperationType::Banr, OperationType::Bani,
        OperationType::Borr, OperationType::Bori,
        OperationType::Setr, OperationType::Seti,
        OperationType::Gtir, OperationType::Gtri, OperationType::Gtrr,
        OperationType::Eqir, OperationType::Eqri, OperationType::Eqrr,
    ];

    operations
    .iter()
    .filter_map(|op_type| {
        match get_operation(*op_type)(before, op, after) {
            true => Some(*op_type),
            false => None
        }
    })
    .collect()
}

pub fn count_three_or_more_compliant(samples: &Vec<Input>) -> usize {
    samples
    .iter()
    .map(|sample| get_compliant_ops(sample.before, sample.op, sample.after))
    .filter(|result| result.len() >= 3)
    .count()
}

fn decode_operations(samples: &Vec<Input>) -> HashMap<usize, OperationType> {
    let mut compliance_list: HashMap<_,_> =
        samples
        .iter()
        .map(|s| (s.op[0], get_compliant_ops(s.before, s.op, s.after)))
        .fold(HashMap::new(), |mut acc, (op_code, maybes)| {
            match acc.get_mut(&op_code) {
                None => { acc.insert(op_code, maybes); }
                Some(hs) => { *hs = hs.intersection(&maybes).cloned().collect(); }
            }
            acc
        });
    
    let mut decode_table: HashMap<usize, OperationType> = HashMap::new();
    
    while !compliance_list.is_empty() {
        let (op_code, op_type) = compliance_list.iter().filter_map(|(code, ops)| {
            match ops.len() {
                1 => Some((*code, ops.clone().into_iter().next().unwrap())),
                _ => None
            }
        }).next().unwrap();

        decode_table.insert(op_code, op_type);

        compliance_list = compliance_list.into_iter().filter_map(|(code, ops)| {
            if code == op_code {
                None
            } else {
                Some((code, ops.into_iter().filter(|op| *op != op_type).collect()))
            }
        }).collect();
    }
    

    println!("{:?}", &decode_table);

    decode_table
}

fn exec_test_sequence(asm: &Vec<[usize; 4]>) {
    let mut memory = [0; 4];
    
    for instr in asm {
        
    }
}

pub fn parse_input(raw: &str) -> (Vec<Input>, TestSequence) {
    let mut parts = raw.split("\r\n\r\n\r\n");
    let inputs: Vec<_> = 
        parts
        .next()
        .unwrap()
        .split("\r\n\r\n")
        .map(|inp| {
            let mut parts = inp.split("\r\n");
            let before: Vec<_> = 
                parts
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ',' || c == '[' || c == ']')
                .collect();
            let before = [
                before[2].parse::<usize>().unwrap(),
                before[4].parse::<usize>().unwrap(),
                before[6].parse::<usize>().unwrap(),
                before[8].parse::<usize>().unwrap()
            ];
            let op: Vec<_> =
                parts
                .next()
                .unwrap()
                .split(' ')
                .collect();
            let op = [
                op[0].parse::<usize>().unwrap(),
                op[1].parse::<usize>().unwrap(),
                op[2].parse::<usize>().unwrap(),
                op[3].trim().parse::<usize>().unwrap(),
            ];
            let after: Vec<_> = 
                parts
                .next()
                .unwrap()
                .split(|c| c == ' ' || c == ',' || c == '[' || c == ']')
                .collect();
            let after = [
                after[3].parse::<usize>().unwrap(),
                after[5].parse::<usize>().unwrap(),
                after[7].parse::<usize>().unwrap(),
                after[9].parse::<usize>().unwrap()
            ];

            Input {
                before, op, after
            }
        })
        .collect();
    
    // let _test_sequence = parts.next().unwrap();

    (inputs, TestSequence)
}

#[test]
fn test_operation_decoder() {
    let content =
        std::fs::read_to_string(r"D:\Git\advent2018\puzzle_inputs\day16.txt")
        .unwrap();

    let (inp, seq) = parse_input(&content);

    let _resultando = decode_operations(&inp);

}

// fix line endings
// #[test]
fn test_op_parsing() {
    let test_input = r"Before: [1, 1, 0, 3]
3 0 2 0
After:  [0, 1, 0, 3]

Before: [0, 1, 2, 3]
12 1 2 3
After:  [0, 1, 2, 0]



2 2 3 3
2 0 3 2
2 2 1 0";

    let (inputs, _test_sequence) = parse_input(test_input);

    let expected_inputs = vec![
        Input {
            before: [1, 1, 0, 3],
            op: [3, 0, 2, 0],
            after: [0, 1, 0, 3]
        },
        Input {
            before: [0, 1, 2, 3],
            op: [12, 1, 2, 3],
            after: [0, 1, 2, 0]
        }
    ];

    assert_eq!(inputs, expected_inputs)
}

#[test]
fn test_op_sample() {
    let before = [3, 2, 1, 1];
    let op = [9, 2, 1, 2];
    let after = [3, 2, 2, 1];

    let mut expected = HashSet::new();
    expected.insert(OperationType::Addi);
    expected.insert(OperationType::Mulr);
    expected.insert(OperationType::Seti);

    assert_eq!(get_compliant_ops(before, op, after), expected);
}