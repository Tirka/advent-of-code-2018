use std::collections::HashMap;

struct Id {
    two: bool,
    three: bool
}
impl Id {
    fn parse_str(raw_str: &str) -> Self {
        let mut hm = HashMap::new();
        for ch in raw_str.chars() {
            if let Some(amount) = hm.get_mut(&ch) {
                *amount += 1;
            } else {
                hm.insert(ch, 1);
            }
        }
        let two = hm.values().any(|val| *val == 2);
        let three = hm.values().any(|val| *val == 3);

        Self { two, three }
    }
}

pub fn calculate_checksum(input: &str) -> usize {
    let ids: Vec<Id> =
        input
        .lines()
        .map(Id::parse_str)
        .collect();
    let twos = ids.iter().filter(|id| id.two).count();
    let threes = ids.iter().filter(|id| id.three).count();
    twos * threes
}

pub fn find_correct_box<'a>(input: &str) -> String {
    let ids: Vec<_> = input.lines().collect();
    for (i, str1) in ids.iter().enumerate() {
        for str2 in ids.iter().skip(i+1) {
            let mut differences_amount = 0;
            let mut possible_result = Vec::new();
            for (ch1, ch2) in str1.chars().zip(str2.chars()) {
                if ch1 != ch2 { differences_amount += 1 }
                else { possible_result.push(ch1) }
            }
            if differences_amount == 1 {
                return possible_result.iter().collect();
            }
        }
    }
    return String::new();
}

#[test]
fn test_example_part_1() {
    let test_data = 
r"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    assert_eq!(calculate_checksum(test_data), 12)
}

#[test]
fn test_example_part_2() {
    let test_data = 
r"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    assert_eq!(find_correct_box(test_data), "fgij")
}