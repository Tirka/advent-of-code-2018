use std::collections::HashSet;

const FABRIC_SIZE: u16 = 1000;

#[derive(Debug, PartialEq)]
enum State {
    Available,
    Reserved(u16),
    Overlapped(HashSet<u16>)
}
impl State {
    fn i_need_it(&mut self, id: u16) {
        match self {
            State::Available => *self = State::Reserved(id),
            State::Reserved(by_id) => {
                let mut hs = HashSet::new();
                hs.insert(*by_id);
                hs.insert(id);
                *self = State::Overlapped(hs)
            },
            State::Overlapped(hs) => { hs.insert(id); }
        }
    }

    fn is_overlapped(&self) -> bool {
        match &self {
            State::Overlapped(_) => true,
            _ => false
        }
    }
}

pub struct Fabric {
    pieces: Vec<Vec<State>>,
    active_ids: HashSet<u16>
}
impl Fabric {
    pub fn new() -> Self {
        let active_ids = HashSet::new();
        let pieces: Vec<Vec<_>> = 
            (0..1000)
            .map(|_| (0..1000).map(|_| State::Available).collect())
            .collect();
        Fabric { pieces, active_ids }
    }

    pub fn make_claim(&mut self, claim: Claim) {
        self.active_ids.insert(claim.id);

        for x in 0..claim.size.x {
            for y in 0..claim.size.y {
                self.pieces[x+claim.offset.x][y+claim.offset.y].i_need_it(claim.id);
            }
        }
    }

    pub fn count_overlapped(&self) -> usize {
        self.pieces
        .iter()
        .flatten()
        .filter(|&state| state.is_overlapped())
        .count()
    }

    pub fn find_magic_id(&self) -> u16 {
        let mut overlapped_ids = HashSet::new();

        for cell in self.pieces.iter().flatten() {
            match cell {
                State::Overlapped(ids) => {
                    for id in ids {
                        overlapped_ids.insert(*id);
                    }
                },
                _ => ()
            }
        }

        *self.active_ids.difference(&overlapped_ids).next().unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    x: usize,
    y: usize
}

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: u16,
    offset: Rectangle,
    size: Rectangle
}
impl Claim {
    // maybe std::str::FromStr trait with proper error handling
    pub fn parse_str(raw: &str) -> Self {
        let parts: Vec<_> = raw.split(|c|
            c == '#' ||
            c == '@' ||
            c == ',' ||
            c == ':' ||
            c == 'x'
        ).collect();

        Claim {
            id: parts[1].trim().parse().unwrap(),
            offset: Rectangle {
                x: parts[2].trim().parse().unwrap(),
                y: parts[3].trim().parse().unwrap()
            },
            size: Rectangle {
                x: parts[4].trim().parse().unwrap(),
                y: parts[5].trim().parse().unwrap()
            }
        }
    }
}

#[test]
fn test_example() {
    let mut fabric = Fabric::new();

    fabric.make_claim(Claim::parse_str("#1 @ 1,3: 4x4"));
    fabric.make_claim(Claim::parse_str("#2 @ 3,1: 4x4"));
    fabric.make_claim(Claim::parse_str("#3 @ 5,5: 2x2"));

    assert_eq!(fabric.count_overlapped(), 4);
    assert_eq!(fabric.find_magic_id(), 3);
}

#[test]
fn test_parsing() {
    let claim = Claim::parse_str("#1 @ 22,33: 444x555");
    let expected = Claim {
        id: 1,
        offset: Rectangle { x: 22, y: 33 },
        size: Rectangle { x: 444, y: 555 }
    };

    assert_eq!(claim, expected)
}
