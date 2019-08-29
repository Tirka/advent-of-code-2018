use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Rule {
    step: char,
    prerequisite: char
}
impl Rule {
    pub fn parse_raw(raw: &str) -> Self {
        let parts: Vec<_> = raw.split(' ').collect();
        Rule {
            step: parts[7].parse().unwrap(),
            prerequisite: parts[1].parse().unwrap()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instructions(HashMap<char, HashSet<char>>);
impl Instructions {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_rule(&mut self, rule: Rule) {
        match self.0.get_mut(&rule.step) {
            Some(set) => {
                set.insert(rule.prerequisite);
            },
            None => {
                let mut set = HashSet::new();
                set.insert(rule.prerequisite);
                self.0.insert(rule.step, set);
            }
        }

        if let None = self.0.get_mut(&rule.prerequisite) {
            self.0.insert(rule.prerequisite, HashSet::new());
        }
    }
}
impl Iterator for Instructions {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for c in (b'A'..=b'Z').map(char::from) {
            if let Some(set) = self.0.get_mut(&c) {
                if set.is_empty() {
                    for val in self.0.values_mut() {
                        val.remove(&c);
                    }
                    self.0.remove(&c);
                    return Some(c);
                }
            } 
        }
        None
    }
}

#[derive(Debug)]
struct Runtime {
    instructions: Instructions,
    workers: Vec<i32>,
    time_spent: u16
}
impl Runtime {
    pub fn new(instructions: Instructions, workers_amount: usize) -> Self {
        let workers = std::iter::repeat(0).take(workers_amount).collect();
        let time_spent = 0;
        Self {
            instructions, workers, time_spent
        }
    }

    pub fn process(&mut self) -> u16 {
        loop {
            println!("{:?}\n", &self);
            let more_available = self.load_available_workers();
            let more_ticks_required = self.tick();
            if !more_available && !more_ticks_required { break; }
        }
        self.time_spent
    }

    // emulates quant of time, returns true if
    // one or more workers still have work to do
    fn tick(&mut self) -> bool {
        for worker in &mut self.workers {
            *worker -= 1;
        }
        self.time_spent += 1;
        self.workers.iter().any(|w| *w > 0)
    }

    fn seconds_required(&self, ch: char) -> i32 {
        (ch as u8 - b'A' + 1) as i32
    }

    // fills all vacant workers with next task,
    // returns true if more tasks in queue still awaiting
    fn load_available_workers(&mut self) -> bool { // more_tasks_available
        let mut is_more_tasks_in_queue = true;
        for i in 0..self.workers.iter().filter(|w| **w <= 0).count() {
            match self.instructions.next() {
                Some(ch) => self.workers[i] = self.seconds_required(ch),
                None => is_more_tasks_in_queue = false
            }
        }
        is_more_tasks_in_queue
    }
}

#[test]
fn test_parsing() {
    let rule = Rule::parse_raw("Step A must be finished before step D can begin.");
    assert_eq!(rule, Rule { step: 'D', prerequisite: 'A' })
}

#[test]
fn test_sequence_processing() {
    let test_data = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    let mut instructions = Instructions::new();
    for rule in test_data.lines().map(Rule::parse_raw) {
        instructions.add_rule(rule);
    }

    let mut runtime = Runtime::new(instructions.clone(), 2);

    let order: String = instructions.collect();
    assert_eq!(order, "CABDFE");

    assert_eq!(runtime.process(), 15);
}