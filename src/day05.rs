use std::iter::FromIterator;

pub fn reduce_and_measure(input: &str) -> usize {
    let mut polymer = Vec::from_iter(input.chars());
    let mut i = 0;
    let mut j = 1;
    let polymer_len = polymer.len();
    
    while j < polymer_len {
        let char_left = polymer[i];
        let char_right = polymer[j];
        
        if  char_left != char_right &&
            char_left.to_ascii_lowercase() == char_right.to_ascii_lowercase()
        {
            polymer[i] = '_';
            polymer[j] = '_';
            j += 1;
            while polymer[i] == '_' && i > 0 {
                i -= 1;
            }
        } else {
            i = j;
            j += 1;
        }
    }

    polymer.into_iter().filter(|ch| *ch != '_').count()
}

pub fn cut_reduce_and_measure(input: &str) -> usize {
    let mut min_length = std::usize::MAX;
    for letter in (b'a'..=b'z').map(char::from) {
        let new_polymer: String =
            input
            .chars()
            .filter(|ch|
                *ch != letter &&
                *ch != letter.to_ascii_uppercase()
            )
            .collect();
        let length = reduce_and_measure(&new_polymer);
        if length < min_length { min_length = length; }
    }
    min_length
}

#[test]
fn test_example() {
    let test_data = "dabAcCaCBAcCcaDA";

    let polymer_length = reduce_and_measure(test_data);
    assert_eq!(polymer_length, 10);

    let min_polymer_length = cut_reduce_and_measure(test_data);
    assert_eq!(min_polymer_length, 4);
}
