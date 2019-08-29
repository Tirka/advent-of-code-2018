use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
pub enum TrackType {
    WestToEast,
    NorthToSouth,
    WestToSouth,
    EastToSouth,
    Crossroad,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    West, North, East, South
}

#[derive(Debug, Eq, PartialEq)]
pub enum Turn {
    Left, Straight, Right
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Eq, PartialEq)]
struct Vector {
    x: i32,
    y: i32,
}
impl Vector {
    fn from(direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
            Direction::North => (0, -1),
        };
        Vector { x, y }
    }

    // https://en.wikipedia.org/wiki/Rotation_matrix
    fn rotate(&mut self, turn: &Turn) {
        let (sin, cos) = match turn {
            Turn::Left => (1, 0),
            Turn::Right => (-1, 0),
            Turn::Straight => (0, 1),
        };
        
        let new_x =  self.x * cos + self.y * sin;
        let new_y = -self.x * sin + self.y * cos;
        
        self.x = new_x;
        self.y = new_y;
    }

    fn description(&self) -> Direction {
        match (&self.x, &self.y) {
            (1, 0) => Direction::East,
            (-1, 0) => Direction::West,
            (0, 1) => Direction::South,
            (0, -1) => Direction::North,
            _ => panic!("impossibru!")
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cart {
    position: Coordinate,
    direction: Vector,
    next_turn: Turn,
}
impl Cart {
    pub fn new(position: Coordinate, direction: Direction) -> Self {
        Self {
            position,
            direction: Vector::from(direction),
            next_turn: Turn::Left
        }
    }

    fn step(&mut self) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }

    fn turn(&mut self, current_track: &TrackType) {
        match current_track {
            TrackType::WestToSouth => {
                match self.direction.description() {
                    Direction::West | Direction::East =>
                        self.direction.rotate(&Turn::Right),
                    Direction::North | Direction::South =>
                        self.direction.rotate(&Turn::Left),
                }
            },
            TrackType::EastToSouth => {
                match self.direction.description() {
                    Direction::West | Direction::East =>
                        self.direction.rotate(&Turn::Left),
                    Direction::North | Direction::South =>
                        self.direction.rotate(&Turn::Right),
                }
            },
            TrackType::Crossroad => {
                self.direction.rotate(&self.next_turn);
                match &self.next_turn {
                    Turn::Left => self.next_turn = Turn::Straight,
                    Turn::Straight => self.next_turn = Turn::Right,
                    Turn::Right => self.next_turn = Turn::Left,
                }
            }
            TrackType::WestToEast | TrackType::NorthToSouth => ()
        }
    }
}

#[derive(Debug)]
pub struct Map {
    tracks: HashMap<Coordinate, TrackType>,
    carts: Vec<Cart>,
    collisions: HashSet<Coordinate>
}
impl Map {
    pub fn new(tracks: HashMap<Coordinate, TrackType>, carts: Vec<Cart>) -> Self {
        let collisions = carts.iter().map(|cart| cart.position).collect();
        Self { tracks, carts, collisions }
    }

    pub fn tick(&mut self) -> Option<Coordinate> {
        self.sort_carts();

        for cart in &mut self.carts {
            self.collisions.remove(&cart.position);
            cart.step();
            cart.turn(self.tracks.get(&cart.position).unwrap());
            if !self.collisions.insert(cart.position) { return Some(cart.position); }
        }

        None
    }

    pub fn get_last_cart(&mut self) -> Coordinate {
        unimplemented!()
    }

    fn sort_carts(&mut self) {
        self.carts.sort_by(|t1, t2| {
            t1.position.y.cmp(&t2.position.y)
                .then(t1.position.x.cmp(&t2.position.x))
        });
    }

    fn remove_collapsed_carts(&mut self) {
        // self.
    }
}

pub fn parse_input(raw: &str) -> (HashMap<Coordinate, TrackType>, Vec<Cart>) {
    let mut tracks = HashMap::new();
    let mut carts = Vec::new();
    for (y, line) in raw.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            let x = x as i32;
            let y = y as i32;
            match ch {
                '-' => { tracks.insert(Coordinate { x, y }, TrackType::WestToEast); },
                '|' => { tracks.insert(Coordinate { x, y }, TrackType::NorthToSouth); },
                '\\' => { tracks.insert(Coordinate { x, y }, TrackType::WestToSouth); },
                '/' => { tracks.insert(Coordinate { x, y }, TrackType::EastToSouth); },
                '+' => { tracks.insert(Coordinate { x, y }, TrackType::Crossroad); },
                'v' => {
                    tracks.insert(Coordinate { x, y }, TrackType::NorthToSouth);
                    carts.push(Cart::new(Coordinate { x, y }, Direction::South));
                },
                '^' => {
                    tracks.insert(Coordinate { x, y }, TrackType::NorthToSouth);
                    carts.push(Cart::new(Coordinate { x, y }, Direction::North));
                },
                '>' => {
                    tracks.insert(Coordinate { x, y }, TrackType::WestToEast);
                    carts.push(Cart::new(Coordinate { x, y }, Direction::East));
                },
                '<' => {
                    tracks.insert(Coordinate { x, y }, TrackType::WestToEast);
                    carts.push(Cart::new(Coordinate { x, y }, Direction::West));
                },
                _ => ()
            }
        }
    }
    (tracks, carts)
}

#[test]
fn test_parsing() {
    let test_data = r"-|/\+><^v";
    
    let (tracks, carts) = parse_input(test_data);

    let expected_tracks: HashMap<_,_> = vec![
        (Coordinate { x: 0, y: 0 }, TrackType::WestToEast),
        (Coordinate { x: 1, y: 0 }, TrackType::NorthToSouth),
        (Coordinate { x: 2, y: 0 }, TrackType::EastToSouth),
        (Coordinate { x: 3, y: 0 }, TrackType::WestToSouth),
        (Coordinate { x: 4, y: 0 }, TrackType::Crossroad),
        (Coordinate { x: 5, y: 0 }, TrackType::WestToEast),
        (Coordinate { x: 6, y: 0 }, TrackType::WestToEast),
        (Coordinate { x: 7, y: 0 }, TrackType::NorthToSouth),
        (Coordinate { x: 8, y: 0 }, TrackType::NorthToSouth),
    ].into_iter().collect();

    assert_eq!(tracks, expected_tracks);

    let expected_carts = vec![
        Cart {
            position: Coordinate { x: 5, y: 0 },
            direction: Vector::from(Direction::East),
            next_turn: Turn::Left,
        },
        Cart {
            position: Coordinate { x: 6, y: 0 },
            direction: Vector::from(Direction::West),
            next_turn: Turn::Left,
        },
        Cart {
            position: Coordinate { x: 7, y: 0 },
            direction: Vector::from(Direction::North),
            next_turn: Turn::Left,
        },
        Cart {
            position: Coordinate { x: 8, y: 0 },
            direction: Vector::from(Direction::South),
            next_turn: Turn::Left,
        }
    ];

    assert_eq!(carts, expected_carts);
}

#[test]
fn test_collision() {
    let test_data = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    let (tracks, carts) = parse_input(test_data);

    let mut map = Map::new(tracks, carts);
    loop {
        if let Some(coord) = map.tick() {
            assert_eq!(coord, Coordinate { x: 7, y: 3 });
            break;
        }
    }
}