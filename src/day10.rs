use regex::Regex;

#[derive(Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq)]
pub struct Star {
    position: Position,
    velocity: Velocity
}

struct MessageRect {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}
impl MessageRect {
    fn from_sky(sky: &Sky) -> Self {
        let mut x_min = std::i64::MAX;
        let mut x_max = std::i64::MIN;
        let mut y_min = std::i64::MAX;
        let mut y_max = std::i64::MIN;
        
        for star in &sky.stars {
            if x_min > star.position.x { x_min = star.position.x; }
            if y_min > star.position.y { y_min = star.position.y; }
            if x_max < star.position.x { x_max = star.position.x; }
            if y_max < star.position.y { y_max = star.position.y; }
        }

        MessageRect { x_min, x_max, y_min, y_max }
    }

    fn area(&self) -> i64 {
        (self.x_max - self.x_min) * (self.y_max - self.y_min)
    }
}

pub fn parse_points(raw: &str) -> Vec<Star> {
    let pattern = r"position=< ?(-?\d*), +(-?\d*)> velocity=< ?(-?\d*), +(-?\d*)>";
    let regex = Regex::new(pattern).unwrap();
    regex
        .captures_iter(raw)
        .map(|mtch|
            Star {
                position: Position {
                    x: mtch[1].parse().unwrap(),
                    y: mtch[2].parse().unwrap()
                },
                velocity: Velocity {
                    x: mtch[3].parse().unwrap(),
                    y: mtch[4].parse().unwrap(),
                }
            }
        )
        .collect()
}

#[derive(Debug)]
pub struct Sky {
    stars: Vec<Star>,
    timestamp: i64,
}
impl Sky {
    pub fn new(stars: Vec<Star>) -> Self {
        Sky { stars, timestamp: 0 }
    }

    pub fn await_some_time(&mut self) {
        let mut possible_msg_area = std::i64::MAX;
        while possible_msg_area > self.message_rect().area() {
            possible_msg_area = self.message_rect().area();
            self.second_forward();
        }
        self.second_backward();
    }

    pub fn read_message(&self) -> String {
        let mut message = String::new();
        
        let msg_rect = &self.message_rect();

        for y in msg_rect.y_min..=msg_rect.y_max {
            for x in msg_rect.x_min..=msg_rect.x_max {
                let pos = Position { x, y };
                if self.stars.iter().any(|star| star.position == pos) {
                    message.push('#')
                } else {
                    message.push('.')
                }
            }
            message.push('\n')
        }
        message
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn message_rect(&self) -> MessageRect {
        MessageRect::from_sky(&self)
    }

    fn second_forward(&mut self) {
        self.move_in_time(1)
    }

    fn second_backward(&mut self) {
        self.move_in_time(-1)
    }

    fn move_in_time(&mut self, seconds: i64) {
        for i in 0..self.stars.len() {
            let star = self.stars.get_mut(i).unwrap();
            star.position.x += star.velocity.x * seconds;
            star.position.y += star.velocity.y * seconds;
        }
        self.timestamp += seconds;
    }
}

#[test]
fn test_parsing() {
    let test_input = r"position=< 52534, -31215> velocity=<-5,  3>
position=< 10658, -31220> velocity=<-1,  3>";

    let points = parse_points(test_input);

    assert_eq!(points, vec![
        Star {
            position: Position {
                x: 52534,
                y: -31215,
            },
            velocity: Velocity {
                x: -5,
                y: 3,
            },
        },
        Star {
            position: Position {
                x: 10658,
                y: -31220,
            },
            velocity: Velocity {
                x: -1,
                y: 3,
            }
        }
    ])
}