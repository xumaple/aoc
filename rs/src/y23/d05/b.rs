#![allow(unused_attributes)]
#![feature(iter_array_chunks)]

use util::*;
#[path = "./shared.rs"]
mod shared;
use shared::*;

#[aoc_proc::aoc_run(23-05b)]
pub fn run(input: impl AsRef<str>) -> Result<i64, BoxError> {
    let mut l_iter = input.as_ref().lines();
    let seeds: Vec<Range<i64>> = l_iter.next().unwrap()[7..]
        .split_whitespace()
        .map(i64::ufrom)
        .array_chunks::<2>()
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect_vec();

    let mappings: Vec<Mapping> = l_iter.fold(Vec::new(), |mut mappings, s| {
        if s.len() == 0 {
            mappings.push(Mapping::new());
            return mappings;
        }
        if !s.chars().next().unwrap().is_digit(10) {
            return mappings;
        }
        mappings
            .last_mut()
            .unwrap()
            .add_map(s.split_whitespace().map(i64::ufrom).collect_vec());
        mappings
    });

    let smallest = seeds
        .iter()
        .map(|seed| {
            mappings
                .iter()
                .fold(vec![seed.clone(); 1], |seed_ranges, map| {
                    map.get_map_range(seed_ranges)
                })
                .iter()
                .map(|r| r.start)
                .min()
                .unwrap()
                .clone()
        })
        .min()
        .unwrap();
    Ok(smallest)
}
