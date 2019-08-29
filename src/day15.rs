use std::collections::HashMap;

type HealthPoints = u32;

#[derive(Debug)]
struct Unit {
    hitpoints: HealthPoints,
    attack_power: u32,
    unit_type: UnitType
}
impl Unit {
    fn get_health(&self) -> UnitHealth {
        unimplemented!()
    }

    fn attack(&mut self) -> UnitHealth {
        unimplemented!()
    }
}

#[derive(Debug)]
enum UnitType { Goblin, Elf }

#[derive(Debug)]
enum UnitHealth { Dead, Alive(HealthPoints) }

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<Coordinate, Terrain>
}
impl Map {
    fn print_map() -> String {
        unimplemented!()
    }
}

#[derive(Debug)]
struct Units {
    elves: Vec<Unit>,
    goblins: Vec<Unit>
}
impl Units {
}

#[derive(Debug)]
struct Game {
    map: Map,
    units: Units,
    score: GameScore
}
impl Game {
    fn turn(&mut self) {
        unimplemented!()
    }

    fn get_score(&self) -> GameScore {
        unimplemented!()
    }
}

type ScorePoints = u32;

#[derive(Debug)]
enum GameScore {
    InProgress,
    Ended(ScorePoints)
}

#[derive(Debug)]
enum Terrain { Wall, Cavern }

fn parse_input(_raw: &str) {
    unimplemented!()
}
    

#[test]
fn test_parsing() {
    let _test_data = r"#####
#...#
#G.E#
#####";
    // let expected = Game::
    unimplemented!();
}    
