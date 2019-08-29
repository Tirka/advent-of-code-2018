#[derive(Debug)]
pub struct TestSequence;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Input {
    before: [usize; 4],
    op: [usize; 4],
    after: [usize; 4],
}

pub fn test_samples(samples: Vec<Input>) -> usize {
    // println!("{:?}", &samples);

    // let asas = 
    //         samples.clone()
    //         .into_iter()
    //         .map(|sample| test_sample(sample.before, sample.op, sample.after))
    //         .count();
    
    // println!("{:?}", &asas);

    samples
    .into_iter()
    .map(|sample| test_sample(sample.before, sample.op, sample.after))
    .filter(|result| *result >= 3)
    .count()
}

fn test_sample(before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> usize {
    let operations = [
        addr, addi,
        mulr, muli,
        banr, bani,
        borr, bori,
        setr, seti,
        gtir, gtri, gtrr,
        eqir, eqri, eqrr
    ];

    operations
    .iter()
    .map(|o| o(before, op, after))
    .filter(|result| *result)
    .count()
}

fn addr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] + before[op[2]];
    before == after
}

fn addi(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] + op[2];
    before == after
}

fn mulr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] * before[op[2]];
    before == after
}

fn muli(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] * op[2];
    before == after
}

fn banr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] & before[op[2]];
    before == after
}

fn bani(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] & op[2];
    before == after
}

fn borr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] | before[op[2]];
    before == after
}

fn bori(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]] | op[2];
    before == after
}

fn setr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = before[op[1]];
    before == after
}

fn seti(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = op[1];
    before == after
}

fn gtir(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = if op[1] > before[op[2]] { 1 } else { 0 };
    before == after
}

fn gtri(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = if before[op[1]] > op[2] { 1 } else { 0 };
    before == after
}

fn gtrr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = if before[op[1]] > before[op[2]] { 1 } else { 0 };
    before == after
}

fn eqir(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = if op[1] == before[op[2]] { 1 } else { 0 };
    before == after
}

fn eqri(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = if before[op[1]] == op[2] { 1 } else { 0 };
    before == after
}

fn eqrr(mut before: [usize; 4], op: [usize; 4], after: [usize; 4]) -> bool {
    before[op[3]] = if before[op[1]] == before[op[2]] { 1 } else { 0 };
    before == after
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

    assert_eq!(test_sample(before, op, after), 3);
}