use std::collections::HashMap;

pub fn parse_string(raw: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    let char_to_uint = |ch| if ch == '#' { 1 } else { 0 };

    let mut parts = raw.split("\n");
    
    let init_state: Vec<_> =
        parts
        .nth(0).unwrap()
        .split(' ')
        .nth(2).unwrap()
        .chars()
        .map(char_to_uint)
        .collect();

    let rules: HashMap<_, _> =
        parts
        .skip(1)
        .map(|rule| {
            let mut rule_parts = rule.split(" => ");

            let from =
                rule_parts
                .nth(0).unwrap()
                .chars()
                .map(char_to_uint)
                .fold(0, |acc, next| (acc << 1) + next);
            
            let to = char_to_uint(rule_parts.nth(0).unwrap().chars().nth(0).unwrap());
            
            (from, to)
        })
        .collect();
    (init_state, rules)
}

#[derive(Debug)]
struct Life {
    pots: Vec<u32>,
    rules: HashMap<u32, u32>
}
impl Life {
    pub fn new(initial_state: Vec<u32>, rules: HashMap<u32, u32>) -> Self {
        let pots = initial_state;

        Self { pots, rules }
    }

    pub fn new_generation(&mut self) {
        let mut new_pots: Vec<u32> = Vec::with_capacity(self.pots.len());
        for (index, _) in self.pots.iter().enumerate() {
            let mut key = 0;
            for n in index-2..=index+2 {
                key = (key << 1) + self.pots.get(n).unwrap_or(&0);
            }
            new_pots.push(*self.rules.get(&key).unwrap_or(&0));
        }
        self.pots = new_pots;
    }

    pub fn observe(&self) -> String {
        self.pots
        .iter()
        .map(|pot| if *pot == 1 { '#' } else { '.' })
        .collect()
    }
}

#[test]
fn test_parsing() {
    
    let test_data = r"initial state: ###.......##....#.#.#..###.##..##.....##

..... => .
#..## => .
..### => #
..#.# => #
.#.#. => .
####. => .";

    let expected_init_state = vec![
        1, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 1, 1, 0, 0, 0, 0,
        1, 0, 1, 0, 1, 0, 0, 1,
        1, 1, 0, 1, 1, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 1, 1
    ];

    let expected_rules: HashMap<u32, u32> = vec![
        ( 0, 0 ),
        ( 19, 0 ),
        ( 7, 1  ),
        ( 5,  1  ),
        ( 10, 0 ),
        ( 30, 0 ),
    ].into_iter().collect();

    let (initial_state, rules) = parse_string(test_data);

    assert_eq!(initial_state, expected_init_state);
    assert_eq!(rules, expected_rules);
}

#[test]
fn test_life() {
    let test_data = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    let (initial_state, rules) = parse_string(test_data);

    let _life = Life::new(initial_state, rules);
}