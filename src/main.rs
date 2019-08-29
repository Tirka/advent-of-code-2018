#![allow(dead_code)]
#![allow(unused_macros)]
// #![allow(unused_variables)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

// mod freestyle;

macro_rules! read_input {
    ($path:expr) => {{
        let base_path =std::path::Path::new(r"D:\Git\advent2018\puzzle_inputs");
        std::fs::read_to_string(base_path.join($path)).unwrap()
    }};
}

fn main() {
    // // DAY 1
    // let freq = day01::calculate_freq(&read_input!("day01.txt"));
    // println!("DAY 01. PART 1: {}", freq);
    // let dupl = day01::find_first_duplicate(&read_input!("day01.txt"));
    // println!("DAY 01. PART 2: {}\n", dupl);


    // // DAY 2
    // let checksum = day02::calculate_checksum(&read_input!("day02.txt"));
    // println!("DAY 02. PART 1: {}", checksum);
    // let correct_box = day02::find_correct_box(&read_input!("day02.txt"));
    // println!("DAY 02. PART 2: {}\n", correct_box);


    // // DAY 3
    // let mut fabric = day03::Fabric::new();
    // for claim in read_input!("day03.txt").lines().map(day03::Claim::parse_str) {
    //     fabric.make_claim(claim);
    // }
    // let overlapped_amount = fabric.count_overlapped();
    // println!("DAY 03. PART 1: {}", overlapped_amount);
    // let the_chosen_one_id = fabric.find_magic_id();
    // println!("DAY 03. PART 2: {}\n", the_chosen_one_id);


    // // DAY 4
    // let observations = day04::parse_input(&read_input!("day04.txt"));
    // let obs_map = day04::ObservationMap::build_from_observations(observations);
    // println!("DAY 04. PART 1: {}", obs_map.strategy_1());
    // println!("DAY 04. PART 2: {}\n", obs_map.strategy_2());

    // // DAY 5
    // let reduced_length = day05::reduce_and_measure(&read_input!("day05.txt"));
    // println!("DAY 05. PART 1: {}", reduced_length);
    // let min_length = day05::cut_reduce_and_measure(&read_input!("day05.txt"));
    // println!("DAY 05. PART 2: {}\n", min_length);


    // // DAY 6
    // let mut plot = day06::Cartesian::new();
    // for point in read_input!("day06.txt").lines().map(day06::Point::parse_str) {
    //     plot.add_point(point);
    // }
    // let area = plot.find_largest_area();
    // println!("DAY 06. PART 1: {}", area);
    // let region_size = plot.find_size_of_sweet_region(10000);
    // println!("DAY 06. PART 2: {}\n", region_size);


    // // DAY 7
    // let mut instructions = day07::Instructions::new();
    // for rule in read_input!("day07.txt").lines().map(day07::Rule::parse_raw) {
    //     instructions.add_rule(rule);
    // }
    // let order: String = instructions.collect();
    // println!("DAY 07. PART 1: {}", order);


    // // DAY 8
    // let input = day08::parse_string(&read_input!("day08.txt"));
    // let tree = day08::Tree::new(input);
    // println!("DAY 08. PART 1: {}", tree.first_check());
    // println!("DAY 08. PART 2: {}\n", tree.second_check());


    // // DAY 9
    // let (total_players, last_marble) = day09::parse_input(&read_input!("day09.txt"));
    // let game = day09::Game::new(total_players, last_marble);
    // println!("DAY 09. PART 1: {}", game.calculate_highscore());
    // let game2 = day09::Game::new(total_players, last_marble * 100);
    // println!("DAY 09. PART 2: {}\n", game2.calculate_highscore());


    // // DAY 10
    // let mut sky = day10::Sky::new(day10::parse_points(&read_input!("day10.txt")));
    // sky.await_some_time();
    // let message = sky.read_message();
    // println!("DAY 10. PART 1:\n{}", message);
    // let timestamp = sky.get_timestamp();
    // println!("DAY 10. PART 2: {}\n", timestamp);


    // // DAY 11
    // let serial: i32 = read_input!("day11.txt").parse().unwrap();
    // let grid = day11::PowerGrid::new(serial);
    // let measure = grid.find_max_power(3);
    // println!("DAY 11. PART 1: {},{}", measure.x, measure.y);
    // let abs = grid.find_max_power_absolute();
    // println!("DAY 11. PART 2: {},{},{}\n", abs.x, abs.y, abs.size);


    // // DAY 12


    // // DAY 13
    // let (tracks, carts) = day13::parse_input(&read_input!("day13.txt"));
    // let mut map = day13::Map::new(tracks, carts);
    // loop {
    //     if let Some(coord) = map.tick() {
    //         println!("DAY 13. PART 1: {},{}", coord.x, coord.y);
    //         break;
    //     }
    // }


    // // DAY 14
    // let mut kitchen = day14::Kitchen::new(vec![3, 7], 2);
    // let score = kitchen.score(&read_input!("day14.txt"));
    // println!("DAY 14. PART 1: {}", score);
    // let kitchen2 = day14::Kitchen::new(vec![3, 7], 2);
    // let appeared = kitchen2.appeared_first(&read_input!("day14.txt"));
    // println!("DAY 14. PART 2: {}\n", appeared);


    // // DAY 16
    // let (samples, _test_sequence) = day16::parse_input(&read_input!("day16.txt"));
    // let amount = day16::test_samples(samples);
    // println!("DAY 16. PART 1: {}", amount);
}
