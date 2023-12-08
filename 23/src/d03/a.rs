use aoc_proc::aoc_run;
use util::*;

fn exists(sorted_list: &Vec<usize>, min_val: usize, max_val: usize) -> bool {
    for &x in sorted_list {
        if x >= min_val {
            if x <= max_val {
                return true;
            }
            if x > max_val {
                break;
            }
        }
    }
    return false;
}

#[aoc_run(03a)]
pub fn run(input: impl AsRef<str>) -> Result<i32, BoxError> {
    let symbol_locs: Vec<_> = input
        .as_ref()
        .lines()
        .into_iter()
        .map(|line| {
            Ok(line
                .char_indices()
                .filter_map(|(i, c)| {
                    if c.is_digit(10) || c == '.' {
                        None
                    } else {
                        Some(i)
                    }
                })
                .collect::<Vec<usize>>())
        })
        .collect::<Result<Vec<_>, BoxError>>()?;
    let sum: i32 = input
        .as_ref()
        .lines()
        .into_iter()
        .enumerate()
        .map(|(curr_y, line)| {
            let mut curr_x = 0;
            let mut line_sum = 0;
            for num_str in line.split(|c: char| !c.is_numeric()).collect::<Vec<_>>() {
                if num_str.len() > 0 {
                    // Search for symbol locations that match
                    let min_val = match curr_x {
                        0 => 0,
                        _ => curr_x - 1,
                    };
                    let max_val = curr_x + num_str.len();
                    if exists(&symbol_locs[curr_y], min_val, max_val)
                        || (curr_y > 0 && exists(&symbol_locs[curr_y - 1], min_val, max_val))
                        || (curr_y < symbol_locs.len() - 1
                            && exists(&symbol_locs[curr_y + 1], min_val, max_val))
                    {
                        line_sum += num_str.parse::<i32>()?;
                    }
                }
                curr_x += num_str.len() + 1;
            }
            Ok(line_sum)
        })
        .collect::<Result<Vec<i32>, BoxError>>()?
        .iter()
        .sum();
    // println!("{:?}", symbol_locs);
    Ok(sum)
}
