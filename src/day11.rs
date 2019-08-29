const GRID_SIZE: i32 = 300;

macro_rules! q {
    ($vec:expr, $x:expr, $y:expr) => {
        unsafe {
            *$vec.get_unchecked(($y as usize -1) * GRID_SIZE as usize + ($x as usize -1))
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct PowerMeasurement {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub power: i32,
}
impl Default for PowerMeasurement {
    fn default() -> Self {
        Self { x: 0, y: 0, size: 0, power: std::i32::MIN }
    }
}

pub struct PowerGrid {
    grid: Vec<i32>
}
impl PowerGrid {
    pub fn new(serial_number: i32) -> Self {
        let mut grid = Vec::new();

        for y in 1..=GRID_SIZE {
            for x in 1..=GRID_SIZE {
                grid.push(Self::power_level(x as i32, y as i32, serial_number))
            }
        }

        Self { grid }
    }

    pub fn find_max_power(&self, size: i32) -> PowerMeasurement {
        let mut power_result = std::i32::MIN;
        let mut x_result = 1;
        let mut y_result = 1;
        let bound = GRID_SIZE - size + 1;

        for y in 1..=bound {
            let mut p = self.total_power(size, 1, y);
            if power_result < p {
                power_result = p;
                x_result = 1;
                y_result = y;
            }
            for x in 2..=bound {
                for i in 0..size {
                    p -= q!(self.grid, x-1, y+i);
                    p += q!(self.grid, x-1+size, y+i);
                }
                if power_result < p {
                    power_result = p;
                    x_result = x;
                    y_result = y;
                }
            }
        }

        PowerMeasurement {
            x: x_result,
            y: y_result,
            size,
            power: power_result
        }
    }

    pub fn find_max_power_absolute(&self) -> PowerMeasurement {
        let mut result = PowerMeasurement::default();

        for s in 1..=GRID_SIZE {
            let measure = self.find_max_power(s);
            if measure.power > result.power {
                result = measure;
            }
        }

        result
    }

    fn total_power(&self, size: i32, x_left: i32, y_top: i32) -> i32 {
        let mut power = 0;
        for y in y_top..y_top+size {
            for x in x_left..x_left+size {
                power += q!(&self.grid, x, y);
            }
        }
        power
    }

    fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
        let rack_id = Self::rack_id(x);
        Self::hundreds_extractor((rack_id * y + serial_number) * rack_id) - 5
    }

    fn rack_id(x: i32) -> i32 {
        x + 10
    }

    fn hundreds_extractor(power_level: i32) -> i32 {
        power_level / 100 % 10
    }
}

#[test]
fn test_hundreds_extractor() {
    assert_eq!(PowerGrid::hundreds_extractor(12345), 3);
    assert_eq!(PowerGrid::hundreds_extractor(33), 0);
}

#[test]
fn test_power_level() {
    assert_eq!(PowerGrid::power_level(122, 79, 57), -5);
    assert_eq!(PowerGrid::power_level(217, 196, 39), 0);
    assert_eq!(PowerGrid::power_level(101, 153, 71), 4);
}

#[test]
fn test_grid_fixed() {
    let grid = PowerGrid::new(18);
    let measure = grid.find_max_power(3);
    let expected = PowerMeasurement {
        x: 33,
        y: 45,
        size: 3,
        power: 29
    };

    assert_eq!(measure, expected)
}

#[test]
fn test_grid_absolute_1() {
    let grid = PowerGrid::new(18);
    let measure = grid.find_max_power_absolute();
    let expected = PowerMeasurement {
        x: 90,
        y: 269,
        size: 16,
        power: 113
    };

    assert_eq!(measure, expected)
}

#[test]
fn test_grid_absolute_2() {
    let grid = PowerGrid::new(42);
    let measure = grid.find_max_power_absolute();
    let expected = PowerMeasurement {
        x: 232,
        y: 251,
        size: 12,
        power: 119
    };

    assert_eq!(measure, expected)
}
