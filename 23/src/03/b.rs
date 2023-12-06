use util::*;

fn adjust_gear(sorted_list: &mut Vec<Gear>, adjustment: i32, min_val: usize, max_val: usize) {
    for g in sorted_list.iter_mut() {
        let x = g.x_index;
        if x >= min_val {
            if x <= max_val {
                g.add_adjacent(adjustment);
            }
            if x > max_val {
                break;
            }
        }
    }
}

struct Gear {
    // struct for all *potential* gears
    pub x_index: usize,
    ratio: i32,
    adjacent: usize,
}

impl Gear {
    pub fn new(x_index: usize) -> Self {
        Self {
            x_index,
            ratio: 1,
            adjacent: 0,
        }
    }

    pub fn add_adjacent(&mut self, num: i32) {
        self.ratio *= num;
        self.adjacent += 1;
    }

    pub fn gear_ratio(&self) -> i32 {
        if self.adjacent == 2 {
            self.ratio
        } else {
            0
        }
    }
}

fn run(filename: &str) -> Result<i32, BoxError> {
    let mut symbol_locs: Vec<_> = read_lines(filename)?
        .into_iter()
        .map(|line| {
            Ok(line?
                .char_indices()
                .filter_map(|(i, c)| if c == '*' { Some(Gear::new(i)) } else { None })
                .collect::<Vec<Gear>>())
        })
        .collect::<Result<Vec<_>, BoxError>>()?;

    for (curr_y, line) in read_lines(filename)?.into_iter().enumerate() {
        let mut curr_x = 0;
        for num_str in line?.split(|c: char| !c.is_numeric()).collect::<Vec<_>>() {
            if num_str.len() > 0 {
                // Search for symbol locations that match
                let min_val = match curr_x {
                    0 => 0,
                    _ => curr_x - 1,
                };
                let max_val = curr_x + num_str.len();
                let num = num_str.parse::<i32>()?;
                adjust_gear(&mut symbol_locs[curr_y], num, min_val, max_val);
                if curr_y > 0 {
                    adjust_gear(&mut symbol_locs[curr_y - 1], num, min_val, max_val);
                }
                if curr_y < symbol_locs.len() - 1 {
                    adjust_gear(&mut symbol_locs[curr_y + 1], num, min_val, max_val);
                }
            }
            curr_x += num_str.len() + 1;
        }
    }
    // println!("{:?}", symbol_locs);
    let sum: i32 = symbol_locs
        .iter()
        .map(|gears| gears.iter().map(|gear| gear.gear_ratio()).sum::<i32>())
        .sum();
    Ok(sum)
}

fn main() -> NulBoxError {
    println!("{}", run("src/03/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_03b {
    use super::run;

    #[test]
    fn official() {
        assert_eq!(run("src/03/input.txt").unwrap(), 80253814)
    }
}
