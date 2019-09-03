use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Input {
    before: [usize; 4],
    op: [usize; 4],
    after: [usize; 4],
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum OperationType {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
    Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr
}

mod operations {
    pub fn addr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] + mem[op[2]];
        mem
    }

    pub fn addi(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] + op[2];
        mem
    }

    pub fn mulr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] * mem[op[2]];
        mem
    }

    pub fn muli(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] * op[2];
        mem
    }

    pub fn banr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] & mem[op[2]];
        mem
    }

    pub fn bani(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] & op[2];
        mem
    }

    pub fn borr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] | mem[op[2]];
        mem
    }

    pub fn bori(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]] | op[2];
        mem
    }

    pub fn setr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = mem[op[1]];
        mem
    }

    pub fn seti(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = op[1];
        mem
    }

    pub fn gtir(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = if op[1] > mem[op[2]] { 1 } else { 0 };
        mem
    }

    pub fn gtri(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = if mem[op[1]] > op[2] { 1 } else { 0 };
        mem
    }

    pub fn gtrr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = if mem[op[1]] > mem[op[2]] { 1 } else { 0 };
        mem
    }

    pub fn eqir(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = if op[1] == mem[op[2]] { 1 } else { 0 };
        mem
    }

    pub fn eqri(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = if mem[op[1]] == op[2] { 1 } else { 0 };
        mem
    }

    pub fn eqrr(mut mem: [usize; 4], op: [usize; 4]) -> [usize; 4] {
        mem[op[3]] = if mem[op[1]] == mem[op[2]] { 1 } else { 0 };
        mem
    }
}

type Operation = fn([usize; 4], [usize; 4]) -> [usize; 4];

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
        if get_operation(*op_type)(before, op) == after {
            Some(*op_type)
        } else {
            None
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

pub fn decode_operations(samples: &Vec<Input>) -> HashMap<usize, OperationType> {
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
        let (op_code, op_type) =
            compliance_list
            .iter()
            .filter_map(|(code, ops)| {
                match ops.len() {
                    1 => Some((*code, ops.clone().into_iter().next().unwrap())),
                    _ => None
                }
            })
            .next()
            .unwrap();

        decode_table.insert(op_code, op_type);

        compliance_list =
            compliance_list
            .into_iter()
            .filter_map(|(code, ops)| {
                if code == op_code {
                    None
                } else {
                    Some((code, ops.into_iter().filter(|op| *op != op_type).collect()))
                }
            })
            .collect();
    }

    decode_table
}

pub fn exec_test_seq(asm: &Vec<[usize; 4]>, decode_table: HashMap<usize, OperationType>)
    -> [usize; 4] {

    let mut memory = [0; 4];

    for instr in asm {
        memory = get_operation(*decode_table.get(&instr[0]).unwrap())(memory, *instr);
    }

    memory
}

pub fn parse_input(raw: &str) -> (Vec<Input>, Vec<[usize; 4]>) {
    let mut parts = raw.split("\r\n\r\n\r\n\r\n");
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

            Input { before, op, after }
        })
        .collect();

    let test_sequence: Vec<_> =
        parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut op = line.split(' ');
            [
                op.next().unwrap().parse::<usize>().unwrap(),
                op.next().unwrap().parse::<usize>().unwrap(),
                op.next().unwrap().parse::<usize>().unwrap(),
                op.next().unwrap().parse::<usize>().unwrap()
            ]
        })
        .collect();

    (inputs, test_sequence)
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

    let (inputs, test_seq) = parse_input(test_input);

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

    assert_eq!(inputs, expected_inputs);

    let expected_test_seq = vec![
        [2, 2, 3, 3],
        [2, 0, 3, 2],
        [2, 2, 1, 0],
    ];

    assert_eq!(test_seq, expected_test_seq);
}

#[test]
fn test_op_sample() {
    let before = [3, 2, 1, 1];
    let op = [9, 2, 1, 2];
    let after = [3, 2, 2, 1];

    let expected: HashSet<OperationType> = [
        OperationType::Addi,
        OperationType::Mulr,
        OperationType::Seti
    ].into_iter().cloned().collect();

    assert_eq!(get_compliant_ops(before, op, after), expected);
}
