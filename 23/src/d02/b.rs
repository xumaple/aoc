use std::collections::HashMap;
use util::*;

pub fn run(filename: &str) -> Result<i32, BoxError> {
    let mut sum = 0;
    for l in read_lines(filename)? {
        let l = l?;
        let (_game_num_str, hands) = l.split_once(": ").expect("Did not find ':' in game line");
        // let game_num: i32 = unsafe { _game_num_str.get_unchecked(5..) }.parse()?;

        let mut maxes: HashMap<&str, i32> = HashMap::new();

        let mut on_num = true;
        let mut prev_num: i32 = -1;
        for token in hands.split(" ") {
            match on_num {
                true => {
                    prev_num = token
                        .parse::<i32>()
                        .expect(format!("Unable to parse int: {token}").as_str());
                    on_num = false;
                }
                false => {
                    let curr_val = maxes
                        .entry(&token.trim_end_matches(&[',', ';']))
                        .or_insert(0);
                    if prev_num > *curr_val {
                        *curr_val = prev_num;
                    }

                    on_num = true;
                }
            }
        }
        sum += maxes.values().fold(1, |a, &b| a * b);
    }
    Ok(sum)
}

#[allow(dead_code)]
fn main() -> NulBoxError {
    println!("{}", run("src/d02/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod test_02b {
    use super::run;

    #[test]
    fn official() {
        assert_eq!(run("src/d02/input.txt").unwrap(), 54911)
    }
}
