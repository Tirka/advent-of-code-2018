use std::collections::{HashMap, VecDeque};

pub fn parse_input(raw: &str) -> (usize, usize) {
    let parts: Vec<_> = raw.split(' ').collect();
    let total_players = parts[0].parse().unwrap();
    let last_marble = parts[6].parse().unwrap();
    (total_players, last_marble)
}

#[derive(Debug)]
pub struct Game {
    total_players: usize,
    curr_player: usize,
    last_marble: usize,
    board: VecDeque<usize>,
    scores: HashMap<usize, usize>
}
impl Game {
    pub fn new(total_players: usize, last_marble: usize) -> Self {
        Self {
            total_players,
            last_marble,
            scores: HashMap::new(),
            board: VecDeque::from(vec![0, 1]),
            curr_player: 2
        }
    }

    pub fn calculate_highscore(mut self) -> usize {
        for marble in 2..=self.last_marble {
            &self.place_marble(marble);
        }
        self.get_highscore()
    }

    fn place_marble(&mut self, marble: usize) {
        if marble % 23 != 0 {
            self.board.rotate_left(2);
            self.board.push_front(marble);
        } else {
            self.add_to_score(self.curr_player, marble);
            self.board.rotate_right(7);
            let addition_score = self.board.pop_front().unwrap();
            self.add_to_score(self.curr_player, addition_score);
        }
        self.curr_player = self.curr_player % self.total_players + 1;
    }

    fn add_to_score(&mut self, player: usize, score: usize) {
        match self.scores.get_mut(&player) {
            None => { self.scores.insert(player, score); },
            Some(s) => *s += score
        }
    }

    fn get_highscore(&self) -> usize {
        *self.scores.values().max().unwrap()
    }
}

#[test]
fn test_marble_game_1() {
    let game = Game::new(10, 1618);
    let highscore = game.calculate_highscore();
    assert_eq!(highscore, 8317);
}


#[test]
fn test_marble_game_2() {
    let game = Game::new(13, 7999);
    let highscore = game.calculate_highscore();
    assert_eq!(highscore, 146373);
}

#[test]
fn test_marble_game_3() {
    let game = Game::new(17, 1104);
    let highscore = game.calculate_highscore();
    assert_eq!(highscore, 2764);
}

#[test]
fn test_marble_game_4() {
    let game = Game::new(21, 6111);
    let highscore = game.calculate_highscore();
    assert_eq!(highscore, 54718);
}

#[test]
fn test_marble_game_5() {
    let game = Game::new(30, 5807);
    let highscore = game.calculate_highscore();
    assert_eq!(highscore, 37305);
}

#[test]
fn test_parsing() {
    let (total_players, last_marble) =
        parse_input("470 players; last marble is worth 72170 points");

    assert_eq!(total_players, 470);
    assert_eq!(last_marble, 72170);
}
