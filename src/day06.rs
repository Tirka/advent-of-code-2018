use std::collections::HashMap;
use itertools::*;

#[derive(Debug)]
enum Area {
    Unknown,
    Infinine,
    Finite(u16)
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}
impl Point {
    pub fn parse_str(raw: &str) -> Self {
        let parts: Vec<_> = raw.split(',').collect();
        
        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].trim().parse().unwrap(),
        }
    }

    pub fn manhattan_distance(&self, point: &Point) -> i32 {
        (self.x - point.x).abs() +
        (self.y - point.y).abs()
    }
}

pub struct Cartesian {
    points: HashMap<Point, Area>,
    x_left: i32,
    x_right: i32,
    y_top: i32,
    y_bottom: i32
}
impl Cartesian {
    pub fn new() -> Self {
        Cartesian {
            points: HashMap::new(),
            x_left: std::i32::MAX,
            x_right: std::i32::MIN,
            y_top: std::i32::MAX,
            y_bottom: std::i32::MIN
        }
    }

    pub fn add_point(&mut self, point: Point) {
        if point.x < self.x_left { self.x_left = point.x - 1 }
        if point.x > self.x_right { self.x_right = point.x + 1 }
        if point.y < self.y_top { self.y_top = point.y - 1 }
        if point.y > self.y_bottom { self.y_bottom = point.y + 1 }
        self.points.insert(point, Area::Unknown);
    }

    pub fn find_largest_area(&mut self) -> u16 {
        self.mark_infinite_areas();
        self.calculate_finite_areas();
        self.get_largest_area()
    }

    pub fn find_size_of_sweet_region(&self, bound: i32) -> u32 {
        let mut size = 0;
        for x in self.x_left..=self.x_right {
            for y in self.y_top..=self.y_bottom {
                let p = Point { x, y };
                let mut total_distance = 0;
                for input_point in self.points.keys() {
                    total_distance += p.manhattan_distance(input_point);
                }
                if total_distance < bound {
                    size += 1;
                }
            }
        }
        size
    }

    fn mark_infinite_areas(&mut self) {
        let border_points = self.generate_border_points();
        
        for p in border_points {
            if let Some(closest) = self.find_the_closest_point(&p) {
                *self.points.get_mut(&closest).unwrap() = Area::Infinine;
            }
        }
    }

    // this function generates minimal size rectangle border around points at plot
    fn generate_border_points(&self) -> Vec<Point> {
        (self.x_left..self.x_right+1)
        .map(|n| Point{ x: n, y: self.y_top })
        .chain(
            (self.y_top+1..self.y_bottom+1)
            .map(|n| Point{ x: self.x_right, y: n })
        )
        .chain(
            (self.x_left..self.x_right)
            .rev()
            .map(|n| Point { x: n, y: self.y_bottom })
        )
        .chain((self.y_top+1..self.y_bottom)
            .rev()
            .map(|n| Point{ x: self.x_left, y: n })
        )
        .collect()
    }

    // this function gives closest point if only one such point exists
    fn find_the_closest_point(&self, point: &Point) -> Option<Point> {
        let distances: Vec<_> =
            self.points.keys()
            .map(|p| (p, p.manhattan_distance(&point)))
            .sorted_by_key(|t| t.1)
            .collect();

        match distances.len() {
            0 => panic!("add some points to plot first!"),
            1 => Some(*distances[0].0),
            _ => {
                let (p1, d1) = distances[0];
                let (__, d2) = distances[1];
                if d1 == d2 { return None; }
                else { return Some(*p1); }
            }
        }
    }

    fn calculate_finite_areas(&mut self) {
        for x in self.x_left..=self.x_right {
            for y in self.y_top..=self.y_bottom {
                let p = Point { x, y };
                if let Some(closest) = self.find_the_closest_point(&p) {
                    let area = self.points.get_mut(&closest).unwrap();
                    match area {
                        Area::Infinine => (),
                        Area::Unknown => *area = Area::Finite(1),
                        Area::Finite(x) => *area = Area::Finite(*x+1)
                    }
                }
            }
        }
    }

    fn get_largest_area(&self) -> u16 {
        self.points.values()
        .map(|v| match v {
            Area::Finite(x) => *x,
            _ => 0
        })
        .max()
        .unwrap()
    }
}

#[test]
fn test_example() {
    let test_data = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    let mut plot = Cartesian::new();
    
    for raw_point in test_data.lines() {
        plot.add_point(Point::parse_str(raw_point));
    }

    assert_eq!(plot.find_largest_area(), 17);
    assert_eq!(plot.find_size_of_sweet_region(32), 16);
}

#[test]
fn test_parsing() {
    let point = Point::parse_str("8, 3");

    assert_eq!(point, Point { x: 8, y: 3 });
}