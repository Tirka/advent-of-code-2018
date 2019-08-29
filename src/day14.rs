#[derive(Debug)]
pub struct Kitchen {
    board: Vec<i32>,
    elves: Vec<i32>
}
impl Kitchen {
    pub fn new(start_board: Vec<i32>, elves: i32) -> Self {
        let elves = (0..elves).collect();
        Self { board: start_board, elves }
    }

    pub fn score(&mut self, input: &str) -> String {
        let recipes = input.parse().unwrap();
        while self.board.len() < recipes + 10 {
            self.step();
        }
        self.board
            .iter()
            .skip(recipes)
            .take(10)
            .map(|num| num.to_string())
            .collect()
    }

    pub fn appeared_first(mut self, score: &str) -> usize {
        let score: Vec<_> =
            score.chars().map(|ch| ch.to_digit(10).unwrap() as i32).collect();
        loop {
            self.step();
            if self.board.len() > score.len() {
                for i in 0..2 {
                    let from = self.board.len()-score.len()-i;
                    let to = self.board.len()-i;
                    if &self.board[from..to] == &score[..] {
                        return self.board.len() - score.len() - i;
                    }
                }
            }
        }
    }

    fn step(&mut self) {
        self.add_new_recipes();
        self.set_currect_recipes();
    }

    fn add_new_recipes(&mut self) {
        let sum: i32 =
            self.elves
            .iter()
            .map(|curr| self.board[*curr as usize])
            .sum();
        
        if sum < 10 {
            self.board.push(sum);
        }
        else {
            self.board.push(sum / 10);
            self.board.push(sum % 10);
        }
    }

    fn set_currect_recipes(&mut self) {
        for curr in &mut self.elves {
            *curr = (self.board[*curr as usize] + *curr + 1) % self.board.len() as i32;
        }
    }
}

#[test]
fn test_recipe_score() {
    let mut kitchen = Kitchen::new(vec![3, 7], 2);
    assert_eq!(kitchen.score("5"), "0124515891");
    assert_eq!(kitchen.score("9"), "5158916779");
    assert_eq!(kitchen.score("18"), "9251071085");
    assert_eq!(kitchen.score("2018"), "5941429882");
}

#[test]
fn test_recipe_backwards_1() {
    let kitchen = Kitchen::new(vec![3, 7], 2);
    let recipes_amount = kitchen.appeared_first("51589");
    assert_eq!(recipes_amount, 9);
}

#[test]
fn test_recipe_backwards_2() {
    let kitchen = Kitchen::new(vec![3, 7], 2);
    let recipes_amount = kitchen.appeared_first("01245");
    assert_eq!(recipes_amount, 5);
}

#[test]
fn test_recipe_backwards_3() {
    let kitchen = Kitchen::new(vec![3, 7], 2);
    let recipes_amount = kitchen.appeared_first("92510");
    assert_eq!(recipes_amount, 18);
}

#[test]
fn test_recipe_backwards_4() {
    let kitchen = Kitchen::new(vec![3, 7], 2);
    let recipes_amount = kitchen.appeared_first("59414");
    assert_eq!(recipes_amount, 2018);
}
