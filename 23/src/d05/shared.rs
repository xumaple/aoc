pub use std::collections::HashMap;
pub use std::ops::Range;

// Range : mapping difference
// Eg. 50 98 2 maps to: [(50, 52), 48]
pub struct Mapping(HashMap<Range<i64>, i64>);

impl Mapping {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_map(&mut self, mapping: Vec<i64>) {
        assert!(mapping.len() == 3);
        self.0
            .insert(mapping[1]..mapping[1] + mapping[2], mapping[0] - mapping[1]);
    }

    pub fn get_map(&self, input: i64) -> i64 {
        for (range, diff) in self.0.iter() {
            if range.contains(&input) {
                return input + diff;
            }
        }
        return input;
    }

    pub fn get_map_range(&self, ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
        ranges.iter().fold(Vec::new(), |mut new_ranges, seed_r| {
            let mut temp_r = vec![seed_r.clone()];
            // let mut i = 0;
            while let Some(seed_r) = temp_r.pop() {
                assert_range(seed_r.clone());
                let mut touched = false;
                for (map_r, diff) in self.0.iter() {
                    let mut new_push = |r: Range<i64>| {
                        new_ranges.push(r.start + diff..r.end + diff);
                    };
                    let mss = map_r.start <= seed_r.start && map_r.end > seed_r.start;
                    let mse = map_r.start <= seed_r.end - 1 && map_r.end > seed_r.end - 1;
                    let sms = seed_r.start <= map_r.start && seed_r.end > map_r.start;
                    let sme = seed_r.start <= map_r.end - 1 && seed_r.end > map_r.end - 1;
                    if mss && mse {
                        // Entire seed_r was captured, can break
                        new_push(seed_r.start..seed_r.end);
                        touched = true;
                        break;
                    } else if sms && sme {
                        new_push(map_r.start..map_r.end);
                        if seed_r.start < map_r.start {
                            temp_r.push(seed_r.start..map_r.start);
                        }
                        if map_r.end < seed_r.end {
                            temp_r.push(map_r.end..seed_r.end);
                        }
                        touched = true;
                        break;
                    } else if mss {
                        new_push(seed_r.start..map_r.end);
                        temp_r.push(map_r.end..seed_r.end);
                        touched = true;
                        break;
                    } else if mse {
                        new_push(map_r.start..seed_r.end);
                        temp_r.push(seed_r.start..map_r.start);
                        touched = true;
                        break;
                    }
                }
                if !touched {
                    new_ranges.push(seed_r);
                }
            }
            new_ranges
        })
    }
}

fn assert_range(r: Range<i64>) {
    if r.end < r.start {
        panic!("Found invalid range {:?}", r);
    }
}
