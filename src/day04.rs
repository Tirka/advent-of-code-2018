use std::collections::HashMap;
use itertools::Itertools;

pub fn parse_input(raw: &str) -> Vec<Observation> {
    raw
    .lines()
    .map(|r| {
        let record = match r.chars().nth(19).unwrap() {
            'f' => RecordType::FallsAsleep,
            'w' => RecordType::WakesUp,
            'G' => {
                let guard_id = r[26..].split(' ').nth(0).unwrap().parse().unwrap();
                RecordType::BeginsShift(guard_id)
            },
            _ => panic!("impossibru")
        };
        Observation {
            year: r[1..5].parse().unwrap(),
            month: r[6..8].parse().unwrap(),
            day: r[9..11].parse().unwrap(),
            hour: r[12..14].parse().unwrap(),
            minute: r[15..17].parse().unwrap(),
            record
        }
    })
    .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum RecordType {
    WakesUp,
    FallsAsleep,
    BeginsShift(usize)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Observation {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    record: RecordType
}

#[derive(Debug)]
pub struct ObservationMap {
    results: HashMap<usize, HashMap<usize, usize>>
}
impl ObservationMap {
    pub fn build_from_observations(mut observations: Vec<Observation>) -> Self {
        observations.sort_by(|o1, o2| {
            o1.year.cmp(&o2.year)
            .then(o1.month.cmp(&o2.month))
            .then(o1.day.cmp(&o2.day))
            .then(o1.hour.cmp(&o2.hour))
            .then(o1.minute.cmp(&o2.minute))
        });

        let mut guard_id = 0;
        let mut asleep_at = 0;

        let mut results = HashMap::new();

        for obs in observations.iter() {
            match obs.record {
                RecordType::BeginsShift(id) => guard_id = id,
                RecordType::FallsAsleep => asleep_at = obs.minute,
                RecordType::WakesUp => {
                    match results.get_mut(&guard_id) {
                        Some(map) => {
                            Self::put_inner(map, asleep_at, obs.minute);
                        },
                        None => {
                            let mut map = HashMap::new();
                            Self::put_inner(&mut map, asleep_at, obs.minute);
                            results.insert(guard_id, map);
                        }
                    }
                }
            }
        }

        Self { results }
    }

    pub fn strategy_1(&self) -> usize {
        let selected_guard =
            self.results
            .iter()
            .map(|(guard, stats)| {
                (guard, stats.iter().fold(0, |acc, (_key, val)| acc + val))
            })
            .sorted_by_key(|x| std::usize::MAX - x.1)
            .map(|(guard, _stats)| *guard)
            .nth(0)
            .unwrap();

        let selected_min =
            self.results.get(&selected_guard).unwrap()
            .iter()
            .sorted_by_key(|(_minute, occurrences)| std::usize::MAX - *occurrences)
            .map(|(minute, _occurrences)| *minute)
            .nth(0)
            .unwrap();

        selected_guard * selected_min
    }

    pub fn strategy_2(&self) -> usize {
        let mut selected_guard = 0;
        let mut selected_min = 0;
        let mut max_occurences = 0;

        for (guard, statistics) in self.results.iter() {
            for (minute, occurrences) in statistics.iter() {
                if *occurrences > max_occurences {
                    max_occurences = *occurrences;
                    selected_guard = *guard;
                    selected_min = *minute;
                }
            }
        }
        
        selected_guard * selected_min
    }

    fn put_inner(map: &mut HashMap<usize, usize>, asleep_at: usize, woke_at: usize) {
        for min in asleep_at..woke_at {
            match map.get_mut(&min) {
                Some(amount) => *amount += 1,
                None => { map.insert(min, 1); }
            }
        }
    }
}

#[test]
fn test_parsing() {
    let test_data = r"[1518-11-01 01:02] Guard #10 begins shift
[1520-09-03 05:06] wakes up
[1519-10-02 03:04] falls asleep";

    let parsed = parse_input(test_data);

    let expected = vec![
        Observation {
            year: 1518,
            month: 11,
            day: 1,
            hour: 1,
            minute: 2,
            record: RecordType::BeginsShift(10)
        },
        Observation {
            year: 1520,
            month: 9,
            day: 3,
            hour: 5,
            minute: 6,
            record: RecordType::WakesUp
        },
        Observation {
            year: 1519,
            month: 10,
            day: 2,
            hour: 3,
            minute: 4,
            record: RecordType::FallsAsleep
        },
    ];

    assert_eq!(parsed, expected)
}

#[test]
fn test_strategies() {
    let test_data = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    let parsed = parse_input(test_data);

    let obs_result = ObservationMap::build_from_observations(parsed);

    assert_eq!(obs_result.strategy_1(), 240);
    assert_eq!(obs_result.strategy_2(), 4455);
}