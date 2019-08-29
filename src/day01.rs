use std::collections::HashSet;

macro_rules! input_iterator {
    ($input: expr) => {
        $input
        .lines()
        .map(|num| num.parse::<i32>().unwrap())
    };
}

pub fn calculate_freq(input: &str) -> i32 {
    input_iterator!(input)
    .sum()
}

pub fn find_first_duplicate(input: &str) -> i32 {
    let iterator = 
        input_iterator!(input)
        .cycle()
        .scan(0, |accum, next| {
            *accum += next;
            Some(*accum)
        });

    let mut hash_set = HashSet::new();

    for r in iterator {
        if hash_set.get(&r).is_some() {
            return r
        } else {
            hash_set.insert(r);
        }
    }
    -1
}

#[test]
fn test_example() {

    let test_input = r"+1
-2
+3
+1";

    assert_eq!(calculate_freq(test_input), 3);
    assert_eq!(find_first_duplicate(test_input), 2);
}