use std::fmt;
use std::collections::{HashMap, LinkedList, HashSet};
use std::cell::RefCell;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coord {
    x: i32, // unpub
    y: i32  // unpub
}
impl Coord {
    fn mahhattan(c1: Coord, c2: Coord) -> i32 {
        (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
    }
}

#[derive(Debug, PartialEq)]
pub enum Terrain { Wall, Cavern }

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Terrain>>,
    adjacency: HashMap<Coord, LinkedList<Coord>>
}

impl Map {
    fn from_tiles(tiles: Vec<Vec<Terrain>>) -> Self {
        let adjacency = build_adjacency(&tiles);

        return Self { tiles, adjacency };

        fn build_adjacency(tiles: &Vec<Vec<Terrain>>)
            -> HashMap<Coord, LinkedList<Coord>> {

            let mut result = HashMap::new();

            for (y, row) in tiles.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    if *tile == Terrain::Cavern {
                        let mut neibhours = LinkedList::new();
                        for (x, y) in [(x, y-1),(x-1, y), (x+1, y), (x, y+1)].iter() {
                            if tiles[*y][*x] == Terrain::Cavern {
                                let x = *x as i32;
                                let y = *y as i32;
                                neibhours.push_back(Coord {x, y});
                            }
                        }

                        result.insert(Coord {x: x as i32, y: y as i32}, neibhours);
                    }
                }
            }

            // println!("ADJACENCY: {:?}", &result);

            result
        }
    }

    fn bfs_pathfinder(&self, moving: &Unit, surrounding: &Vec<Unit>) -> Coord {
        let mut queue = LinkedList::new();
        let mut parent = HashMap::new();
        let mut visited = HashSet::new();

        queue.push_back(moving.coordinate);

        let enemies: Vec<_> =
            surrounding.iter().filter(|u| u.creature != moving.creature).collect();

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            // println!("CURR: {:?}", curr);
            if enemies.iter().any(|u| curr == u.coordinate) {
                return backtrace(moving.coordinate, curr, parent)[1];
            }

            for adj in &self.adjacency[&curr] {
                // if !queue.contains(adj) {
                //     queue.push_back(*adj);
                //     parent.insert(*adj, curr);
                // }
                if !visited.contains(adj) && !queue.contains(adj) {
                    queue.push_back(*adj);
                    visited.insert(*adj);
                    parent.insert(*adj, curr);
                }
            }
        }

        return Coord { x: -1, y: -1 };

        fn backtrace(start: Coord, end: Coord, parent: HashMap<Coord, Coord>)
            -> Vec<Coord> {

            // println!("PARENT = {:?}", &parent);

            let mut result = vec![end];
            while *result.last().unwrap() != start {
                result.push(parent[result.last().unwrap()]);
            }
            result.reverse();
            result
        }
    }

}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Creature { Elf, Goblin }

#[derive(Debug, Copy, Clone)]
pub struct Unit {
    coordinate: Coord,
    attack_power: i32,
    health_points: i32,
    creature: Creature
}

impl Unit {
    pub fn new(coordinate: Coord, creature: Creature) -> Self {
        Self {
            attack_power: 3,
            health_points: 200,
            creature,
            coordinate
        }
    }

    fn set_position(mut self, position: Coord) {
        self.coordinate = position;
    }
}

#[derive(Debug)]
struct Units {
    // units: RefCell<Vec<Unit>>,
    units: Vec<Unit>,
}

impl Units {
    fn new(units: Vec<Unit>) -> Self {
        // Self { units: RefCell::new(units) }
        Self { units }
    }

    // fn reorder(&self) {
    //     self.units.borrow_mut().sort_by(|u1, u2| {
    //         u1.coordinate.y.cmp(&u2.coordinate.y)
    //         .then(u1.coordinate.x.cmp(&u2.coordinate.x))
    //     });
    // }

    fn reorder(&mut self) {
        self.units.sort_by(|u1, u2| {
            u1.coordinate.y.cmp(&u2.coordinate.y)
            .then(u1.coordinate.x.cmp(&u2.coordinate.x))
        });
    }

    fn len(&self) -> usize {
        self.units.len()
    }

    fn all(&self) -> &Vec<Unit> {
        &self.units
    }

    // fn find_unit(&self, x: usize, y: usize) -> Option<Unit> {
    //     let x = x as i32;
    //     let y = y as i32;

    //     self.units.borrow().iter().find(|u| u.coordinate == Coord { x, y}).map(|x| x.clone())
    // }

    fn find_unit(&self, x: usize, y: usize) -> Option<Unit> {
        let x = x as i32;
        let y = y as i32;

        self.units.iter().find(|u| u.coordinate == Coord { x, y}).map(|x| x.clone())
    }
}

pub struct Game {
    map: Map,
    units: Units
}
impl Game {
    pub fn new(tiles: Vec<Vec<Terrain>>, units: Vec<Unit>) -> Self {
        let map = Map::from_tiles(tiles);
        let units = Units::new(units);
        Self { map, units }
    }

    pub fn play_game(&mut self) {
        for _i in 0..1 {
            self.units.reorder();

            // for (i, mut unit) in self.units.units.iter_mut().enumerate() {
            //     let azazka = self.units.all_imm();
            //     let new_coord = self.map.bfs_pathfinder(unit, azazka);
            //     unit.coordinate = new_coord;
            //     // unit
            // }

            for j in 0..self.units.len() {
                let new_coord = self.map.bfs_pathfinder(&self.units.units[j], self.units.all());
                self.units.units[j].coordinate = new_coord;
            }
            // for unit in self.units.all_imm() {
            //     let azazka = self.units.all_imm().clone();
            //     let new_coord = self.map.bfs_pathfinder(&unit, azazka);
            //     println!("U1: {:?}; NEW: {:?}", &unit.coordinate, &new_coord);
            //     // (*unit).coordinate = new_coord;
            // }
        }

        // for unit in self.units.all_imm() {
        //     let new_coord = self.map.bfs_pathfinder(unit, &self.units.all_imm());
        //     println!("U1: {:?}; NEW: {:?}", &unit, &new_coord);
        //     unit.coordinate.x = new_coord.x;
        //     println!("U2: {:?}", &unit);
        // }
        // self.units.units.iter().map(|u| {
        //     let new_coord = self.map.bfs_pathfinder(&u, self.units.all_imm());
        //     // *u.coordinate = new_coord;
        // });
        // for unit in self.units.all_imm() {
        //     let new_coord = self.map.bfs_pathfinder(unit, self.units.all_imm());
        //     unit.set_position(new_coord);
        // }
    }
}
impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                match ch {
                    Terrain::Wall => result.push('#'),
                    Terrain::Cavern => {
                        match &self.units.find_unit(x, y) {
                            None => result.push('.'),
                            Some(unit) => match unit.creature {
                                Creature::Elf => result.push('E'),
                                Creature::Goblin => result.push('G')
                            }
                        }
                    },
                }
            }
            result.push('\n');
        }
        writeln!(f, "{}", &result)
    }
}

pub  fn parse_input(raw: &str) -> (Vec<Vec<Terrain>>, Vec<Unit>) {
    let mut map = vec![];
    let mut units = vec![];

    for (y, line) in raw.lines().enumerate() {
        let mut row = vec![];
        for (x, tile) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match tile {
                '#' => row.push(Terrain::Wall),
                '.' => row.push(Terrain::Cavern),
                'E' => {
                    row.push(Terrain::Cavern);
                    units.push(Unit::new(Coord { x, y }, Creature::Elf));
                },
                'G' => {
                    row.push(Terrain::Cavern);
                    units.push(Unit::new(Coord { x, y }, Creature::Goblin));
                }
                _ => panic!("impossibru!")
            }
        }
        map.push(row);
    }

    (map, units)
}

#[test]
fn babikas() {
    let test_map = r"#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########";

    let (tiles, units) = parse_input(test_map);

    let mut game = Game::new(tiles, units);

    println!("{:?}", &game);

    game.play_game();

    println!("{:?}", &game);
}
